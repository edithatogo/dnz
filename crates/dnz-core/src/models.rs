//! Models mapping to DigitalNZ API v3 schemas.

use quick_xml::events::Event;
use quick_xml::Reader;
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
    /// One-based page number returned by the provider, when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Requested page size returned by the provider, when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    /// List of records in current page.
    pub results: Vec<Record>,
    /// Optional dictionary of facets returned.
    #[serde(default)]
    pub facets: HashMap<String, HashMap<String, u64>>,
    /// Request metadata returned by the provider, preserved without guessing its shape.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
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
                    .unwrap_or(results.len() as u64);
                let page = object
                    .remove("page")
                    .and_then(|value| value.as_u64())
                    .and_then(|value| u32::try_from(value).ok());
                let per_page = object
                    .remove("per_page")
                    .and_then(|value| value.as_u64())
                    .and_then(|value| u32::try_from(value).ok());
                let facets = object
                    .remove("facets")
                    .map(serde_json::from_value)
                    .transpose()?
                    .unwrap_or_default();
                let request = object.remove("request");
                return Ok(SearchResponse {
                    search: SearchMetadata {
                        result_count,
                        page,
                        per_page,
                        results,
                        facets,
                        request,
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
                page: None,
                per_page: None,
                results: serde_json::from_value(serde_json::Value::Array(records))?,
                facets: HashMap::new(),
                request: None,
            },
        }),
        _ => Err(anyhow::anyhow!("search response is not an object or array")),
    }
}

/// Normalize the verified DigitalNZ v3 XML search response shape.
pub fn normalize_xml_search_response(xml: &[u8]) -> anyhow::Result<SearchResponse> {
    let root = parse_xml_document(xml)?;
    if root.name != "search" {
        anyhow::bail!("XML search response root must be <search>");
    }

    let result_count = child_text(&root, &["result-count", "result_count"])
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or_default();
    let page = child_text(&root, &["page"]).and_then(|value| value.parse::<u32>().ok());
    let per_page =
        child_text(&root, &["per-page", "per_page"]).and_then(|value| value.parse::<u32>().ok());
    let request = child_text(&root, &["request-url", "request_url"])
        .map(|url| serde_json::json!({"url": url}));
    let results = root
        .children
        .iter()
        .find(|child| child.name == "results")
        .map(|results| {
            results
                .children
                .iter()
                .filter(|child| child.name == "result")
                .map(xml_record_value)
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .transpose()?
        .unwrap_or_default();

    Ok(SearchResponse {
        search: SearchMetadata {
            result_count,
            page,
            per_page,
            results: results
                .into_iter()
                .map(serde_json::from_value)
                .collect::<Result<Vec<Record>, _>>()?,
            facets: HashMap::new(),
            request,
        },
    })
}

/// Normalize one record returned in the verified DigitalNZ v3 XML format.
pub fn normalize_xml_record_response(xml: &[u8]) -> anyhow::Result<Record> {
    let root = parse_xml_document(xml)?;
    if root.name == "result" || root.name == "record" {
        return serde_json::from_value(xml_record_value(&root)?).map_err(Into::into);
    }
    normalize_xml_search_response(xml)?
        .search
        .results
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("XML record response contains no records"))
}

/// Normalize a standard RSS 2.0 response into the common search model.
pub fn normalize_rss_search_response(xml: &[u8]) -> anyhow::Result<SearchResponse> {
    let root = parse_xml_document(xml)?;
    let records = descendant_nodes(&root, "item")
        .iter()
        .map(|item| serde_json::from_value(rss_item_value(item)))
        .collect::<Result<Vec<Record>, _>>()?;
    Ok(SearchResponse {
        search: SearchMetadata {
            result_count: records.len() as u64,
            page: None,
            per_page: None,
            results: records,
            facets: HashMap::new(),
            request: None,
        },
    })
}

/// Normalize the first RSS item as a record response.
pub fn normalize_rss_record_response(xml: &[u8]) -> anyhow::Result<Record> {
    normalize_rss_search_response(xml)?
        .search
        .results
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("RSS response contains no items"))
}

#[derive(Debug, Default)]
struct XmlNode {
    name: String,
    text: String,
    children: Vec<XmlNode>,
}

fn descendant_nodes<'a>(node: &'a XmlNode, name: &str) -> Vec<&'a XmlNode> {
    let mut matches = Vec::new();
    if node.name.rsplit(':').next().unwrap_or(&node.name) == name {
        matches.push(node);
    }
    for child in &node.children {
        matches.extend(descendant_nodes(child, name));
    }
    matches
}

