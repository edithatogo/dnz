//! Export routines generating open-science Frictionless and JSON-LD formats.

use crate::client::Client;
use crate::models::Record;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const GAZETTE_COLLECTION: &str = "New Zealand Gazette";

/// Deterministic provenance and integrity metadata for an export bundle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExportProvenance {
    pub schema_version: u32,
    pub checksum_algorithm: String,
    pub source_url: String,
    pub record_count: usize,
    pub files: BTreeMap<String, String>,
    pub limitations: Vec<String>,
}

/// Build provenance metadata for already-published export files.
pub fn build_export_provenance(
    source_url: impl Into<String>,
    record_count: usize,
    files: &[PathBuf],
) -> anyhow::Result<ExportProvenance> {
    let mut checksums = BTreeMap::new();
    for path in files {
        let relative = path.to_string_lossy().replace('\\', "/");
        checksums.insert(relative, file_checksum(path)?);
    }
    Ok(ExportProvenance {
        schema_version: 1,
        checksum_algorithm: "fnv1a64".to_string(),
        source_url: source_url.into(),
        record_count,
        files: checksums,
        limitations: vec![
            "fnv1a64 provides deterministic change detection, not cryptographic authenticity."
                .to_string(),
            "Source metadata reflects the supplied endpoint and does not prove provider completeness."
                .to_string(),
        ],
    })
}

/// Atomically write provenance metadata as a JSON descriptor.
pub fn write_export_provenance(
    path: impl AsRef<Path>,
    provenance: &ExportProvenance,
) -> anyhow::Result<()> {
    write_pretty_json(path.as_ref(), provenance)
}

fn file_checksum(path: &Path) -> anyhow::Result<String> {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in fs::read(path)? {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64-{hash:016x}"))
}

/// Write normalized records as deterministic JSONL using an atomic publish.
pub fn write_records_jsonl(path: impl AsRef<Path>, records: &[Record]) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = temporary_path(path);
    let mut writer = BufWriter::new(File::create(&temporary)?);
    for record in records {
        serde_json::to_writer(&mut writer, record)?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    drop(writer);
    atomic_replace(&temporary, path)
}

/// Write a stable, spreadsheet-safe CSV projection of normalized records.
///
/// The projection is intentionally explicit; unknown provider fields remain
/// available through JSONL. Formula-like leading characters are prefixed so
/// common spreadsheet consumers do not interpret metadata as executable data.
pub fn write_records_csv(path: impl AsRef<Path>, records: &[Record]) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = temporary_path(path);
    let mut writer = BufWriter::new(File::create(&temporary)?);
    writer.write_all(b"id,title,description,content_partner,category,source_url,usage\n")?;
    for record in records {
        let fields = [
            record.id.as_str(),
            record.title.as_str(),
            record.description.as_deref().unwrap_or_default(),
            &join_values(record.content_partner.as_deref()),
            &join_values(record.category.as_deref()),
            record.source_url.as_deref().unwrap_or_default(),
            record.usage.as_deref().unwrap_or_default(),
        ];
        for (index, field) in fields.iter().enumerate() {
            if index > 0 {
                writer.write_all(b",")?;
            }
            writer.write_all(csv_field(field).as_bytes())?;
        }
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    drop(writer);
    atomic_replace(&temporary, path)
}

/// Write validated record locations as a GeoJSON FeatureCollection.
///
/// Records without finite WGS84 coordinates are omitted. Provider location
/// payloads vary, so extraction accepts common latitude/longitude key pairs
/// and `[longitude, latitude]` coordinate arrays while enforcing GeoJSON
/// ranges before emission.
pub fn write_records_geojson(path: impl AsRef<Path>, records: &[Record]) -> anyhow::Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    let features: Vec<_> = records
        .iter()
        .filter_map(|record| {
            record
                .locations
                .as_ref()
                .and_then(find_coordinates)
                .map(|(longitude, latitude)| {
                    json!({
                        "type": "Feature",
                        "geometry": {"type": "Point", "coordinates": [longitude, latitude]},
                        "properties": {
                            "id": record.id,
                            "title": record.title,
                            "source_url": record.source_url,
                            "rights": record.rights,
                        }
                    })
                })
        })
        .collect();
    let collection = json!({"type": "FeatureCollection", "features": features});
    write_pretty_json(path.as_ref(), &collection)
}

