//! Export routines generating open-science Frictionless and JSON-LD formats.

use crate::models::Record;
use serde_json::json;

/// Generate a Frictionless Data Package descriptor (datapackage.json) for record sets.
pub fn generate_frictionless_datapackage(records: &[Record], package_name: &str) -> serde_json::Value {
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
