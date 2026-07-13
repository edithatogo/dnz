//! Core DigitalNZ Client Library
//! Defines models and logic for interacting with the DigitalNZ API.

pub mod autopilot;
pub mod cache;
pub mod client;
pub mod dataframe;
pub mod digest;
pub mod errors;
pub mod export;
pub mod models;
pub mod vector;

pub use autopilot::Autopilot;
pub use cache::PersistentCache;
pub use client::Client;
pub use client::QueryBuilder;
pub use dataframe::IntoDataFrame;
pub use digest::deduplicate_records;
pub use digest::generate_chronological_timeline;
pub use digest::generate_citations;
pub use digest::to_rag_xml;
pub use errors::DnzError;
pub use export::export_gazette;
pub use export::generate_frictionless_datapackage;
pub use export::generate_schema_ld;
pub use export::GazetteExportConfig;
pub use export::GazetteExportManifest;
pub use vector::cosine_similarity;
pub use vector::ensure_embedding_model;
pub use vector::DocumentVector;
pub use vector::EmbeddingModel;
pub use vector::EmbeddingModelDownload;
pub use vector::VectorStore;

/// Return a greeting to verify environment routing works.
pub fn greeting() -> &'static str {
    "Hello from dnz-core!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_returns_expected_message() {
        assert_eq!(greeting(), "Hello from dnz-core!");
    }
}