fn rss_item_value(item: &XmlNode) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    let mut repeated: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    for child in &item.children {
        let name = child.name.rsplit(':').next().unwrap_or(&child.name);
        let key = match name {
            "guid" => "id",
            "link" => "source_url",
            "pubDate" | "published" | "updated" => "syndication_date",
            other => other,
        }
        .replace('-', "_");
        let value = serde_json::Value::String(child.text.clone());
        if matches!(key.as_str(), "category" | "subject" | "creator") {
            repeated.entry(key).or_default().push(value);
        } else {
            object.insert(key, value);
        }
    }
    for (key, values) in repeated {
        object.insert(key, serde_json::Value::Array(values));
    }
    object
        .entry("id")
        .or_insert_with(|| serde_json::Value::String(String::new()));
    object
        .entry("title")
        .or_insert_with(|| serde_json::Value::String(String::new()));
    serde_json::Value::Object(object)
}

fn parse_xml_document(xml: &[u8]) -> anyhow::Result<XmlNode> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();
    let mut stack: Vec<XmlNode> = Vec::new();
    let mut root = None;

    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(start) => stack.push(XmlNode {
                name: String::from_utf8(start.name().as_ref().to_vec())?,
                ..XmlNode::default()
            }),
            Event::Empty(empty) => attach_xml_node(
                &mut stack,
                &mut root,
                XmlNode {
                    name: String::from_utf8(empty.name().as_ref().to_vec())?,
                    ..XmlNode::default()
                },
            )?,
            Event::Text(text) => {
                if let Some(node) = stack.last_mut() {
                    node.text.push_str(&text.unescape()?);
                }
            }
            Event::CData(text) => {
                if let Some(node) = stack.last_mut() {
                    node.text.push_str(&String::from_utf8_lossy(text.as_ref()));
                }
            }
            Event::End(_) => {
                let node = stack
                    .pop()
                    .ok_or_else(|| anyhow::anyhow!("XML response has an unexpected end tag"))?;
                attach_xml_node(&mut stack, &mut root, node)?;
            }
            Event::DocType(_) => {
                anyhow::bail!("XML response contains unsupported entity declarations")
            }
            Event::Eof => break,
            Event::Decl(_) | Event::Comment(_) | Event::PI(_) => {}
        }
        buffer.clear();
    }

    if !stack.is_empty() {
        anyhow::bail!("XML response is not well formed");
    }
    root.ok_or_else(|| anyhow::anyhow!("XML response is empty"))
}

fn attach_xml_node(
    stack: &mut [XmlNode],
    root: &mut Option<XmlNode>,
    node: XmlNode,
) -> anyhow::Result<()> {
    if let Some(parent) = stack.last_mut() {
        parent.children.push(node);
    } else if root.replace(node).is_some() {
        anyhow::bail!("XML response contains multiple roots");
    }
    Ok(())
}

fn child_text<'a>(node: &'a XmlNode, names: &[&str]) -> Option<&'a str> {
    node.children
        .iter()
        .find(|child| names.contains(&child.name.as_str()))
        .map(|child| child.text.as_str())
}

fn xml_record_value(node: &XmlNode) -> anyhow::Result<serde_json::Value> {
    let mut object = serde_json::Map::new();
    for child in &node.children {
        let key = child.name.replace('-', "_");
        let mut value = xml_node_value(child);
        if matches!(
            key.as_str(),
            "category"
                | "collection"
                | "content_partner"
                | "creator"
                | "date"
                | "dc_identifier"
                | "dc_type"
                | "format"
                | "language"
                | "placename"
                | "subject"
        ) && !value.is_array()
        {
            value = serde_json::Value::Array(vec![value]);
        }
        if key == "is_commercial_use" {
            if let Some(text) = value.as_str() {
                value = serde_json::Value::Bool(text.eq_ignore_ascii_case("true"));
            }
        }
        if let Some(existing) = object.remove(&key) {
            let mut values = match existing {
                serde_json::Value::Array(values) => values,
                other => vec![other],
            };
            match value {
                serde_json::Value::Array(mut additional) => values.append(&mut additional),
                other => values.push(other),
            }
            object.insert(key, serde_json::Value::Array(values));
        } else {
            object.insert(key, value);
        }
    }
    Ok(serde_json::Value::Object(object))
}