/// Write a stable SQLite projection of normalized records using an atomic publish.
///
/// JSONL remains the lossless interchange format. This selected relational
/// projection is intended for lightweight local querying and keeps rights and
/// provenance fields explicit without flattening every provider-specific field.
pub fn write_records_sqlite(path: impl AsRef<Path>, records: &[Record]) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = temporary_path(path);
    let result = (|| -> anyhow::Result<()> {
        let connection = Connection::open(&temporary)?;
        connection.execute_batch(
            r#"CREATE TABLE export_metadata (schema_version INTEGER NOT NULL, record_count INTEGER NOT NULL);
               CREATE TABLE records (
                   id TEXT PRIMARY KEY NOT NULL,
                   title TEXT NOT NULL,
                   description TEXT,
                   source_url TEXT,
                   rights TEXT,
                   rights_url TEXT,
                   usage TEXT,
                   is_commercial_use INTEGER
               );"#,
        )?;
        let transaction = connection.unchecked_transaction()?;
        for record in records {
            transaction.execute(
                r#"INSERT INTO records (id, title, description, source_url, rights, rights_url, usage, is_commercial_use)
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"#,
                params![
                    record.id,
                    record.title,
                    record.description,
                    record.source_url,
                    record.rights,
                    record.rights_url,
                    record.usage,
                    record.is_commercial_use.map(i64::from),
                ],
            )?;
        }
        transaction.execute(
            "INSERT INTO export_metadata (schema_version, record_count) VALUES (?1, ?2)",
            params![1_i64, records.len() as i64],
        )?;
        transaction.commit()?;
        connection.execute_batch("VACUUM")?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;
    atomic_replace(&temporary, path)
}

fn find_coordinates(value: &serde_json::Value) -> Option<(f64, f64)> {
    match value {
        serde_json::Value::Object(object) => {
            let latitude = ["latitude", "lat"]
                .iter()
                .find_map(|key| object.get(*key).and_then(serde_json::Value::as_f64));
            let longitude = ["longitude", "lon", "lng"]
                .iter()
                .find_map(|key| object.get(*key).and_then(serde_json::Value::as_f64));
            if let (Some(longitude), Some(latitude)) = (longitude, latitude) {
                if longitude.is_finite()
                    && latitude.is_finite()
                    && (-180.0..=180.0).contains(&longitude)
                    && (-90.0..=90.0).contains(&latitude)
                {
                    return Some((longitude, latitude));
                }
            }
            object.values().find_map(find_coordinates)
        }
        serde_json::Value::Array(values) => {
            if values.len() >= 2 {
                let longitude = values[0].as_f64();
                let latitude = values[1].as_f64();
                if let (Some(longitude), Some(latitude)) = (longitude, latitude) {
                    if longitude.is_finite()
                        && latitude.is_finite()
                        && (-180.0..=180.0).contains(&longitude)
                        && (-90.0..=90.0).contains(&latitude)
                    {
                        return Some((longitude, latitude));
                    }
                }
            }
            values.iter().find_map(find_coordinates)
        }
        _ => None,
    }
}

fn join_values(values: Option<&[String]>) -> String {
    values.map(|values| values.join(" | ")).unwrap_or_default()
}

fn csv_field(value: &str) -> String {
    let safe = match value.chars().next() {
        Some('=') | Some('+') | Some('-') | Some('@') => format!("'{value}"),
        _ => value.to_string(),
    };
    if safe.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", safe.replace('"', "\"\""))
    } else {
        safe
    }
}

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
        "distribution": [
            {
                "@type": "DataDownload",
                "encodingFormat": "text/csv",
                "contentUrl": format!("{}/records.csv", base_uri)
            }
        ],
        "size": records.len(),
    })
}

