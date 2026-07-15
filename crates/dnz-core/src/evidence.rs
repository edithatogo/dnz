//! Deterministic evidence packs with explicit search provenance.

use crate::models::Record;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Identifies where a similarity result came from; API MLT is not local vector search.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SearchProvenance {
    /// DigitalNZ computed the related-record result server-side.
    DigitalNzMoreLikeThis { endpoint: String, record_id: String },
    /// A local index computed similarity from an explicitly named model.
    LocalVector {
        model: String,
        embedding_dimension: usize,
        index: String,
    },
    /// A local token-based index with an explicitly named analyzer.
    Lexical { index: String, analyzer: String },
    /// A result combines named provider and local components.
    Hybrid { components: Vec<SearchProvenance> },
}

/// One source-grounded item in an evidence pack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceItem {
    pub record_id: String,
    pub title: String,
    pub citation_key: String,
    pub source_url: Option<String>,
    pub rights: Option<String>,
    pub rights_url: Option<String>,
}

/// A deterministic, reviewable evidence bundle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidencePack {
    pub schema_version: u32,
    pub query: String,
    pub provenance: SearchProvenance,
    pub items: Vec<EvidenceItem>,
    pub limitations: Vec<String>,
}

/// Build an evidence pack without fetching sources or downloading models.
pub fn build_evidence_pack(
    query: impl Into<String>,
    records: &[Record],
    provenance: SearchProvenance,
) -> EvidencePack {
    let items = records
        .iter()
        .map(|record| EvidenceItem {
            record_id: record.id.clone(),
            title: record.title.clone(),
            citation_key: citation_key(&record.id),
            source_url: record
                .source_url
                .clone()
                .or_else(|| record.landing_url.clone()),
            rights: record.rights.clone().or_else(|| record.usage.clone()),
            rights_url: record.rights_url.clone(),
        })
        .collect();
    EvidencePack {
        schema_version: 1,
        query: query.into(),
        provenance,
        items,
        limitations: vec![
            "This pack preserves metadata links and does not capture or reproduce source content."
                .to_string(),
            "Rights fields are provider metadata and require source-specific review; they are not legal advice."
                .to_string(),
        ],
    }
}

/// Write a deterministic JSON evidence pack atomically.
pub fn write_evidence_pack(path: impl AsRef<Path>, pack: &EvidencePack) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension(format!(
        "{}.tmp",
        path.extension()
            .and_then(|value| value.to_str())
            .unwrap_or("json")
    ));
    let bytes = serde_json::to_vec_pretty(pack)?;
    fs::write(&temporary, bytes)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(temporary, path)?;
    Ok(())
}

/// A CSL-JSON-compatible reference with only fields supported by the source metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CslReference {
    #[serde(rename = "type")]
    pub reference_type: String,
    pub id: String,
    pub title: String,
    #[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued: Option<CslDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CslDate {
    #[serde(rename = "date-parts")]
    pub date_parts: Vec<Vec<i32>>,
}

/// Build CSL-compatible web references from normalized provider metadata.
pub fn build_csl_references(records: &[Record]) -> Vec<CslReference> {
    records
        .iter()
        .map(|record| CslReference {
            reference_type: "webpage".to_string(),
            id: citation_key(&record.id),
            title: record.title.clone(),
            url: record
                .source_url
                .clone()
                .or_else(|| record.landing_url.clone()),
            publisher: record.display_content_partner.clone().or_else(|| {
                record
                    .content_partner
                    .as_ref()
                    .and_then(|items| items.first().cloned())
            }),
            issued: record.date.as_ref().and_then(|items| {
                items.first().and_then(|value| {
                    let year = value.get(..4)?.parse::<i32>().ok()?;
                    Some(CslDate {
                        date_parts: vec![vec![year]],
                    })
                })
            }),
        })
        .collect()
}

/// Write a deterministic CSL-JSON array atomically.
pub fn write_csl_references(
    path: impl AsRef<Path>,
    references: &[CslReference],
) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(references)?)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(temporary, path)?;
    Ok(())
}

