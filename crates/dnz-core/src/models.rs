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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Collections this item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Vec<String>>,
    /// Content partner institutions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_partner: Option<Vec<String>>,
    /// Creator names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Vec<String>>,
    /// Display URL in DigitalNZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    /// Original URL from source system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// General categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<String>>,
    /// Dates associated with the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    /// Syndication timestamps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syndication_date: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_default() {
        let record = Record::default();
        assert_eq!(record.id, "");
        assert_eq!(record.title, "");
        assert!(record.description.is_none());
        assert!(record.collection.is_none());
        assert!(record.content_partner.is_none());
        assert!(record.creator.is_none());
        assert!(record.display_url.is_none());
        assert!(record.source_url.is_none());
        assert!(record.category.is_none());
        assert!(record.date.is_none());
        assert!(record.syndication_date.is_none());
    }

    #[test]
    fn test_search_metadata_default() {
        let metadata = SearchMetadata::default();
        assert_eq!(metadata.result_count, 0);
        assert!(metadata.results.is_empty());
        assert!(metadata.facets.is_empty());
    }

    #[test]
    fn test_search_response_default() {
        let resp = SearchResponse::default();
        assert_eq!(resp.search.result_count, 0);
    }

    #[test]
    fn test_search_response_deserialization() {
        let json = r#"{
            "search": {
                "result_count": 42,
                "results": [
                    {
                        "id": "12345",
                        "title": "Test Record",
                        "description": "A description",
                        "content_partner": ["Partner A"],
                        "category": ["Images"]
                    }
                ],
                "facets": {
                    "category": {
                        "Images": 10,
                        "Text": 5
                    }
                }
            }
        }"#;
        let response: SearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.search.result_count, 42);
        assert_eq!(response.search.results.len(), 1);
        assert_eq!(response.search.results[0].id, "12345");
        assert_eq!(response.search.results[0].title, "Test Record");
        assert_eq!(
            response.search.results[0].description.as_deref(),
            Some("A description")
        );
        assert_eq!(
            response.search.results[0].content_partner,
            Some(vec!["Partner A".to_string()])
        );
        assert_eq!(
            response.search.results[0].category,
            Some(vec!["Images".to_string()])
        );
        assert!(response.search.results[0].creator.is_none());
        assert!(response.search.results[0].source_url.is_none());
        assert!(response.search.results[0].date.is_none());
        let category_facets = &response.search.facets["category"];
        assert_eq!(category_facets.get("Images"), Some(&10));
        assert_eq!(category_facets.get("Text"), Some(&5));
    }

    #[test]
    fn test_record_serialization_roundtrip() {
        let record = Record {
            id: "rec-1".to_string(),
            title: "Kauri Tree".to_string(),
            description: Some("A large native tree".to_string()),
            collection: Some(vec!["Auckland Museum".to_string()]),
            content_partner: Some(vec!["Partner".to_string()]),
            creator: Some(vec!["Creator".to_string()]),
            display_url: Some("https://digitalnz.org/records/rec-1".to_string()),
            source_url: Some("https://example.test/1".to_string()),
            category: Some(vec!["Images".to_string()]),
            date: Some(vec!["1900".to_string()]),
            syndication_date: Some("2024-01-01".to_string()),
        };
        let json = serde_json::to_string(&record).unwrap();
        let deserialized: Record = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "rec-1");
        assert_eq!(deserialized.title, "Kauri Tree");
        assert_eq!(
            deserialized.description.as_deref(),
            Some("A large native tree")
        );
        assert_eq!(
            deserialized.collection,
            Some(vec!["Auckland Museum".to_string()])
        );
        assert_eq!(
            deserialized.content_partner,
            Some(vec!["Partner".to_string()])
        );
        assert_eq!(deserialized.creator, Some(vec!["Creator".to_string()]));
        assert_eq!(
            deserialized.display_url.as_deref(),
            Some("https://digitalnz.org/records/rec-1")
        );
        assert_eq!(
            deserialized.source_url.as_deref(),
            Some("https://example.test/1")
        );
        assert_eq!(deserialized.category, Some(vec!["Images".to_string()]));
        assert_eq!(deserialized.date, Some(vec!["1900".to_string()]));
        assert_eq!(deserialized.syndication_date.as_deref(), Some("2024-01-01"));
    }

    #[test]
    fn test_record_serialization_omits_none_fields() {
        let rec = Record {
            id: "1".to_string(),
            title: "Minimal".to_string(),
            description: None,
            collection: None,
            content_partner: None,
            creator: None,
            display_url: None,
            source_url: None,
            category: None,
            date: None,
            syndication_date: None,
        };
        let json = serde_json::to_string(&rec).expect("serialize");
        assert!(!json.contains("description"));
        let back: Record = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.id, "1");
        assert_eq!(back.title, "Minimal");
        assert!(back.description.is_none());
    }
}
