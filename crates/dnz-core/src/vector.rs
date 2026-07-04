//! Modular traits and math helpers for local semantic vector searches.
//!
//! # Performance
//! The `MemoryVectorStore` uses a flat Vec with pre-computed query-time scoring.
//! For hot-path `query_similarity`, we use `select_nth_unstable_by` (partial sort)
//! to avoid full O(n log n) sorting; only the top-k elements are fully ordered.

use serde::{Deserialize, Serialize};
use std::path::Component;
use std::path::{Path, PathBuf};

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

/// Configuration for downloading an embedding model artifact on demand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddingModelDownload {
    /// Stable local filename for the model artifact.
    pub filename: String,
    /// HTTPS or test URL for retrieving the artifact.
    pub url: String,
}

impl EmbeddingModelDownload {
    /// Create a new download descriptor.
    pub fn new(filename: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            url: url.into(),
        }
    }
}

/// Ensure an embedding model artifact exists locally, downloading it if needed.
pub async fn ensure_embedding_model(
    cache_dir: impl AsRef<Path>,
    model: &EmbeddingModelDownload,
) -> anyhow::Result<PathBuf> {
    let cache_dir = cache_dir.as_ref();
    std::fs::create_dir_all(cache_dir)?;

    validate_model_filename(&model.filename)?;
    let model_path = cache_dir.join(&model.filename);
    if model_path.is_file() {
        return Ok(model_path);
    }

    let response = reqwest::get(&model.url).await?;
    let status = response.status();
    if !status.is_success() {
        return Err(anyhow::anyhow!(
            "failed to download embedding model from {}: HTTP {}",
            model.url,
            status
        ));
    }

    let bytes = response.bytes().await?;
    let temp_path = model_path.with_extension("download");
    tokio::fs::write(&temp_path, bytes).await?;
    if tokio::fs::rename(&temp_path, &model_path).await.is_err() {
        tokio::fs::copy(&temp_path, &model_path).await?;
        let _ = tokio::fs::remove_file(&temp_path).await;
    }

    Ok(model_path)
}

fn validate_model_filename(filename: &str) -> anyhow::Result<()> {
    let path = Path::new(filename);
    let mut components = path.components();
    let is_single_normal_component = matches!(components.next(), Some(Component::Normal(_)))
        && components.next().is_none()
        && !filename.trim().is_empty();

    if is_single_normal_component {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "embedding model filename must be a single safe path component"
        ))
    }
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
    use std::time::{SystemTime, UNIX_EPOCH};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn temp_model_dir(name: &str) -> PathBuf {
        let unique = format!(
            "dnz-model-{name}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock should be after unix epoch")
                .as_nanos()
        );
        std::env::temp_dir().join(unique)
    }

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

    #[tokio::test]
    async fn ensure_embedding_model_downloads_missing_file() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/model.bin"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes("model-bytes"))
            .expect(1)
            .mount(&server)
            .await;

        let dir = temp_model_dir("download");
        let model = EmbeddingModelDownload::new("model.bin", format!("{}/model.bin", server.uri()));

        let path = ensure_embedding_model(&dir, &model)
            .await
            .expect("model should download");

        assert_eq!(std::fs::read(&path).expect("model file"), b"model-bytes");

        let _ = std::fs::remove_dir_all(dir);
    }

    #[tokio::test]
    async fn ensure_embedding_model_reuses_existing_file() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/model.bin"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes("new-model-bytes"))
            .expect(0)
            .mount(&server)
            .await;

        let dir = temp_model_dir("reuse");
        std::fs::create_dir_all(&dir).expect("temp dir");
        let existing = dir.join("model.bin");
        std::fs::write(&existing, b"existing-model-bytes").expect("existing model");
        let model = EmbeddingModelDownload::new("model.bin", format!("{}/model.bin", server.uri()));

        let path = ensure_embedding_model(&dir, &model)
            .await
            .expect("existing model should be reused");

        assert_eq!(path, existing);
        assert_eq!(
            std::fs::read(&path).expect("model file"),
            b"existing-model-bytes"
        );

        let _ = std::fs::remove_dir_all(dir);
    }

    #[tokio::test]
    async fn ensure_embedding_model_rejects_path_traversal_filename() {
        let dir = temp_model_dir("unsafe");
        let model = EmbeddingModelDownload::new("../model.bin", "http://127.0.0.1/model.bin");

        let err = ensure_embedding_model(&dir, &model)
            .await
            .expect_err("unsafe model filename should be rejected");

        assert!(err.to_string().contains("single safe path component"));

        let _ = std::fs::remove_dir_all(dir);
    }
}
