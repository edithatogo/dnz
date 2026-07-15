//! Deterministic evidence packs with explicit search provenance.

use crate::models::Record;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Identifies where a similarity result came from; API MLT is not local vector search.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SearchProvenance {
    /// DigitalNZ computed the related-record result server-side.
    DigitalNzMoreLikeThis { endpoint: String, record_id: String },
    /// A local index computed similarity from an explicitly named model.
    LocalVector {
        model: String,
        embedding_dimension: usize,
        index: String,
    },
    /// A result combines named provider and local components.
    Hybrid { components: Vec<SearchProvenance> },
}

/// One source-grounded item in an evidence pack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceItem {
    pub record_id: String,
    pub title: String,
    pub citation_key: String,
    pub source_url: Option<String>,
    pub rights: Option<String>,
    pub rights_url: Option<String>,
}

/// A deterministic, reviewable evidence bundle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidencePack {
    pub schema_version: u32,
    pub query: String,
    pub provenance: SearchProvenance,
    pub items: Vec<EvidenceItem>,
    pub limitations: Vec<String>,
}

/// Build an evidence pack without fetching sources or downloading models.
pub fn build_evidence_pack(
    query: impl Into<String>,
    records: &[Record],
    provenance: SearchProvenance,
) -> EvidencePack {
    let items = records
        .iter()
        .map(|record| EvidenceItem {
            record_id: record.id.clone(),
            title: record.title.clone(),
            citation_key: citation_key(&record.id),
            source_url: record
                .source_url
                .clone()
                .or_else(|| record.landing_url.clone()),
            rights: record.rights.clone().or_else(|| record.usage.clone()),
            rights_url: record.rights_url.clone(),
        })
        .collect();
    EvidencePack {
        schema_version: 1,
        query: query.into(),
        provenance,
        items,
        limitations: vec![
            "This pack preserves metadata links and does not capture or reproduce source content."
                .to_string(),
            "Rights fields are provider metadata and require source-specific review; they are not legal advice."
                .to_string(),
        ],
    }
}

/// Write a deterministic JSON evidence pack atomically.
pub fn write_evidence_pack(path: impl AsRef<Path>, pack: &EvidencePack) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension(format!(
        "{}.tmp",
        path.extension()
            .and_then(|value| value.to_str())
            .unwrap_or("json")
    ));
    let bytes = serde_json::to_vec_pretty(pack)?;
    fs::write(&temporary, bytes)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(temporary, path)?;
    Ok(())
}

fn citation_key(id: &str) -> String {
    let mut key = String::from("dnz-");
    for character in id.chars() {
        if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
            key.push(character.to_ascii_lowercase());
        } else {
            key.push('-');
        }
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_pack_preserves_source_and_distinguishes_api_mlt() {
        let record = Record {
            id: "A/1".into(),
            title: "A record".into(),
            source_url: Some("https://example.test/source".into()),
            rights: Some("CC BY".into()),
            ..Record::default()
        };
        let pack = build_evidence_pack(
            "historic map",
            &[record],
            SearchProvenance::DigitalNzMoreLikeThis {
                endpoint: "https://api.digitalnz.org/v3/records/more_like_this.json".into(),
                record_id: "A/1".into(),
            },
        );
        assert_eq!(pack.items[0].citation_key, "dnz-a-1");
        assert_eq!(
            pack.items[0].source_url.as_deref(),
            Some("https://example.test/source")
        );
        assert!(matches!(
            pack.provenance,
            SearchProvenance::DigitalNzMoreLikeThis { .. }
        ));
        assert!(pack
            .limitations
            .iter()
            .any(|item| item.contains("not legal advice")));
    }
}
