//! Modular traits and math helpers for local semantic vector searches.

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
    fn query_similarity(&self, query_vector: &[f32], limit: usize) -> anyhow::Result<Vec<(String, f32)>>;
}

/// Abstract representation of an embedding engine.
pub trait EmbeddingModel {
    /// Compute float array embedding for input text.
    fn embed(&self, text: &str) -> anyhow::Result<Vec<f32>>;
}

/// Compute cosine similarity between two float vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot_product += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

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
        let res = self.vectors.iter()
            .find(|v| v.record_id == record_id)
            .map(|v| v.embedding.clone());
        Ok(res)
    }

    fn query_similarity(&self, query_vector: &[f32], limit: usize) -> anyhow::Result<Vec<(String, f32)>> {
        let mut scores: Vec<(String, f32)> = self.vectors.iter()
            .map(|v| {
                let score = cosine_similarity(query_vector, &v.embedding);
                (v.record_id.clone(), score)
            })
            .collect();

        // Sort descending by score
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(limit);
        Ok(scores)
    }
}