/// Write a human-readable evidence pack without implying a formal citation style.
pub fn write_evidence_pack_markdown(
    path: impl AsRef<Path>,
    pack: &EvidencePack,
) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut markdown = format!(
        "# Evidence Pack\n\n- Query: {}\n- Provenance: `{}`\n\n",
        pack.query,
        provenance_label(&pack.provenance)
    );
    markdown.push_str("## Sources\n\n");
    for item in &pack.items {
        let source = item
            .source_url
            .as_deref()
            .unwrap_or("(no source URL supplied)");
        markdown.push_str(&format!(
            "- **{}** (`{}`): {}\n",
            item.title, item.citation_key, source
        ));
        if let Some(rights) = &item.rights {
            markdown.push_str(&format!("  - Rights metadata: {}\n", rights));
        }
        if let Some(rights_url) = &item.rights_url {
            markdown.push_str(&format!("  - Rights URL: {}\n", rights_url));
        }
    }
    markdown.push_str("\n## Limitations\n\n");
    for limitation in &pack.limitations {
        markdown.push_str(&format!("- {}\n", limitation));
    }
    let temporary = path.with_extension("md.tmp");
    fs::write(&temporary, markdown)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(temporary, path)?;
    Ok(())
}

fn provenance_label(provenance: &SearchProvenance) -> &'static str {
    match provenance {
        SearchProvenance::DigitalNzMoreLikeThis { .. } => "digital_nz_more_like_this",
        SearchProvenance::LocalVector { .. } => "local_vector",
        SearchProvenance::Lexical { .. } => "lexical",
        SearchProvenance::Hybrid { .. } => "hybrid",
    }
}

fn citation_key(id: &str) -> String {
    let mut key = String::from("dnz-");
    for character in id.chars() {
        if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
            key.push(character.to_ascii_lowercase());
        } else {
            key.push('-');
        }
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_pack_preserves_source_and_distinguishes_api_mlt() {
        let record = Record {
            id: "A/1".into(),
            title: "A record".into(),
            source_url: Some("https://example.test/source".into()),
            rights: Some("CC BY".into()),
            ..Record::default()
        };
        let pack = build_evidence_pack(
            "historic map",
            &[record],
            SearchProvenance::DigitalNzMoreLikeThis {
                endpoint: "https://api.digitalnz.org/v3/records/more_like_this.json".into(),
                record_id: "A/1".into(),
            },
        );
        assert_eq!(pack.items[0].citation_key, "dnz-a-1");
        assert_eq!(
            pack.items[0].source_url.as_deref(),
            Some("https://example.test/source")
        );
        assert!(matches!(
            pack.provenance,
            SearchProvenance::DigitalNzMoreLikeThis { .. }
        ));
        assert!(pack
            .limitations
            .iter()
            .any(|item| item.contains("not legal advice")));
    }

    #[test]
    fn citations_and_markdown_are_explicitly_source_linked() {
        let record = Record {
            id: "1".into(),
            title: "A title".into(),
            source_url: Some("https://example.test/item".into()),
            date: Some(vec!["1901-02-03".into()]),
            ..Record::default()
        };
        let citations = build_csl_references(std::slice::from_ref(&record));
        assert_eq!(citations[0].reference_type, "webpage");
        assert_eq!(
            citations[0].issued.as_ref().unwrap().date_parts,
            vec![vec![1901]]
        );
        let pack = build_evidence_pack(
            "query",
            &[record],
            SearchProvenance::LocalVector {
                model: "test-model".into(),
                embedding_dimension: 3,
                index: "memory-v1".into(),
            },
        );
        let output = std::env::temp_dir().join(format!("dnz-evidence-{}.md", std::process::id()));
        write_evidence_pack_markdown(&output, &pack).unwrap();
        let markdown = fs::read_to_string(&output).unwrap();
        assert!(markdown.contains("local_vector"));
        assert!(markdown.contains("https://example.test/item"));
        let _ = fs::remove_file(output);
    }
}
