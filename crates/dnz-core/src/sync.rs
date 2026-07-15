//! Deterministic, record-level incremental sync manifests.

use crate::models::Record;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

const MANIFEST_SCHEMA_VERSION: u32 = 1;

/// A stable identity/fingerprint pair for one normalized record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SyncRecord {
    pub id: String,
    pub fingerprint: String,
}

/// A deterministic summary of one incremental synchronization input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncrementalSyncManifest {
    pub schema_version: u32,
    pub source_url: String,
    pub records: Vec<SyncRecord>,
    pub added: u64,
    pub updated: u64,
    pub removed: u64,
    pub limitations: Vec<String>,
}

/// Compare normalized records with an optional prior manifest.
///
/// Deletions are reported only when a prior manifest is supplied; the API does
/// not expose a complete provider-side deletion feed.
pub fn build_incremental_sync_manifest(
    source_url: impl Into<String>,
    records: &[Record],
    previous: Option<&IncrementalSyncManifest>,
) -> IncrementalSyncManifest {
    let mut current = BTreeMap::new();
    for record in records {
        current.insert(record.id.clone(), record_fingerprint(record));
    }

    let mut prior = BTreeMap::new();
    if let Some(previous) = previous {
        for record in &previous.records {
            prior.insert(record.id.clone(), record.fingerprint.clone());
        }
    }

    let added = current.keys().filter(|id| !prior.contains_key(*id)).count() as u64;
    let updated = current
        .iter()
        .filter(|(id, fingerprint)| prior.get(*id).is_some_and(|old| old != *fingerprint))
        .count() as u64;
    let removed = prior.keys().filter(|id| !current.contains_key(*id)).count() as u64;
    let records = current
        .into_iter()
        .map(|(id, fingerprint)| SyncRecord { id, fingerprint })
        .collect();

    let mut limitations =
        vec!["Changes are determined from normalized record fingerprints.".to_string()];
    if previous.is_none() {
        limitations
            .push("Removed records cannot be identified without a prior manifest.".to_string());
    } else {
        limitations.push(
            "Removed records reflect only the supplied previous manifest, not a provider deletion feed."
                .to_string(),
        );
    }

    IncrementalSyncManifest {
        schema_version: MANIFEST_SCHEMA_VERSION,
        source_url: source_url.into(),
        records,
        added,
        updated,
        removed,
        limitations,
    }
}

/// Serialize a manifest with stable field and record ordering.
pub fn render_incremental_sync_manifest(
    manifest: &IncrementalSyncManifest,
) -> anyhow::Result<String> {
    Ok(serde_json::to_string_pretty(manifest)? + "\n")
}

/// Atomically write a deterministic manifest beside its destination.
pub fn write_incremental_sync_manifest(
    path: impl AsRef<Path>,
    manifest: &IncrementalSyncManifest,
) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("json.tmp");
    std::fs::write(&temporary, render_incremental_sync_manifest(manifest)?)?;
    std::fs::rename(temporary, path)?;
    Ok(())
}

fn record_fingerprint(record: &Record) -> String {
    let bytes = serde_json::to_vec(record).expect("Record serialization is infallible");
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64-{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(id: &str, title: &str) -> Record {
        Record {
            id: id.to_string(),
            title: title.to_string(),
            ..Record::default()
        }
    }

    #[test]
    fn manifests_are_sorted_and_report_changes() {
        let first = build_incremental_sync_manifest(
            "https://api.digitalnz.org/v3/records.json",
            &[record("b", "B"), record("a", "A")],
            None,
        );
        assert_eq!(first.records[0].id, "a");
        assert_eq!(first.added, 2);
        assert_eq!(first.removed, 0);

        let second = build_incremental_sync_manifest(
            "https://api.digitalnz.org/v3/records.json",
            &[record("a", "A updated"), record("c", "C")],
            Some(&first),
        );
        assert_eq!((second.added, second.updated, second.removed), (1, 1, 1));
        assert!(second
            .limitations
            .iter()
            .any(|limitation| limitation.contains("provider deletion feed")));
    }

    #[test]
    fn rendering_is_repeatable_and_atomic() {
        let manifest = build_incremental_sync_manifest("source", &[record("1", "One")], None);
        let first = render_incremental_sync_manifest(&manifest).unwrap();
        let second = render_incremental_sync_manifest(&manifest).unwrap();
        assert_eq!(first, second);

        let path = std::env::temp_dir().join(format!(
            "dnz-sync-manifest-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        write_incremental_sync_manifest(&path, &manifest).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), first);
        assert!(!path.with_extension("json.tmp").exists());
        let _ = std::fs::remove_file(path);
    }
}