fn xml_node_value(node: &XmlNode) -> serde_json::Value {
    if node.children.is_empty() {
        return serde_json::Value::String(node.text.clone());
    }
    if node
        .children
        .iter()
        .all(|child| child.name == node.children[0].name)
    {
        return serde_json::Value::Array(node.children.iter().map(xml_node_value).collect());
    }
    let mut object = serde_json::Map::new();
    if !node.text.is_empty() {
        object.insert(
            "_text".to_string(),
            serde_json::Value::String(node.text.clone()),
        );
    }
    for child in &node.children {
        object.insert(child.name.replace('-', "_"), xml_node_value(child));
    }
    serde_json::Value::Object(object)
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
            "page": 3,
            "per_page": 25,
            "facets": {"category": {"Images": 2}},
            "request": {"sort": "date"},
            "records": [{"id": 1, "title": "One"}, {"id": "2", "title": "Two"}]
        }))
        .unwrap();

        assert_eq!(response.search.result_count, 2);
        assert_eq!(response.search.page, Some(3));
        assert_eq!(response.search.per_page, Some(25));
        assert_eq!(response.search.facets["category"]["Images"], 2);
        assert_eq!(
            response.search.request,
            Some(serde_json::json!({"sort": "date"}))
        );
        assert_eq!(response.search.results[0].id, "1");
        assert_eq!(response.search.results[1].id, "2");
    }

    #[test]
    fn normalize_search_response_preserves_envelope_metadata() {
        let response = normalize_search_response(serde_json::json!({
            "search": {
                "result_count": 1,
                "page": 2,
                "per_page": 10,
                "request": {"text": "harbour"},
                "results": [{"id": "1", "title": "Harbour"}]
            }
        }))
        .unwrap();

        assert_eq!(response.search.page, Some(2));
        assert_eq!(response.search.per_page, Some(10));
        assert_eq!(
            response.search.request,
            Some(serde_json::json!({"text": "harbour"}))
        );
    }

    #[test]
    fn normalize_xml_search_response_accepts_verified_v3_shape() {
        let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<search>
  <page type="integer">1</page>
  <per-page type="integer">1</per-page>
  <result-count type="integer">1</result-count>
  <request-url>https://api.example.test/records.xml?text=kiwi&amp;per_page=1</request-url>
  <results type="array">
    <result>
      <id type="integer">41278482</id>
      <title>John &amp; Jane</title>
      <content-partner>Partner A</content-partner>
      <subject>history</subject>
      <subject>people</subject>
      <is-commercial-use>true</is-commercial-use>
      <provider-extension>kept</provider-extension>
    </result>
  </results>
  <facets type="array"/>
</search>"#;

        let response = normalize_xml_search_response(xml).unwrap();
        assert_eq!(response.search.page, Some(1));
        assert_eq!(response.search.per_page, Some(1));
        assert_eq!(response.search.result_count, 1);
        assert_eq!(response.search.results[0].id, "41278482");
        assert_eq!(response.search.results[0].title, "John & Jane");
        assert_eq!(
            response.search.results[0].subject,
            Some(vec!["history".to_string(), "people".to_string()])
        );
        assert_eq!(response.search.results[0].is_commercial_use, Some(true));
        assert_eq!(
            response.search.results[0].extra_fields["provider_extension"],
            "kept"
        );
        assert_eq!(
            response.search.request,
            Some(serde_json::json!({
                "url": "https://api.example.test/records.xml?text=kiwi&per_page=1"
            }))
        );
    }

    #[test]
    fn normalize_xml_record_response_accepts_direct_result() {
        let xml = br#"<result><id type="integer">7</id><title>Seven</title></result>"#;
        let record = normalize_xml_record_response(xml).unwrap();
        assert_eq!(record.id, "7");
        assert_eq!(record.title, "Seven");
    }

    #[test]
    fn normalize_xml_response_rejects_doctype_entities() {
        let xml = br#"<!DOCTYPE search [<!ENTITY secret "hidden">]><search><results/></search>"#;
        let error = normalize_xml_search_response(xml).unwrap_err();
        assert!(error
            .to_string()
            .contains("unsupported entity declarations"));
    }

    #[test]
    fn normalize_rss_fixture_accepts_standard_items() {
        let response = normalize_rss_search_response(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/digitalnz-search.rss"
        )))
        .unwrap();

        assert_eq!(response.search.result_count, 1);
        assert_eq!(response.search.results[0].id, "41278482");
        assert_eq!(
            response.search.results[0].source_url.as_deref(),
            Some("https://digitalnz.org/records/41278482")
        );
        assert_eq!(
            response.search.results[0].category,
            Some(vec!["Images".to_string(), "Photographs".to_string()])
        );
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
