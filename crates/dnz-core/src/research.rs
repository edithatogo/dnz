//! Optional offline lexical/vector/hybrid research indexes.

use crate::evidence::SearchProvenance;
use crate::models::Record;
use crate::vector::VectorStore;
use std::collections::BTreeMap;

/// A scored research result with the mechanism that produced it.
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchHit {
    pub record_id: String,
    pub score: f32,
    pub provenance: SearchProvenance,
}

/// Small deterministic token-overlap index for offline lexical retrieval.
#[derive(Debug, Clone)]
pub struct LexicalIndex {
    name: String,
    documents: BTreeMap<String, String>,
}

impl LexicalIndex {
    /// Build an index from record IDs and searchable title/description text.
    pub fn from_records(name: impl Into<String>, records: &[Record]) -> Self {
        let documents = records
            .iter()
            .map(|record| {
                let mut text = record.title.clone();
                if let Some(description) = &record.description {
                    text.push(' ');
                    text.push_str(description);
                }
                (record.id.clone(), text)
            })
            .collect();
        Self {
            name: name.into(),
            documents,
        }
    }

    /// Return deterministic token-overlap scores, descending by score then ID.
    pub fn search(&self, query: &str, limit: usize) -> Vec<ResearchHit> {
        let query_terms = tokens(query);
        if query_terms.is_empty() || limit == 0 {
            return Vec::new();
        }
        let provenance = SearchProvenance::Lexical {
            index: self.name.clone(),
            analyzer: "unicode-alphanumeric-lowercase-v1".into(),
        };
        let mut hits: Vec<_> = self
            .documents
            .iter()
            .filter_map(|(record_id, text)| {
                let document_terms = tokens(text);
                let matched = query_terms
                    .iter()
                    .filter(|term| document_terms.iter().any(|candidate| candidate == *term))
                    .count();
                (matched > 0).then(|| ResearchHit {
                    record_id: record_id.clone(),
                    score: matched as f32 / query_terms.len() as f32,
                    provenance: provenance.clone(),
                })
            })
            .collect();
        sort_hits(&mut hits);
        hits.truncate(limit.min(hits.len()));
        hits
    }
}

/// Query an existing vector store while recording model and index provenance.
pub fn vector_search<S: VectorStore>(
    store: &S,
    query_vector: &[f32],
    limit: usize,
    model: impl Into<String>,
    index: impl Into<String>,
) -> anyhow::Result<Vec<ResearchHit>> {
    let model = model.into();
    let index = index.into();
    let provenance = SearchProvenance::LocalVector {
        model,
        embedding_dimension: query_vector.len(),
        index,
    };
    Ok(store
        .query_similarity(query_vector, limit)?
        .into_iter()
        .map(|(record_id, score)| ResearchHit {
            record_id,
            score,
            provenance: provenance.clone(),
        })
        .collect())
}

/// Combine lexical and vector rankings using equal-weight score averaging.
pub fn hybrid_search(
    lexical: &[ResearchHit],
    vector: &[ResearchHit],
    limit: usize,
) -> Vec<ResearchHit> {
    if limit == 0 {
        return Vec::new();
    }
    let mut scores: BTreeMap<String, (f32, usize)> = BTreeMap::new();
    for hit in lexical.iter().chain(vector) {
        let entry = scores.entry(hit.record_id.clone()).or_insert((0.0, 0));
        entry.0 += hit.score;
        entry.1 += 1;
    }
    let lexical_provenance = lexical.first().map(|hit| hit.provenance.clone());
    let vector_provenance = vector.first().map(|hit| hit.provenance.clone());
    let components: Vec<_> = lexical_provenance
        .into_iter()
        .chain(vector_provenance)
        .collect();
    let provenance = SearchProvenance::Hybrid { components };
    let mut hits: Vec<_> = scores
        .into_iter()
        .map(|(record_id, (score, count))| ResearchHit {
            record_id,
            score: score / count as f32,
            provenance: provenance.clone(),
        })
        .collect();
    sort_hits(&mut hits);
    hits.truncate(limit.min(hits.len()));
    hits
}

fn tokens(value: &str) -> Vec<String> {
    value
        .split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_lowercase())
        .collect()
}

fn sort_hits(hits: &mut [ResearchHit]) {
    hits.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| left.record_id.cmp(&right.record_id))
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::{MemoryVectorStore, VectorStore};

    fn records() -> Vec<Record> {
        vec![
            Record {
                id: "b".into(),
                title: "Kauri forest map".into(),
                description: Some("Historic survey".into()),
                ..Record::default()
            },
            Record {
                id: "a".into(),
                title: "Harbour photograph".into(),
                ..Record::default()
            },
        ]
    }

    #[test]
    fn lexical_search_is_deterministic_and_provenance_labelled() {
        let index = LexicalIndex::from_records("test-index", &records());
        let hits = index.search("historic kauri", 10);
        assert_eq!(hits[0].record_id, "b");
        assert!(matches!(
            hits[0].provenance,
            SearchProvenance::Lexical { .. }
        ));
    }

    #[test]
    fn vector_and_hybrid_search_preserve_mechanism_provenance() {
        let mut store = MemoryVectorStore::default();
        store.insert("a", &[1.0, 0.0]).unwrap();
        store.insert("b", &[0.0, 1.0]).unwrap();
        let vector = vector_search(&store, &[1.0, 0.0], 2, "model-v1", "memory-v1").unwrap();
        assert!(matches!(
            vector[0].provenance,
            SearchProvenance::LocalVector { .. }
        ));
        let lexical = LexicalIndex::from_records("test-index", &records()).search("harbour", 2);
        let hybrid = hybrid_search(&lexical, &vector, 2);
        assert!(matches!(
            hybrid[0].provenance,
            SearchProvenance::Hybrid { .. }
        ));
    }
}