/// Generate a minimal source-grounded RO-Crate metadata graph.
pub fn generate_ro_crate_metadata(
    records: &[Record],
    base_uri: &str,
    provenance: &ExportProvenance,
) -> serde_json::Value {
    json!({
        "@context": "https://w3id.org/ro/crate/1.1/context",
        "@graph": [
            {
                "@id": "ro-crate-metadata.json",
                "@type": "CreativeWork",
                "about": {"@id": "./"}
            },
            {
                "@id": "./",
                "@type": "Dataset",
                "name": "DigitalNZ export",
                "url": base_uri,
                "hasPart": [{"@id": "records.jsonl"}],
                "source": {"@id": provenance.source_url},
                "sdPublisher": {"@id": "https://digitalnz.org/"},
                "description": "Metadata export; rights and completeness require source-specific review."
            },
            {
                "@id": "records.jsonl",
                "@type": "File",
                "encodingFormat": "application/jsonl",
                "contentSize": records.len()
            }
        ]
    })
}

/// Configuration for a deterministic New Zealand Gazette export.
#[derive(Debug, Clone)]
pub struct GazetteExportConfig {
    pub output_dir: PathBuf,
    pub text: String,
    pub start_page: u32,
    pub max_pages: Option<u32>,
    pub per_page: u32,
    pub sort: Option<String>,
    pub direction: String,
}

impl GazetteExportConfig {
    pub fn new(output_dir: impl Into<PathBuf>) -> Self {
        Self {
            output_dir: output_dir.into(),
            text: String::new(),
            start_page: 1,
            max_pages: None,
            per_page: 100,
            sort: Some("date".to_string()),
            direction: "asc".to_string(),
        }
    }
}

/// Summary written to `manifest.json` for downstream archive validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GazetteExportManifest {
    pub collection: String,
    pub text: String,
    pub start_page: u32,
    pub per_page: u32,
    pub sort: Option<String>,
    pub direction: String,
    pub total_results: u64,
    pub pages_written: u32,
    pub records_written: usize,
    pub completed: bool,
    pub files: GazetteExportFiles,
    pub access: GazetteExportAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GazetteExportFiles {
    pub records_jsonl: String,
    pub manifest_json: String,
    pub raw_pages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GazetteExportAccess {
    pub api_key_required: bool,
    pub anonymous_supported: bool,
    pub api_key_source: String,
    pub note: String,
}

/// Export Gazette pages as raw page JSON plus normalized record JSONL.
pub async fn export_gazette(
    client: &Client,
    config: GazetteExportConfig,
) -> anyhow::Result<GazetteExportManifest> {
    if config.start_page == 0 {
        anyhow::bail!("start_page must be 1 or greater");
    }
    if config.per_page == 0 {
        anyhow::bail!("per_page must be 1 or greater");
    }

    fs::create_dir_all(&config.output_dir)?;
    let pages_dir = config.output_dir.join("pages");
    fs::create_dir_all(&pages_dir)?;

    let records_path = config.output_dir.join("records.jsonl");
    let records_temp_path = temporary_path(&records_path);
    let manifest_path = config.output_dir.join("manifest.json");
    let mut records_writer = BufWriter::new(File::create(&records_temp_path)?);

    let mut current_page = config.start_page;
    let mut total_results = 0_u64;
    let mut pages_written = 0_u32;
    let mut records_written = 0_usize;
    let mut raw_pages = Vec::new();
    let mut completed = false;

    loop {
        if let Some(max_pages) = config.max_pages {
            if pages_written >= max_pages {
                break;
            }
        }

        let mut query = client
            .search(&config.text)
            .page(current_page)
            .per_page(config.per_page)
            .and_filter("primary_collection", vec![GAZETTE_COLLECTION.to_string()]);

        if let Some(sort) = &config.sort {
            query = query.sort(sort, &config.direction);
        }

        let response = query.send().await?;
        if pages_written == 0 {
            total_results = response.search.result_count;
        }

        let raw_page_path = pages_dir.join(format!("page-{current_page:06}.json"));
        write_pretty_json(&raw_page_path, &response)?;
        raw_pages.push(relative_path(&config.output_dir, &raw_page_path));
        pages_written += 1;

        for record in &response.search.results {
            serde_json::to_writer(&mut records_writer, record)?;
            records_writer.write_all(b"\n")?;
            records_written += 1;
        }

        let page_record_count = response.search.results.len() as u64;
        let fetched_results =
            (current_page - config.start_page) as u64 * config.per_page as u64 + page_record_count;
        if page_record_count == 0 || fetched_results >= total_results {
            completed = true;
            break;
        }

        current_page += 1;
    }
    records_writer.flush()?;
    drop(records_writer);
    atomic_replace(&records_temp_path, &records_path)?;

    let manifest = GazetteExportManifest {
        collection: GAZETTE_COLLECTION.to_string(),
        text: config.text,
        start_page: config.start_page,
        per_page: config.per_page.clamp(1, 100),
        sort: config.sort,
        direction: config.direction,
        total_results,
        pages_written,
        records_written,
        completed,
        files: GazetteExportFiles {
            records_jsonl: "records.jsonl".to_string(),
            manifest_json: "manifest.json".to_string(),
            raw_pages,
        },
        access: GazetteExportAccess {
            api_key_required: true,
            anonymous_supported: false,
            api_key_source: "DIGITALNZ_API_KEY or --api-key".to_string(),
            note: "dnz export commands use authenticated DigitalNZ API requests; API keys are never written to export artifacts.".to_string(),
        },
    };

    validate_manifest_files(&config.output_dir, &manifest)?;
    write_pretty_json(&manifest_path, &manifest)?;
    Ok(manifest)
}

fn write_pretty_json(path: &Path, value: &impl Serialize) -> anyhow::Result<()> {
    let temporary = temporary_path(path);
    let file = File::create(&temporary)?;
    serde_json::to_writer_pretty(&file, value)?;
    file.sync_all()?;
    drop(file);
    atomic_replace(&temporary, path)?;
    Ok(())
}

fn temporary_path(path: &Path) -> PathBuf {
    path.with_extension(format!(
        "{}tmp",
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| format!("{extension}."))
            .unwrap_or_default()
    ))
}

