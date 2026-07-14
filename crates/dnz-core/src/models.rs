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
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub id: String,
    /// Main title of the item.
    pub title: String,
    /// Detailed description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional provider-supplied description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_description: Option<String>,
    /// Provider creation timestamp or source date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Provider update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Collections this item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Vec<String>>,
    /// Content partner institutions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_partner: Option<Vec<String>>,
    /// Display-ready partner label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content_partner: Option<String>,
    /// Display-ready collection label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_collection: Option<String>,
    /// Primary collection identifier or label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_collection: Option<String>,
    /// Collection title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_title: Option<String>,
    /// Creator names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Vec<String>>,
    /// Subjects assigned by the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Vec<String>>,
    /// Dublin Core identifier values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_identifier: Option<Vec<String>>,
    /// Dublin Core type values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_type: Option<Vec<String>>,
    /// Format values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Vec<String>>,
    /// Language values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    /// Place names associated with the record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placename: Option<Vec<String>>,
    /// Display URL in DigitalNZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    /// Original URL from source system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// Provider landing URL; source_url remains preferred for public links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_url: Option<String>,
    /// Thumbnail URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Large thumbnail URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_thumbnail_url: Option<String>,
    /// General categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<String>>,
    /// Dates associated with the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Vec<String>>,
    /// Display-formatted date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_date: Option<String>,
    /// Syndication timestamps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syndication_date: Option<String>,
    /// Whether the provider marks this record as commercially usable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_commercial_use: Option<bool>,
    /// Provider usage statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    /// Provider copyright statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    /// Rights statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,
    /// Rights URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights_url: Option<String>,
    /// Location payload, retained as JSON because provider shapes vary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<serde_json::Value>,
    /// Provider-specific fields not yet modeled by dnz.
    #[serde(flatten, default, skip_serializing_if = "HashMap::is_empty")]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(value) => Ok(value),
        serde_json::Value::Number(value) => Ok(value.to_string()),
        other => Err(serde::de::Error::custom(format!(
            "record id must be a string or number, got {other}"
        ))),
    }
}

/// Normalize the verified v3 record response shapes into one record.
///
/// The public documentation describes metadata responses as using the same
/// format as search results, while OpenAPI/fixtures also expose a direct
/// object or a `record`/`results` wrapper. Unknown record fields are retained
/// by [`Record::extra_fields`].
pub fn normalize_record_response(value: serde_json::Value) -> anyhow::Result<Record> {
    let candidate = match value {
        serde_json::Value::Object(mut object) => {
            if let Some(record) = object.remove("record") {
                record
            } else if let Some(search) = object.remove("search") {
                first_record_from_collection(search)?
            } else if let Some(results) = object.remove("results") {
                first_record_from_collection(results)?
            } else {
                serde_json::Value::Object(object)
            }
        }
        other => other,
    };

    serde_json::from_value(candidate).map_err(Into::into)
}

/// Normalize the documented search envelope and the flat OpenAPI result shape.
pub fn normalize_search_response(value: serde_json::Value) -> anyhow::Result<SearchResponse> {
    match value {
        serde_json::Value::Object(mut object) => {
            if let Some(search) = object.remove("search") {
                return Ok(SearchResponse {
                    search: serde_json::from_value(search)?,
                });
            }
            if object.contains_key("results") || object.contains_key("records") {
                let records = object
                    .remove("results")
                    .or_else(|| object.remove("records"))
                    .unwrap_or_else(|| serde_json::Value::Array(Vec::new()));
                let results: Vec<Record> = serde_json::from_value(records)?;
                let result_count = object
                    .remove("result_count")
                    .and_then(|value| value.as_u64())
                    .unwrap_or_else(|| results.len() as u64);
                return Ok(SearchResponse {
                    search: SearchMetadata {
                        result_count,
                        results,
                        facets: HashMap::new(),
                    },
                });
            }
            Err(anyhow::anyhow!(
                "search response contains no result collection"
            ))
        }
        serde_json::Value::Array(records) => Ok(SearchResponse {
            search: SearchMetadata {
                result_count: records.len() as u64,
                results: serde_json::from_value(serde_json::Value::Array(records))?,
                facets: HashMap::new(),
            },
        }),
        _ => Err(anyhow::anyhow!("search response is not an object or array")),
    }
}

fn first_record_from_collection(value: serde_json::Value) -> anyhow::Result<serde_json::Value> {
    let results = match value {
        serde_json::Value::Object(mut object) => object
            .remove("results")
            .ok_or_else(|| anyhow::anyhow!("record response contains no results"))?,
        other => other,
    };
    match results {
        serde_json::Value::Array(mut records) => records
            .drain(..)
            .next()
            .ok_or_else(|| anyhow::anyhow!("record response contains no records")),
        _ => Err(anyhow::anyhow!("record response results is not an array")),
    }
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
        assert!(record.extra_fields.is_empty());
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
                        "category": ["Images"],
                        "license": "CC-BY",
                        "usage": "Some rights reserved"
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
        assert_eq!(
            response.search.results[0].extra_fields["license"],
            serde_json::Value::String("CC-BY".to_string())
        );
        assert_eq!(
            response.search.results[0].usage.as_deref(),
            Some("Some rights reserved")
        );
        let category_facets = &response.search.facets["category"];
        assert_eq!(category_facets.get("Images"), Some(&10));
        assert_eq!(category_facets.get("Text"), Some(&5));
    }

    #[test]
    fn record_id_accepts_integer_and_preserves_unknown_fields() {
        let record: Record = serde_json::from_value(serde_json::json!({
            "id": 37757055,
            "title": "A record",
            "is_commercial_use": true,
            "provider_extension": {"raw": "kept"}
        }))
        .unwrap();

        assert_eq!(record.id, "37757055");
        assert_eq!(record.is_commercial_use, Some(true));
        assert_eq!(record.extra_fields["provider_extension"]["raw"], "kept");
    }

    #[test]
    fn normalize_record_response_accepts_standardized_wrappers() {
        let wrapped = serde_json::json!({
            "search": {"result_count": 1, "results": [{"id": 12, "title": "Wrapped"}]}
        });
        let direct = serde_json::json!({"record": {"id": "13", "title": "Direct"}});

        assert_eq!(normalize_record_response(wrapped).unwrap().id, "12");
        assert_eq!(normalize_record_response(direct).unwrap().title, "Direct");
    }

    #[test]
    fn normalize_search_response_accepts_flat_records() {
        let response = normalize_search_response(serde_json::json!({
            "result_count": 2,
            "records": [{"id": 1, "title": "One"}, {"id": "2", "title": "Two"}]
        }))
        .unwrap();

        assert_eq!(response.search.result_count, 2);
        assert_eq!(response.search.results[0].id, "1");
        assert_eq!(response.search.results[1].id, "2");
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
            extra_fields: HashMap::from([(
                "license".to_string(),
                serde_json::Value::String("CC-BY".to_string()),
            )]),
            ..Record::default()
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
        assert_eq!(
            deserialized.extra_fields["license"],
            serde_json::Value::String("CC-BY".to_string())
        );
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
            extra_fields: HashMap::new(),
            ..Record::default()
        };
        let json = serde_json::to_string(&rec).expect("serialize");
        assert!(!json.contains("description"));
        let back: Record = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.id, "1");
        assert_eq!(back.title, "Minimal");
        assert!(back.description.is_none());
    }
}
