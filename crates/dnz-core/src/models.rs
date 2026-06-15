//! Models mapping to DigitalNZ API v3 schemas.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response returned from a search query.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SearchResponse {
    /// Summary of metadata containing result counts.
    pub search: SearchMetadata,
}

/// Metadata block within search response.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SearchMetadata {
    /// Total number of records matching query.
    pub result_count: u64,
    /// List of records in current page.
    pub results: Vec<Record>,
    /// Optional dictionary of facets returned.
    #[serde(default)]
    pub facets: HashMap<String, HashMap<String, u64>>,
}

/// A single heritage record from DigitalNZ.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Record {
    /// Unique identifier.
    pub id: String,
    /// Main title of the item.
    pub title: String,
    /// Detailed description.
    pub description: Option<String>,
    /// Collections this item belongs to.
    pub collection: Option<Vec<String>>,
    /// Content partner institutions.
    pub content_partner: Option<Vec<String>>,
    /// Creator names.
    pub creator: Option<Vec<String>>,
    /// Display URL in DigitalNZ.
    pub display_url: Option<String>,
    /// Original URL from source system.
    pub source_url: Option<String>,
    /// General categories.
    pub category: Option<Vec<String>>,
    /// Dates associated with the item.
    pub date: Option<Vec<String>>,
    /// Syndication timestamps.
    pub syndication_date: Option<String>,
}
