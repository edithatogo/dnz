//! Core DigitalNZ Client Library
//! Defines models and logic for interacting with the DigitalNZ API.

pub mod autopilot;
pub mod cache;
pub mod client;
#[cfg(feature = "dataframe")]
pub mod dataframe;
pub mod digest;
pub mod errors;
pub mod export;
pub mod models;
pub mod sync;
pub mod vector;

pub use autopilot::{Autopilot, HarvestOptions};
pub use cache::{CacheEntry, CacheProvenance, PersistentCache};
pub use client::Client;
pub use client::FilterExpr;
pub use client::MoreLikeThisQueryBuilder;
pub use client::QueryBuilder;
pub use client::RecordQueryBuilder;
pub use client::SearchPageStream;
#[cfg(feature = "dataframe")]
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
pub use models::normalize_record_response;
pub use models::normalize_rss_record_response;
pub use models::normalize_rss_search_response;
pub use models::normalize_search_response;
pub use models::normalize_xml_record_response;
pub use models::normalize_xml_search_response;
pub use sync::{
    build_incremental_sync_manifest, render_incremental_sync_manifest,
    write_incremental_sync_manifest, IncrementalSyncManifest, SyncRecord,
};
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
