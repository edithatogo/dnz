//! Export routines generating open-science Frictionless and JSON-LD formats.

use crate::models::Record;
use serde_json::json;

/// Generate a Frictionless Data Package descriptor (datapackage.json) for record sets.
pub fn generate_frictionless_datapackage(
    records: &[Record],
    package_name: &str,
) -> serde_json::Value {
    json!({
        "profile": "tabular-data-package",
        "name": package_name,
        "title": "DigitalNZ Harvest Dataset",
        "description": "Tabular dataset containing cultural heritage metadata records harvested from the DigitalNZ API.",
        "resources": [
            {
                "name": "records",
                "path": "records.csv",
                "profile": "tabular-data-resource",
                "schema": {
                    "fields": [
                        { "name": "id", "type": "string", "description": "Unique record identifier" },
                        { "name": "title", "type": "string", "description": "Title of the item" },
                        { "name": "description", "type": "string", "description": "Item description" },
                        { "name": "content_partner", "type": "string", "description": "Contributing institutions" },
                        { "name": "category", "type": "string", "description": "Media categories" }
                    ]
                },
                "record_count": records.len()
            }
        ]
    })
}

/// Compile a schema.org JSON-LD descriptor for academic search indexes.
pub fn generate_schema_ld(records: &[Record], base_uri: &str) -> serde_json::Value {
    json!({
        "@context": "https://schema.org",
        "@type": "Dataset",
        "name": "DigitalNZ Harvest Collection",
        "description": "Harvested archives representing digital heritage collections from libraries and museums in New Zealand.",
        "url": base_uri,
        "license": "https://creativecommons.org/publicdomain/zero/1.0/",
        "distribution": [
            {
                "@type": "DataDownload",
                "encodingFormat": "text/csv",
                "contentUrl": format!("{}/records.csv", base_uri)
            }
        ],
        "size": records.len(),
        "temporalCoverage": "1800/2026"
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn records() -> Vec<Record> {
        vec![Record {
            id: "1".to_string(),
            title: "Kauri".to_string(),
            ..Record::default()
        }]
    }

    #[test]
    fn frictionless_datapackage_declares_records_resource() {
        let package = generate_frictionless_datapackage(&records(), "dnz-test");

        assert_eq!(package["profile"], "tabular-data-package");
        assert_eq!(package["name"], "dnz-test");
        assert_eq!(package["resources"][0]["name"], "records");
        assert_eq!(package["resources"][0]["path"], "records.csv");
        assert_eq!(package["resources"][0]["record_count"], 1);
    }

    #[test]
    fn schema_ld_declares_dataset_distribution() {
        let schema = generate_schema_ld(&records(), "https://example.test/dnz");

        assert_eq!(schema["@type"], "Dataset");
        assert_eq!(schema["url"], "https://example.test/dnz");
        assert_eq!(
            schema["distribution"][0]["contentUrl"],
            "https://example.test/dnz/records.csv"
        );
        assert_eq!(schema["size"], 1);
    }

    #[test]
    fn test_frictionless_empty_records() {
        let package = generate_frictionless_datapackage(&[], "empty-test");

        assert_eq!(package["profile"], "tabular-data-package");
        assert_eq!(package["name"], "empty-test");
        assert_eq!(package["resources"][0]["record_count"], 0);
    }

    #[test]
    fn test_schema_ld_empty_records() {
        let schema = generate_schema_ld(&[], "https://example.test/empty");

        assert_eq!(schema["@type"], "Dataset");
        assert_eq!(schema["size"], 0);
        assert_eq!(schema["url"], "https://example.test/empty");
    }
}
