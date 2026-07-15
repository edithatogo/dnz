//! Core DigitalNZ Client Library
//! Defines models and logic for interacting with the DigitalNZ API.

pub mod autopilot;
pub mod cache;
pub mod client;
#[cfg(feature = "dataframe")]
pub mod dataframe;
pub mod digest;
pub mod errors;
pub mod evidence;
pub mod export;
pub mod models;
#[cfg(feature = "parquet")]
pub mod parquet;
pub mod quality;
pub mod sync;
pub mod vector;

pub use autopilot::{plan_density_partitions, Autopilot, DensityPartition, HarvestOptions};
pub use cache::{CacheEntry, CacheProvenance, PersistentCache};
pub use client::Client;
pub use client::FilterExpr;
pub use client::MoreLikeThisQueryBuilder;
pub use client::QueryBuilder;
pub use client::RecordQueryBuilder;
pub use client::RecordStream;
pub use client::SearchPageStream;
#[cfg(feature = "dataframe")]
pub use dataframe::IntoDataFrame;
pub use digest::deduplicate_records;
pub use digest::generate_chronological_timeline;
pub use digest::generate_citations;
pub use digest::to_rag_xml;
pub use errors::DnzError;
pub use evidence::{
    build_evidence_pack, write_evidence_pack, EvidenceItem, EvidencePack, SearchProvenance,
};
pub use export::export_gazette;
pub use export::generate_frictionless_datapackage;
pub use export::generate_schema_ld;
pub use export::write_records_csv;
pub use export::write_records_geojson;
pub use export::write_records_jsonl;
pub use export::write_records_sqlite;
pub use export::GazetteExportConfig;
pub use export::GazetteExportManifest;
pub use export::{
    build_export_provenance, generate_ro_crate_metadata, write_export_provenance, ExportProvenance,
};
pub use models::normalize_record_response;
pub use models::normalize_rss_record_response;
pub use models::normalize_rss_search_response;
pub use models::normalize_search_response;
pub use models::normalize_xml_record_response;
pub use models::normalize_xml_search_response;
#[cfg(feature = "parquet")]
pub use parquet::write_records_parquet;
pub use quality::{assess_data_quality, audit_rights_reuse, DataQualityReport, RightsReuseAudit};
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
