//! Core DigitalNZ Client Library
//! Defines models and logic for interacting with the DigitalNZ API.

pub mod autopilot;
pub mod client;
pub mod dataframe;
pub mod digest;
pub mod export;
pub mod models;
pub mod vector;

pub use autopilot::Autopilot;
pub use client::Client;
pub use client::QueryBuilder;
pub use dataframe::IntoDataFrame;
pub use digest::deduplicate_records;
pub use digest::generate_chronological_timeline;
pub use digest::generate_citations;
pub use digest::to_rag_xml;
pub use export::generate_frictionless_datapackage;
pub use export::generate_schema_ld;
pub use vector::cosine_similarity;
pub use vector::DocumentVector;
pub use vector::EmbeddingModel;
pub use vector::VectorStore;

/// Return a greeting to verify environment routing works.
pub fn greeting() -> &'static str {
    "Hello from dnz-core!"
}