fn atomic_replace(temporary: &Path, destination: &Path) -> anyhow::Result<()> {
    if std::fs::rename(temporary, destination).is_ok() {
        return Ok(());
    }

    if destination.exists() {
        std::fs::remove_file(destination)?;
        std::fs::rename(temporary, destination)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "failed to publish export file {}",
            destination.display()
        ))
    }
}

fn validate_manifest_files(root: &Path, manifest: &GazetteExportManifest) -> anyhow::Result<()> {
    if manifest.files.raw_pages.len() != manifest.pages_written as usize {
        anyhow::bail!("manifest page count does not match raw page list");
    }

    for relative in std::iter::once(manifest.files.records_jsonl.as_str())
        .chain(manifest.files.raw_pages.iter().map(String::as_str))
    {
        let path = root.join(relative);
        if !path.starts_with(root) || !path.is_file() {
            anyhow::bail!("manifest references missing export file {relative}");
        }
    }
    Ok(())
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
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
    fn records_jsonl_is_deterministic_and_atomic() {
        let output = std::env::temp_dir().join(format!(
            "dnz-records-{}-{}.jsonl",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        write_records_jsonl(&output, &records()).unwrap();
        let first = std::fs::read_to_string(&output).unwrap();
        write_records_jsonl(&output, &records()).unwrap();
        assert_eq!(first, std::fs::read_to_string(&output).unwrap());
        assert!(!output.with_extension("jsonl.tmp").exists());
        let _ = std::fs::remove_file(output);
    }

    #[test]
    fn records_csv_escapes_fields_and_blocks_formula_values() {
        let output = std::env::temp_dir().join(format!(
            "dnz-records-{}-{}.csv",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let records = vec![Record {
            id: "=unsafe".to_string(),
            title: "Title, with \"quotes\"".to_string(),
            ..Record::default()
        }];
        write_records_csv(&output, &records).unwrap();
        let csv = std::fs::read_to_string(&output).unwrap();
        assert!(csv.contains("'=unsafe"));
        assert!(csv.contains("\"Title, with \"\"quotes\"\"\""));
        assert!(!output.with_extension("csv.tmp").exists());
        let _ = std::fs::remove_file(output);
    }

    #[test]
    fn sqlite_export_is_queryable_and_records_schema_metadata() {
        let output = std::env::temp_dir().join(format!(
            "dnz-export-{}.sqlite",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        write_records_sqlite(&output, &records()).unwrap();
        let connection = Connection::open(&output).unwrap();
        let record_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM records", [], |row| row.get(0))
            .unwrap();
        let schema_version: i64 = connection
            .query_row("SELECT schema_version FROM export_metadata", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(record_count, 1);
        assert_eq!(schema_version, 1);
        let _ = std::fs::remove_file(output);
    }

    #[test]
    fn geojson_emits_only_valid_coordinates() {
        let output = std::env::temp_dir().join(format!(
            "dnz-records-{}-{}.geojson",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut valid = Record {
            id: "1".into(),
            title: "Valid".into(),
            locations: Some(json!({"latitude": -36.85, "longitude": 174.76})),
            ..Record::default()
        };
        let invalid = Record {
            id: "2".into(),
            title: "Invalid".into(),
            locations: Some(json!({"latitude": 95.0, "longitude": 174.76})),
            ..Record::default()
        };
        write_records_geojson(&output, &[valid.clone(), invalid]).unwrap();
        let value: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&output).unwrap()).unwrap();
        assert_eq!(value["features"].as_array().unwrap().len(), 1);
        assert_eq!(value["features"][0]["properties"]["id"], "1");
        valid.locations = Some(json!({"coordinates": [174.76, -36.85]}));
        write_records_geojson(&output, &[valid]).unwrap();
        assert!(std::fs::read_to_string(&output).unwrap().contains("174.76"));
        let _ = std::fs::remove_file(output);
    }

    #[test]
    fn provenance_is_deterministic_and_discloses_checksum_limits() {
        let file = std::env::temp_dir().join(format!(
            "dnz-provenance-input-{}-{}.txt",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(&file, b"stable export").unwrap();
        let provenance = build_export_provenance(
            "https://api.digitalnz.org/v3/records.json",
            2,
            std::slice::from_ref(&file),
        )
        .unwrap();
        assert_eq!(provenance.checksum_algorithm, "fnv1a64");
        assert_eq!(provenance.files.len(), 1);
        assert!(provenance
            .limitations
            .iter()
            .any(|item| item.contains("not cryptographic")));
        let _ = std::fs::remove_file(file);
    }

    #[test]
    fn ro_crate_metadata_is_source_grounded() {
        let provenance = ExportProvenance {
            schema_version: 1,
            checksum_algorithm: "fnv1a64".into(),
            source_url: "https://api.digitalnz.org/v3/records.json".into(),
            record_count: 1,
            files: BTreeMap::new(),
            limitations: vec!["metadata only".into()],
        };
        let crate_metadata =
            generate_ro_crate_metadata(&records(), "https://example.test/export", &provenance);
        assert_eq!(
            crate_metadata["@graph"][1]["source"]["@id"],
            provenance.source_url
        );
        assert!(crate_metadata["@graph"][1]["description"]
            .as_str()
            .unwrap()
            .contains("rights and completeness"));
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
        assert!(schema.get("license").is_none());
        assert!(schema.get("temporalCoverage").is_none());
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

    #[test]
    fn manifest_reconciliation_rejects_missing_files() {
        let root = std::env::temp_dir().join(format!(
            "dnz-export-reconcile-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be after unix epoch")
                .as_nanos()
        ));
        std::fs::create_dir_all(root.join("pages")).expect("export root");
        std::fs::write(root.join("records.jsonl"), b"{}").expect("records file");

        let manifest = GazetteExportManifest {
            collection: GAZETTE_COLLECTION.to_string(),
            text: String::new(),
            start_page: 1,
            per_page: 1,
            sort: None,
            direction: "asc".to_string(),
            total_results: 1,
            pages_written: 1,
            records_written: 1,
            completed: true,
            files: GazetteExportFiles {
                records_jsonl: "records.jsonl".to_string(),
                manifest_json: "manifest.json".to_string(),
                raw_pages: vec!["pages/page-000001.json".to_string()],
            },
            access: GazetteExportAccess {
                api_key_required: false,
                anonymous_supported: true,
                api_key_source: "none".to_string(),
                note: "test".to_string(),
            },
        };

        let error = validate_manifest_files(&root, &manifest).unwrap_err();
        assert!(error.to_string().contains("missing export file"));
        let _ = std::fs::remove_dir_all(root);
    }

    #[tokio::test]
    async fn gazette_export_writes_pages_records_and_manifest() {
        use wiremock::matchers::{method, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;
        let output_dir = std::env::temp_dir().join(format!(
            "dnz-gazette-export-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be after unix epoch")
                .as_nanos()
        ));

        Mock::given(method("GET"))
            .and(query_param("text", ""))
            .and(query_param("page", "1"))
            .and(query_param("per_page", "2"))
            .and(query_param("and[primary_collection][]", GAZETTE_COLLECTION))
            .and(query_param("sort", "date"))
            .and(query_param("direction", "asc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "search": {
                    "result_count": 3,
                    "results": [
                        { "id": "gaz-1", "title": "Gazette 1", "license": "CC-BY" },
                        { "id": "gaz-2", "title": "Gazette 2", "usage": "open" }
                    ]
                }
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(query_param("text", ""))
            .and(query_param("page", "2"))
            .and(query_param("per_page", "2"))
            .and(query_param("and[primary_collection][]", GAZETTE_COLLECTION))
            .and(query_param("sort", "date"))
            .and(query_param("direction", "asc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "search": {
                    "result_count": 3,
                    "results": [
                        { "id": "gaz-3", "title": "Gazette 3" }
                    ]
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new("key").with_base_url(mock_server.uri());
        let mut config = GazetteExportConfig::new(&output_dir);
        config.per_page = 2;

        let manifest = export_gazette(&client, config).await.unwrap();

        assert_eq!(manifest.collection, GAZETTE_COLLECTION);
        assert_eq!(manifest.total_results, 3);
        assert_eq!(manifest.pages_written, 2);
        assert_eq!(manifest.records_written, 3);
        assert!(manifest.completed);
        assert_eq!(
            manifest.files.raw_pages,
            vec!["pages/page-000001.json", "pages/page-000002.json"]
        );
        assert!(output_dir.join("pages/page-000001.json").is_file());
        assert!(!output_dir.join("records.jsonl.tmp").exists());
        assert!(!output_dir.join("manifest.json.tmp").exists());
        assert!(!output_dir.join("pages/page-000001.json.tmp").exists());

        let jsonl = std::fs::read_to_string(output_dir.join("records.jsonl")).unwrap();
        let lines: Vec<&str> = jsonl.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains("\"license\":\"CC-BY\""));

        let written_manifest: GazetteExportManifest = serde_json::from_str(
            &std::fs::read_to_string(output_dir.join("manifest.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(written_manifest, manifest);

        let _ = std::fs::remove_dir_all(output_dir);
    }

    #[tokio::test]
    async fn gazette_export_can_stop_after_max_pages() {
        use wiremock::matchers::{method, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;
        let output_dir = std::env::temp_dir().join(format!(
            "dnz-gazette-export-max-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be after unix epoch")
                .as_nanos()
        ));

        Mock::given(method("GET"))
            .and(query_param("page", "5"))
            .and(query_param("and[primary_collection][]", GAZETTE_COLLECTION))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "search": {
                    "result_count": 10,
                    "results": [
                        { "id": "gaz-5", "title": "Gazette 5" }
                    ]
                }
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = Client::new("key").with_base_url(mock_server.uri());
        let mut config = GazetteExportConfig::new(&output_dir);
        config.start_page = 5;
        config.max_pages = Some(1);
        config.per_page = 1;

        let manifest = export_gazette(&client, config).await.unwrap();

        assert_eq!(manifest.start_page, 5);
        assert_eq!(manifest.pages_written, 1);
        assert_eq!(manifest.records_written, 1);
        assert!(!manifest.completed);

        let _ = std::fs::remove_dir_all(output_dir);
    }
}
