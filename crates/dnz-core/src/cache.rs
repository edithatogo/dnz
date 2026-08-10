//! Persistent query cache backed by SQLite.

use crate::models::SearchResponse;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA_VERSION: i64 = 2;

/// Provenance recorded alongside a cached response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheProvenance {
    pub source_url: String,
    pub auth_namespace: String,
}

/// A cached response with its freshness and request provenance.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub response: SearchResponse,
    pub provenance: CacheProvenance,
    pub created_at: i64,
}

/// SQLite-backed cache for DigitalNZ search responses.
#[derive(Debug, Clone)]
pub struct PersistentCache {
    path: PathBuf,
}

impl PersistentCache {
    /// Open or create a persistent cache at `path`.
    pub fn new(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let cache = Self { path: path.into() };
        cache.initialize()?;
        Ok(cache)
    }

    /// Return the backing SQLite file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Create the cache schema if it is absent.
    pub fn initialize(&self) -> anyhow::Result<()> {
        let conn = self.open()?;
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            CREATE TABLE IF NOT EXISTS cache_metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS search_cache (
                cache_key TEXT PRIMARY KEY,
                response_json TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                source_url TEXT NOT NULL DEFAULT '',
                auth_namespace TEXT NOT NULL DEFAULT ''
            );
            "#,
        )?;
        for (column, definition) in [
            ("source_url", "TEXT NOT NULL DEFAULT ''"),
            ("auth_namespace", "TEXT NOT NULL DEFAULT ''"),
        ] {
            let exists: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM pragma_table_info('search_cache') WHERE name = ?1)",
                params![column],
                |row| row.get(0),
            )?;
            if !exists {
                conn.execute(
                    &format!("ALTER TABLE search_cache ADD COLUMN {column} {definition}"),
                    [],
                )?;
            }
        }
        conn.execute(
            "INSERT INTO cache_metadata(key, value) VALUES ('schema_version', ?1)\
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![SCHEMA_VERSION.to_string()],
        )?;
        Ok(())
    }

    /// Read a cached search response.
    pub fn get(&self, cache_key: &str) -> anyhow::Result<Option<SearchResponse>> {
        self.get_with_max_age(cache_key, None)
    }

    /// Read a cached response, optionally rejecting entries older than `max_age`.
    pub fn get_with_max_age(
        &self,
        cache_key: &str,
        max_age: Option<std::time::Duration>,
    ) -> anyhow::Result<Option<SearchResponse>> {
        Ok(self
            .get_entry_with_max_age(cache_key, max_age)?
            .map(|entry| entry.response))
    }

    /// Read a cache entry with optional freshness validation and provenance.
    pub fn get_entry_with_max_age(
        &self,
        cache_key: &str,
        max_age: Option<std::time::Duration>,
    ) -> anyhow::Result<Option<CacheEntry>> {
        let conn = self.open()?;
        let cached = conn
            .query_row(
                "SELECT response_json, created_at, source_url, auth_namespace
                 FROM search_cache WHERE cache_key = ?1",
                params![cache_key],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                    ))
                },
            )
            .optional()?;

        let Some((response_json, created_at, source_url, auth_namespace)) = cached else {
            return Ok(None);
        };
        if let Some(max_age) = max_age {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
            if max_age.is_zero() || now.saturating_sub(created_at) > max_age.as_secs() as i64 {
                return Ok(None);
            }
        }
        Ok(Some(CacheEntry {
            response: serde_json::from_str(&response_json)?,
            provenance: CacheProvenance {
                source_url,
                auth_namespace,
            },
            created_at,
        }))
    }

    /// Store a search response.
    pub fn put(&self, cache_key: &str, response: &SearchResponse) -> anyhow::Result<()> {
        self.put_with_provenance(
            cache_key,
            response,
            &CacheProvenance {
                source_url: String::new(),
                auth_namespace: String::new(),
            },
        )
    }

    /// Store a search response and the source/auth namespace used to obtain it.
    pub fn put_with_provenance(
        &self,
        cache_key: &str,
        response: &SearchResponse,
        provenance: &CacheProvenance,
    ) -> anyhow::Result<()> {
        let conn = self.open()?;
        let response_json = serde_json::to_string(response)?;
        let created_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
        conn.execute(
            r#"
            INSERT INTO search_cache(cache_key, response_json, created_at, source_url, auth_namespace)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(cache_key) DO UPDATE
            SET response_json = excluded.response_json,
                created_at = excluded.created_at,
                source_url = excluded.source_url,
                auth_namespace = excluded.auth_namespace
            "#,
            params![
                cache_key,
                response_json,
                created_at,
                provenance.source_url,
                provenance.auth_namespace
            ],
        )?;
        Ok(())
    }

    /// Retain at most `max_entries`, evicting the oldest entries first.
    pub fn prune_to_limit(&self, max_entries: usize) -> anyhow::Result<()> {
        let conn = self.open()?;
        let count: usize = conn.query_row("SELECT COUNT(*) FROM search_cache", [], |row| {
            row.get::<_, i64>(0).map(|value| value.max(0) as usize)
        })?;
        if count <= max_entries {
            return Ok(());
        }
        let remove = count - max_entries;
        let mut statement = conn.prepare(
            "SELECT cache_key FROM search_cache ORDER BY created_at ASC, cache_key ASC LIMIT ?1",
        )?;
        let keys: Vec<String> = statement
            .query_map(params![remove as i64], |row| row.get(0))?
            .collect::<Result<_, _>>()?;
        drop(statement);
        for key in keys {
            conn.execute(
                "DELETE FROM search_cache WHERE cache_key = ?1",
                params![key],
            )?;
        }
        Ok(())
    }

    /// Delete all cached responses while preserving schema metadata.
    pub fn clear(&self) -> anyhow::Result<()> {
        let conn = self.open()?;
        conn.execute("DELETE FROM search_cache", [])?;
        Ok(())
    }

    /// Return the current schema version stored in SQLite metadata.
    pub fn schema_version(&self) -> anyhow::Result<i64> {
        let conn = self.open()?;
        let version = conn.query_row(
            "SELECT value FROM cache_metadata WHERE key = 'schema_version'",
            [],
            |row| row.get::<_, String>(0),
        )?;
        Ok(version.parse().unwrap_or(SCHEMA_VERSION))
    }

    fn open(&self) -> anyhow::Result<Connection> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(Connection::open(&self.path)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Record, SearchMetadata};

    fn temp_cache_path(name: &str) -> PathBuf {
        let unique = format!(
            "dnz-cache-{name}-{}-{}.sqlite",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock should be after unix epoch")
                .as_nanos()
        );
        std::env::temp_dir().join(unique)
    }

    fn sample_response(title: &str) -> SearchResponse {
        SearchResponse {
            search: SearchMetadata {
                result_count: 1,
                page: None,
                per_page: None,
                results: vec![Record {
                    id: "rec-1".to_string(),
                    title: title.to_string(),
                    ..Record::default()
                }],
                facets: Default::default(),
                request: None,
            },
        }
    }

    #[test]
    fn persistent_cache_initializes_schema() {
        let path = temp_cache_path("schema");
        let cache = PersistentCache::new(&path).expect("cache should initialize");

        assert_eq!(cache.schema_version().expect("schema version"), 2);
        assert_eq!(cache.path(), path.as_path());

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn persistent_cache_round_trips_search_response() {
        let path = temp_cache_path("roundtrip");
        let cache = PersistentCache::new(&path).expect("cache should initialize");

        cache
            .put("query-key", &sample_response("Kauri"))
            .expect("put should succeed");

        let cached = cache
            .get("query-key")
            .expect("get should succeed")
            .expect("response should be cached");

        assert_eq!(cached.search.results[0].title, "Kauri");

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn persistent_cache_clear_removes_responses() {
        let path = temp_cache_path("clear");
        let cache = PersistentCache::new(&path).expect("cache should initialize");
        cache
            .put("query-key", &sample_response("Kauri"))
            .expect("put should succeed");

        cache.clear().expect("clear should succeed");

        assert!(cache
            .get("query-key")
            .expect("get should succeed")
            .is_none());

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn persistent_cache_ttl_rejects_expired_entries() {
        let path = temp_cache_path("ttl");
        let cache = PersistentCache::new(&path).expect("cache should initialize");
        cache
            .put("query-key", &sample_response("Kauri"))
            .expect("put should succeed");

        assert!(cache
            .get_with_max_age("query-key", Some(std::time::Duration::ZERO))
            .expect("get should succeed")
            .is_none());

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn persistent_cache_records_provenance_and_prunes_oldest_entries() {
        let path = temp_cache_path("provenance");
        let cache = PersistentCache::new(&path).expect("cache should initialize");
        let provenance = CacheProvenance {
            source_url: "https://api.digitalnz.org/v3/records.json".into(),
            auth_namespace: "public".into(),
        };
        cache
            .put_with_provenance("a", &sample_response("A"), &provenance)
            .expect("put should succeed");
        cache
            .put_with_provenance("b", &sample_response("B"), &provenance)
            .expect("put should succeed");

        let entry = cache
            .get_entry_with_max_age("b", None)
            .expect("get should succeed")
            .expect("entry should exist");
        assert_eq!(entry.provenance, provenance);
        cache.prune_to_limit(1).expect("prune should succeed");
        assert!(cache.get("a").expect("get should succeed").is_none());
        assert!(cache.get("b").expect("get should succeed").is_some());

        let _ = std::fs::remove_file(path);
    }
}
