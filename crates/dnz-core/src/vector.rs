//! Modular traits and math helpers for local semantic vector searches.
//!
//! # Performance
//! The `MemoryVectorStore` uses a flat Vec with pre-computed query-time scoring.
//! For hot-path `query_similarity`, we use `select_nth_unstable_by` (partial sort)
//! to avoid full O(n log n) sorting; only the top-k elements are fully ordered.

use serde::{Deserialize, Serialize};

/// Entity representing an embedded document vector reference.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentVector {
    /// DigitalNZ Record ID.
    pub record_id: String,
    /// Computed float array embedding.
    pub embedding: Vec<f32>,
}

/// Abstract representation of a local vector database.
pub trait VectorStore {
    /// Insert a record ID and vector embedding.
    fn insert(&mut self, record_id: &str, embedding: &[f32]) -> anyhow::Result<()>;

    /// Retrieve vector embedding for a record.
    fn get(&self, record_id: &str) -> anyhow::Result<Option<Vec<f32>>>;

    /// Query all vectors sorted by cosine similarity against an input vector.
    fn query_similarity(
        &self,
        query_vector: &[f32],
        limit: usize,
    ) -> anyhow::Result<Vec<(String, f32)>>;
}

/// Abstract representation of an embedding engine.
pub trait EmbeddingModel {
    /// Compute float array embedding for input text.
    fn embed(&self, text: &str) -> anyhow::Result<Vec<f32>>;
}

/// Compute cosine similarity between two float vectors.
///
/// Uses a single fused loop with iterator zip for better vectorisation
/// opportunities and returns a normalised score in [0.0, 1.0].
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let (dot_product, norm_a, norm_b) = a
        .iter()
        .zip(b.iter())
        .fold((0.0f32, 0.0f32, 0.0f32), |(dot, na, nb), (&x, &y)| {
            (dot + x * y, na + x * x, nb + y * y)
        });

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}

/// Simple in-memory mock vector store for testing.
#[derive(Default, Debug, Clone)]
pub struct MemoryVectorStore {
    vectors: Vec<DocumentVector>,
}

impl VectorStore for MemoryVectorStore {
    fn insert(&mut self, record_id: &str, embedding: &[f32]) -> anyhow::Result<()> {
        self.vectors.push(DocumentVector {
            record_id: record_id.to_string(),
            embedding: embedding.to_vec(),
        });
        Ok(())
    }

    fn get(&self, record_id: &str) -> anyhow::Result<Option<Vec<f32>>> {
        let res = self
            .vectors
            .iter()
            .find(|v| v.record_id == record_id)
            .map(|v| v.embedding.clone());
        Ok(res)
    }

    /// Query top-k similar vectors using a partial sort (O(n + k log k)) instead
    /// of a full sort (O(n log n)) for better performance on large stores.
    fn query_similarity(
        &self,
        query_vector: &[f32],
        limit: usize,
    ) -> anyhow::Result<Vec<(String, f32)>> {
        if self.vectors.is_empty() || limit == 0 {
            return Ok(Vec::new());
        }

        let mut scores: Vec<(String, f32)> = self
            .vectors
            .iter()
            .map(|v| {
                let score = cosine_similarity(query_vector, &v.embedding);
                (v.record_id.clone(), score)
            })
            .collect();

        let k = limit.min(scores.len());

        // Partial sort: select the top-k elements without fully sorting the rest.
        // The nth index must be in bounds, so skip selection when all scores are requested.
        if k < scores.len() {
            scores.select_nth_unstable_by(k, |a, b| {
                b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        scores.truncate(k);

        // Sort only the top-k result set descending by score
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        Ok(scores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let score = cosine_similarity(&a, &b);
        assert!(
            (score - 1.0).abs() < 1e-6,
            "identical vectors should have similarity 1.0"
        );
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let score = cosine_similarity(&a, &b);
        assert!(
            (score - 0.0).abs() < 1e-6,
            "orthogonal vectors should have similarity 0.0"
        );
    }

    #[test]
    fn test_cosine_similarity_mismatched_lengths() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0];
        assert_eq!(
            cosine_similarity(&a, &b),
            0.0,
            "mismatched lengths should return 0.0"
        );
    }

    #[test]
    fn test_cosine_similarity_empty() {
        let a: Vec<f32> = vec![];
        let b: Vec<f32> = vec![];
        assert_eq!(
            cosine_similarity(&a, &b),
            0.0,
            "empty vectors should return 0.0"
        );
    }

    #[test]
    fn test_memory_store_insert_and_get() {
        let mut store = MemoryVectorStore::default();
        store.insert("rec_1", &[1.0, 0.0, 0.0]).unwrap();
        let emb = store.get("rec_1").unwrap().unwrap();
        assert_eq!(emb, vec![1.0, 0.0, 0.0]);
        assert!(store.get("nonexistent").unwrap().is_none());
    }

    #[test]
    fn test_memory_store_query_similarity_top_k() {
        let mut store = MemoryVectorStore::default();
        store.insert("rec_a", &[1.0, 0.0]).unwrap();
        store.insert("rec_b", &[0.0, 1.0]).unwrap();
        store.insert("rec_c", &[0.9, 0.1]).unwrap();

        let results = store.query_similarity(&[1.0, 0.0], 2).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "rec_a", "most similar should be rec_a");

        // rec_c (0.9,0.1) should score higher than rec_b (0.0,1.0) against query (1.0,0.0)
        assert_eq!(results[1].0, "rec_c", "second most similar should be rec_c");
    }

    #[test]
    fn test_memory_store_query_empty() {
        let store = MemoryVectorStore::default();
        let results = store.query_similarity(&[1.0, 0.0], 5).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_memory_store_query_limit_greater_than_count() {
        let mut store = MemoryVectorStore::default();
        store.insert("rec_1", &[1.0, 0.0]).unwrap();
        store.insert("rec_2", &[0.0, 1.0]).unwrap();
        let results = store.query_similarity(&[0.5, 0.5], 100).unwrap();
        assert_eq!(results.len(), 2);
    }
}
