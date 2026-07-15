//! Client and query builder implementations for the DigitalNZ API.

use crate::cache::{CacheProvenance, PersistentCache};
use crate::errors::DnzError;
use crate::models::{
    normalize_record_response, normalize_rss_record_response, normalize_rss_search_response,
    normalize_search_response, normalize_xml_record_response, normalize_xml_search_response,
    Record, SearchResponse,
};
use reqwest::Client as HttpClient;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{debug, warn};

/// Main client struct interacting with DigitalNZ endpoints.
#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    legacy_query_key_auth: bool,
    base_url: String,
    http_client: HttpClient,
    // Thread-safe query cache
    cache: Arc<Mutex<HashMap<String, SearchResponse>>>,
    persistent_cache: Option<PersistentCache>,
    cache_ttl: Option<Duration>,
    cache_max_entries: Option<usize>,
    offline: bool,
}

impl Client {
    /// Create a new Client.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            legacy_query_key_auth: false,
            base_url: "https://api.digitalnz.org/v3/records.json".to_string(),
            http_client: HttpClient::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("default HTTP client configuration is valid"),
            cache: Arc::new(Mutex::new(HashMap::new())),
            persistent_cache: None,
            cache_ttl: None,
            cache_max_entries: None,
            offline: false,
        }
    }

    /// Provide a custom API endpoint (useful for mock tests).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Override the default request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> anyhow::Result<Self> {
        self.http_client = HttpClient::builder().timeout(timeout).build()?;
        Ok(self)
    }

    /// Enable the legacy `api_key` query parameter authentication mode.
    ///
    /// Tokens otherwise use the `Authentication-Token` header and are never
    /// placed in URLs or request previews.
    pub fn with_legacy_query_key_auth(mut self) -> Self {
        self.legacy_query_key_auth = true;
        self
    }

    /// Construct an unauthenticated client for DigitalNZ's public endpoints.
    pub fn unauthenticated() -> Self {
        Self::new("")
    }

    fn auth_cache_namespace(&self) -> String {
        if self.api_key.is_empty() {
            return "public".to_string();
        }
        let mut hasher = DefaultHasher::new();
        self.api_key.hash(&mut hasher);
        let mode = if self.legacy_query_key_auth {
            "legacy-query"
        } else {
            "header"
        };
        format!("{mode}-{:016x}", hasher.finish())
    }

    /// Enable SQLite-backed cache storage for responses across sessions.
    pub fn with_cache_path(mut self, cache_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        self.persistent_cache = Some(PersistentCache::new(cache_path)?);
        Ok(self)
    }

    /// Reject persistent cache entries older than `ttl`.
    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = Some(ttl);
        self
    }

    /// Bound persistent cache growth by retaining only the newest entries.
    pub fn with_cache_max_entries(mut self, max_entries: usize) -> Self {
        self.cache_max_entries = Some(max_entries);
        self
    }

    /// Prevent network requests; queries must be satisfied by cache entries.
    pub fn offline(mut self) -> Self {
        self.offline = true;
        self
    }

    /// Clear cache entries.
    pub fn clear_cache(&self) {
        if let Ok(mut c) = self.cache.lock() {
            c.clear();
        }
        if let Some(cache) = &self.persistent_cache {
            if let Err(err) = cache.clear() {
                warn!(error = ?err, "Failed to clear persistent cache");
            }
        }
    }

    /// Create a search query builder.
    pub fn search(&self, text: impl Into<String>) -> QueryBuilder {
        QueryBuilder::new(self.clone(), text.into())
    }

    /// Create a bounded, caller-driven stream of search pages.
    ///
    /// Pages are fetched only when [`SearchPageStream::next_page`] is called,
    /// so callers control backpressure and can cancel by dropping the stream.
    pub fn search_pages(&self, text: impl Into<String>) -> SearchPageStream {
        SearchPageStream {
            client: self.clone(),
            text: text.into(),
            next_page: 1,
            per_page: 20,
            max_pages: None,
            finished: false,
        }
    }

    /// Create a lazy record stream backed by bounded page requests.
    pub fn records(&self, text: impl Into<String>) -> RecordStream {
        RecordStream {
            pages: self.search_pages(text),
            current: Vec::new(),
            max_records: None,
            emitted: 0,
        }
    }

    /// Create a record-by-ID metadata builder.
    pub fn record(&self, record_id: impl Into<String>) -> RecordQueryBuilder {
        RecordQueryBuilder {
            client: self.clone(),
            record_id: record_id.into(),
            fields: Vec::new(),
        }
    }

    /// Create a More Like This builder for a record ID.
    pub fn more_like_this(&self, record_id: impl Into<String>) -> MoreLikeThisQueryBuilder {
        MoreLikeThisQueryBuilder {
            client: self.clone(),
            record_id: record_id.into(),
            page: 1,
            per_page: 20,
            fields: Vec::new(),
            filters: Vec::new(),
        }
    }
}

/// Lazy page stream for bounded search harvesting.
#[derive(Debug, Clone)]
pub struct SearchPageStream {
    client: Client,
    text: String,
    next_page: u32,
    per_page: u32,
    max_pages: Option<u32>,
    finished: bool,
}

impl SearchPageStream {
    /// Set the requested page size, clamped to the API maximum.
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = per_page.min(100);
        self
    }

    /// Set a hard upper bound on the number of pages fetched.
    pub fn max_pages(mut self, max_pages: u32) -> Self {
        self.max_pages = Some(max_pages);
        self
    }

    /// Fetch the next page, returning `None` when the configured limit or the
    /// provider's empty page marks the stream complete.
    pub async fn next_page(&mut self) -> anyhow::Result<Option<SearchResponse>> {
        if self.finished || self.max_pages.is_some_and(|limit| self.next_page > limit) {
            self.finished = true;
            return Ok(None);
        }

        let response = self
            .client
            .search(&self.text)
            .page(self.next_page)
            .per_page(self.per_page)
            .send()
            .await?;
        self.next_page = self.next_page.saturating_add(1);
        if response.search.results.is_empty() {
            self.finished = true;
        }
        Ok(Some(response))
    }
}

/// Lazy record stream that applies backpressure at individual record reads.
#[derive(Debug, Clone)]
pub struct RecordStream {
    pages: SearchPageStream,
    current: Vec<Record>,
    max_records: Option<u32>,
    emitted: u32,
}

impl RecordStream {
    /// Set a hard upper bound on records yielded by this stream.
    pub fn max_records(mut self, max_records: u32) -> Self {
        self.max_records = Some(max_records);
        self
    }

    /// Set the page size used by the underlying lazy page stream.
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.pages = self.pages.per_page(per_page);
        self
    }

    /// Fetch and yield one record, returning `None` at the configured limit or end.
    pub async fn next_record(&mut self) -> anyhow::Result<Option<Record>> {
        if self.max_records.is_some_and(|limit| self.emitted >= limit) {
            return Ok(None);
        }
        loop {
            if let Some(record) = self.current.pop() {
                self.emitted = self.emitted.saturating_add(1);
                return Ok(Some(record));
            }
            let Some(page) = self.pages.next_page().await? else {
                return Ok(None);
            };
            self.current = page.search.results;
            self.current.reverse();
        }
    }
}

/// Builder for the DigitalNZ v3 get-metadata endpoint.
#[derive(Debug, Clone)]
pub struct RecordQueryBuilder {
    client: Client,
    record_id: String,
    fields: Vec<String>,
}

impl RecordQueryBuilder {
    /// Restrict the metadata fields returned by the provider.
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
        self
    }

    /// Fetch and normalize one record from a verified response shape.
    pub async fn send(self) -> anyhow::Result<Record> {
        let endpoint = record_endpoint_url(&self.client.base_url, &self.record_id)?;
        let format = response_format(&endpoint)?;
        let mut params = Vec::new();
        if !self.fields.is_empty() {
            params.push(("fields", self.fields.join(",")));
        }

        let mut request = self.client.http_client.get(endpoint).query(&params);
        if !self.client.api_key.is_empty() && !self.client.legacy_query_key_auth {
            request = request.header("Authentication-Token", &self.client.api_key);
        }

        let response = request
            .send()
            .await
            .map_err(|_| anyhow::Error::new(DnzError::Transport))?;
        let status = response.status();
        if !status.is_success() {
            return Err(anyhow::Error::new(DnzError::HttpStatus {
                status: status.as_u16(),
                retry_after: retry_after_delay(response.headers()),
            }));
        }

        let payload = response
            .bytes()
            .await
            .map_err(|err| anyhow::Error::new(DnzError::Decode).context(err))?;
        decode_record_response(format, &payload)
    }
}

/// Builder for the documented More Like This endpoint.
#[derive(Debug, Clone)]
pub struct MoreLikeThisQueryBuilder {
    client: Client,
    record_id: String,
    page: u32,
    per_page: u32,
    fields: Vec<String>,
    filters: Vec<FilterExpr>,
}

impl MoreLikeThisQueryBuilder {
    pub fn page(mut self, page: u32) -> Self {
        self.page = page.max(1);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = per_page.min(100);
        self
    }

    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
        self
    }

    pub fn try_filter(mut self, filter: FilterExpr) -> anyhow::Result<Self> {
        filter.validate()?;
        self.filters.push(filter);
        Ok(self)
    }

    pub async fn send(self) -> anyhow::Result<SearchResponse> {
        let endpoint = more_like_this_endpoint_url(&self.client.base_url, &self.record_id)?;
        let format = response_format(&endpoint)?;
        let mut params = vec![
            ("page", self.page.to_string()),
            ("per_page", self.per_page.to_string()),
        ];
        if !self.fields.is_empty() {
            params.push(("fields", self.fields.join(",")));
        }
        let mut filter_params = Vec::new();
        for filter in &self.filters {
            filter.append_params(&[], &mut filter_params);
        }

        let mut request = self
            .client
            .http_client
            .get(endpoint)
            .query(&params)
            .query(&filter_params);
        if !self.client.api_key.is_empty() && !self.client.legacy_query_key_auth {
            request = request.header("Authentication-Token", &self.client.api_key);
        }
        let response = request
            .send()
            .await
            .map_err(|_| anyhow::Error::new(DnzError::Transport))?;
        let status = response.status();
        if !status.is_success() {
            return Err(anyhow::Error::new(DnzError::HttpStatus {
                status: status.as_u16(),
                retry_after: retry_after_delay(response.headers()),
            }));
        }
        let payload = response
            .bytes()
            .await
            .map_err(|err| anyhow::Error::new(DnzError::Decode).context(err))?;
        decode_search_response(format, &payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use wiremock::matchers::{header, method, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn temp_cache_path(name: &str) -> PathBuf {
        let unique = format!(
            "dnz-client-cache-{name}-{}-{}.sqlite",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock should be after unix epoch")
                .as_nanos()
        );
        std::env::temp_dir().join(unique)
    }

    #[test]
    fn per_page_clamps_to_api_bounds() {
        let client = Client::new("test");

        assert_eq!(client.search("a").per_page(0).per_page, 0);
        assert_eq!(client.search("a").per_page(1).per_page, 1);
        assert_eq!(client.search("a").per_page(100).per_page, 100);
        assert_eq!(client.search("a").per_page(1_000).per_page, 100);
    }

    #[test]
    fn query_contract_validates_bbox_and_protected_extra_parameters() {
        let client = Client::unauthenticated();
        assert!(client
            .search("a")
            .try_geo_bbox(40.0, -10.0, -40.0, 10.0)
            .is_ok());
        assert!(client
            .search("a")
            .try_geo_bbox(100.0, 0.0, 0.0, 0.0)
            .is_err());
        assert!(client
            .search("a")
            .try_extra_param("api_key", "secret")
            .is_err());
        assert!(client.search("a").try_extra_param("format", "json").is_ok());
        assert!(client.search("a").try_extra_param("fields", "id").is_err());
    }

    #[test]
    fn query_builder_stores_structured_filters() {
        let builder = Client::new("test")
            .search("kauri")
            .and_filter("year", vec!["1901".to_string()])
            .or_filter("category", vec!["Images".to_string()])
            .without_filter("content_partner", vec!["Example".to_string()])
            .geo_bbox(1.0, 2.0, 3.0, 4.0)
            .sort("date", "desc");

        assert_eq!(builder.and_filters["year"], vec!["1901"]);
        assert_eq!(builder.or_filters["category"], vec!["Images"]);
        assert_eq!(builder.without_filters["content_partner"], vec!["Example"]);
        assert_eq!(builder.geo_bbox, Some([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(builder.sort.as_deref(), Some("date"));
        assert_eq!(builder.direction.as_deref(), Some("desc"));
    }

    #[test]
    fn canonical_query_params_encode_nested_filters_and_repeated_values() {
        let builder = Client::unauthenticated()
            .search("te reo & 日本語")
            .per_page(0)
            .try_extra_param("format", "json&csv")
            .unwrap()
            .try_filter(FilterExpr::all(vec![
                FilterExpr::field("title", vec!["A&B".into(), "第二".into()]),
                FilterExpr::any(vec![FilterExpr::field("category", vec!["Images".into()])]),
            ]))
            .unwrap();

        let params = builder.query_params().unwrap();
        assert!(params.contains(&("per_page".into(), "0".into())));
        assert!(params.contains(&("and[title][]".into(), "A&B".into())));
        assert!(params.contains(&("and[title][]".into(), "第二".into())));
        assert!(params.contains(&("and[or][category][]".into(), "Images".into())));
        assert!(params.contains(&("format".into(), "json&csv".into())));
        assert!(params.windows(2).all(|pair| pair[0] <= pair[1]));
    }

    #[test]
    fn query_contract_rejects_antimeridian_and_unsafe_extra_keys() {
        let client = Client::unauthenticated();
        assert!(client
            .search("a")
            .try_geo_bbox(10.0, 170.0, -10.0, -170.0)
            .is_err());
        assert!(client.search("a").try_extra_param("", "value").is_err());
        assert!(client
            .search("a")
            .try_extra_param("format?x", "json")
            .is_err());
        assert!(client
            .search("a")
            .try_filter(FilterExpr::field("bad[field]", vec!["x".into()]))
            .is_err());
    }

    #[test]
    fn test_clear_cache() {
        let client = Client::new("test");
        // Send a search (no actual HTTP call will be made with a fake API key,
        // but the builder is created, and cache is empty initially)
        let _builder = client.search("test").fields(vec!["id".to_string()]);
        // Cache is empty after creation
        {
            let cache = client.cache.lock().unwrap();
            assert!(cache.is_empty());
        }
        // Clear cache (should not panic even when already empty)
        client.clear_cache();
        {
            let cache = client.cache.lock().unwrap();
            assert!(cache.is_empty());
        }
    }

    #[tokio::test]
    async fn test_cache_hit_returns_cached_response() {
        let mock_server = MockServer::start().await;

        let response_body = serde_json::json!({
            "search": {
                "result_count": 1,
                "results": [
                    { "id": "1", "title": "Cached Record" }
                ]
            }
        });

        // Only expect 1 HTTP request — the second call should use cache
        Mock::given(method("GET"))
            .and(query_param("text", "kauri"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = Client::new("test_key").with_base_url(mock_server.uri());

        // First call — hits the mock server
        let result1 = client.search("kauri").send().await.unwrap();
        assert_eq!(result1.search.result_count, 1);
        assert_eq!(result1.search.results[0].id, "1");

        // Second call — should return from cache
        let result2 = client.search("kauri").send().await.unwrap();
        assert_eq!(result2.search.result_count, 1);
        assert_eq!(result2.search.results[0].id, "1");
    }

    #[tokio::test]
    async fn offline_mode_fails_without_a_cached_response() {
        let error = Client::unauthenticated()
            .offline()
            .search("uncached")
            .send()
            .await
            .expect_err("offline uncached query should fail");

        assert!(error.to_string().contains("offline mode"));
    }

    #[tokio::test]
    async fn persistent_cache_is_reused_across_client_instances() {
        let mock_server = MockServer::start().await;
        let cache_path = temp_cache_path("reuse");

        let response_body = serde_json::json!({
            "search": {
                "result_count": 1,
                "results": [
                    { "id": "1", "title": "Persisted Record" }
                ],
                "facets": {}
            }
        });

        Mock::given(method("GET"))
            .and(query_param("text", "kauri"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let first_client = Client::new("first-key")
            .with_base_url(mock_server.uri())
            .with_cache_path(&cache_path)
            .expect("persistent cache should initialize");
        let first_result = first_client.search("kauri").send().await.unwrap();
        assert_eq!(first_result.search.results[0].title, "Persisted Record");

        let second_client = Client::new("first-key")
            .with_base_url(mock_server.uri())
            .with_cache_path(&cache_path)
            .expect("persistent cache should initialize");
        let second_result = second_client.search("kauri").send().await.unwrap();
        assert_eq!(second_result.search.results[0].title, "Persisted Record");

        let _ = std::fs::remove_file(cache_path);
    }

    #[tokio::test]
    async fn configured_token_uses_header_by_default() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(header("Authentication-Token", "secret-token"))
            .and(query_param("per_page", "0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "search": {"result_count": 2, "results": [], "facets": {}}
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let response = Client::new("secret-token")
            .with_base_url(mock_server.uri())
            .search("health")
            .per_page(0)
            .send()
            .await
            .expect("header-authenticated request should succeed");
        assert_eq!(response.search.result_count, 2);
    }

    #[tokio::test]
    async fn legacy_auth_is_explicitly_query_based() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(query_param("api_key", "legacy-secret"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "search": {"result_count": 1, "results": [], "facets": {}}
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        Client::new("legacy-secret")
            .with_base_url(mock_server.uri())
            .with_legacy_query_key_auth()
            .search("health")
            .send()
            .await
            .expect("explicit legacy auth should succeed");
    }

    #[test]
    fn cache_key_excludes_api_key_value() {
        let builder = Client::new("secret-key").search("kauri");
        let query_params = vec![
            ("api_key".to_string(), "secret-key".to_string()),
            ("text".to_string(), "kauri".to_string()),
        ];

        let cache_key = builder.cache_key(&query_params);

        assert!(!cache_key.contains("secret-key"));
        assert!(cache_key.contains("kauri"));
    }

    #[test]
    fn cache_key_canonicalizes_query_parameter_order() {
        let builder = Client::new("secret-key").search("kauri");
        let left = vec![
            ("api_key".to_string(), "secret-key".to_string()),
            ("text".to_string(), "kauri".to_string()),
            ("and[category][]".to_string(), "Images".to_string()),
        ];
        let right = vec![
            ("and[category][]".to_string(), "Images".to_string()),
            ("text".to_string(), "kauri".to_string()),
            ("api_key".to_string(), "different-key".to_string()),
        ];

        assert_eq!(builder.cache_key(&left), builder.cache_key(&right));
    }

    #[test]
    fn cache_namespace_differs_between_credentials_without_revealing_them() {
        let first = Client::new("first-secret").search("kauri");
        let second = Client::new("second-secret").search("kauri");
        let params = vec![("text".to_string(), "kauri".to_string())];

        let first_key = first.cache_key(&params);
        let second_key = second.cache_key(&params);
        assert_ne!(first_key, second_key);
        assert!(!first_key.contains("first-secret"));
        assert!(!second_key.contains("second-secret"));
    }

    #[test]
    fn retry_after_is_parsed_and_bounded() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::RETRY_AFTER, "999".parse().unwrap());
        assert_eq!(retry_after_delay(&headers), Some(Duration::from_secs(60)));

        headers.insert(reqwest::header::RETRY_AFTER, "0".parse().unwrap());
        assert_eq!(retry_after_delay(&headers), Some(Duration::ZERO));
    }

    #[test]
    fn test_query_builder_url_construction() {
        let client = Client::new("test_key");

        // Test basic fields and facets
        let builder = client
            .search("kauri")
            .fields(vec!["id".to_string(), "title".to_string()])
            .facet("category")
            .page(2)
            .per_page(50);

        assert_eq!(builder.text, "kauri");
        assert_eq!(builder.fields, vec!["id", "title"]);
        assert_eq!(builder.facets, vec!["category"]);
        assert_eq!(builder.page, 2);
        assert_eq!(builder.per_page, 50);

        // Test sort and direction
        let builder = client.search("test").sort("date", "desc");
        assert_eq!(builder.sort.as_deref(), Some("date"));
        assert_eq!(builder.direction.as_deref(), Some("desc"));

        // Test geo_bbox
        let builder = client.search("test").geo_bbox(-45.0, 166.0, -48.0, 179.0);
        assert_eq!(builder.geo_bbox, Some([-45.0, 166.0, -48.0, 179.0]));

        // Test filters combination
        let builder = client
            .search("test")
            .and_filter("year", vec!["1900".to_string(), "1901".to_string()])
            .or_filter("category", vec!["Images".to_string()])
            .without_filter("content_partner", vec!["Excluded".to_string()]);
        assert_eq!(builder.and_filters["year"], vec!["1900", "1901"]);
        assert_eq!(builder.or_filters["category"], vec!["Images"]);
        assert_eq!(builder.without_filters["content_partner"], vec!["Excluded"]);

        // Test facets_page and facets_per_page
        let builder = client
            .search("test")
            .facet("category")
            .facets_page(3)
            .facets_per_page(25);
        assert_eq!(builder.facets_page, 3);
        assert_eq!(builder.facets_per_page, 25);

        // Test disable_cache
        let builder = client.search("test").disable_cache();
        assert!(!builder.use_cache);

        // Test default values
        let builder = client.search("default");
        assert_eq!(builder.page, 1);
        assert_eq!(builder.per_page, 20);
        assert!(builder.fields.is_empty());
        assert!(builder.facets.is_empty());
        assert!(builder.sort.is_none());
        assert!(builder.direction.is_none());
        assert!(builder.geo_bbox.is_none());
        assert!(builder.and_filters.is_empty());
        assert!(builder.or_filters.is_empty());
        assert!(builder.without_filters.is_empty());
        assert!(!builder.exclude_filters_from_facets);
        assert!(builder.extra_params.is_empty());
        assert!(builder.use_cache);
    }
}

/// Builder pattern wrapper for composing complex search parameters.
#[derive(Debug)]
pub struct QueryBuilder {
    client: Client,
    text: String,
    page: u32,
    per_page: u32,
    fields: Vec<String>,
    facets: Vec<String>,
    facets_page: u32,
    facets_per_page: u32,
    exclude_filters_from_facets: bool,
    extra_params: Vec<(String, String)>,
    sort: Option<String>,
    direction: Option<String>,
    geo_bbox: Option<[f64; 4]>,
    and_filters: HashMap<String, Vec<String>>,
    or_filters: HashMap<String, Vec<String>>,
    without_filters: HashMap<String, Vec<String>>,
    filter_exprs: Vec<FilterExpr>,
    use_cache: bool,
}

/// Boolean expression used to serialize nested DigitalNZ filters safely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterExpr {
    /// Require every child expression.
    All(Vec<FilterExpr>),
    /// Require at least one child expression.
    Any(Vec<FilterExpr>),
    /// Exclude the child expression.
    Not(Box<FilterExpr>),
    /// Match one or more values for a field.
    Field { field: String, values: Vec<String> },
}

impl FilterExpr {
    pub fn field(field: impl Into<String>, values: Vec<String>) -> Self {
        Self::Field {
            field: field.into(),
            values,
        }
    }

    pub fn all(children: Vec<Self>) -> Self {
        Self::All(children)
    }

    pub fn any(children: Vec<Self>) -> Self {
        Self::Any(children)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(child: Self) -> Self {
        Self::Not(Box::new(child))
    }

    fn validate(&self) -> anyhow::Result<()> {
        match self {
            Self::Field { field, .. }
                if field.is_empty() || field.contains(['[', ']', '&', '=', '#', '?']) =>
            {
                anyhow::bail!("filter field is empty or contains unsafe characters")
            }
            Self::Field { .. } => Ok(()),
            Self::All(children) | Self::Any(children) => {
                if children.is_empty() {
                    anyhow::bail!("boolean filter expression cannot be empty");
                }
                for child in children {
                    child.validate()?;
                }
                Ok(())
            }
            Self::Not(child) => child.validate(),
        }
    }

    fn append_params(&self, prefix: &[String], params: &mut Vec<(String, String)>) {
        match self {
            Self::Field { field, values } => {
                let root = ["and".to_string()];
                let prefix = if prefix.is_empty() { &root[..] } else { prefix };
                let mut key = prefix[0].clone();
                for component in &prefix[1..] {
                    key.push('[');
                    key.push_str(component);
                    key.push(']');
                }
                key.push('[');
                key.push_str(field);
                key.push_str("][]");
                for value in values {
                    params.push((key.clone(), value.clone()));
                }
            }
            Self::All(children) => {
                for child in children {
                    let mut next = prefix.to_vec();
                    next.push("and".to_string());
                    child.append_params(&next, params);
                }
            }
            Self::Any(children) => {
                for child in children {
                    let mut next = prefix.to_vec();
                    next.push("or".to_string());
                    child.append_params(&next, params);
                }
            }
            Self::Not(child) => {
                let mut next = prefix.to_vec();
                next.push("without".to_string());
                child.append_params(&next, params);
            }
        }
    }
}

impl QueryBuilder {
    fn new(client: Client, text: String) -> Self {
        Self {
            client,
            text,
            page: 1,
            per_page: 20,
            fields: Vec::new(),
            facets: Vec::new(),
            facets_page: 1,
            facets_per_page: 10,
            exclude_filters_from_facets: false,
            extra_params: Vec::new(),
            sort: None,
            direction: None,
            geo_bbox: None,
            and_filters: HashMap::new(),
            or_filters: HashMap::new(),
            without_filters: HashMap::new(),
            filter_exprs: Vec::new(),
            use_cache: true,
        }
    }

    /// Disable cache retrieval for this query.
    pub fn disable_cache(mut self) -> Self {
        self.use_cache = false;
        self
    }

    /// Set page index.
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    /// Set result records count per page.
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = per_page.min(100);
        self
    }

    /// Restrict the fields returned in the result records.
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
        self
    }

    /// Add facets to harvest.
    pub fn facet(mut self, field: impl Into<String>) -> Self {
        self.facets.push(field.into());
        self
    }

    /// Set facets page index.
    pub fn facets_page(mut self, facets_page: u32) -> Self {
        self.facets_page = facets_page;
        self
    }

    /// Set count of facet terms returned.
    pub fn facets_per_page(mut self, facets_per_page: u32) -> Self {
        self.facets_per_page = facets_per_page.min(350);
        self
    }

    /// Exclude active filters from facet calculations.
    pub fn exclude_filters_from_facets(mut self, enabled: bool) -> Self {
        self.exclude_filters_from_facets = enabled;
        self
    }

    /// Add a safe provider parameter, rejecting request identity and auth keys.
    pub fn try_extra_param(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> anyhow::Result<Self> {
        let key = key.into();
        if key.is_empty()
            || is_protected_extra_param(&key)
            || key.contains(['&', '=', '#', '?', '[', ']'])
        {
            anyhow::bail!("extra parameter is protected or unsafe: {key}");
        }
        self.extra_params.push((key, value.into()));
        Ok(self)
    }

    /// Sort by field and direction.
    pub fn sort(mut self, field: impl Into<String>, direction: impl Into<String>) -> Self {
        self.sort = Some(field.into());
        self.direction = Some(direction.into());
        self
    }

    /// Limit search by geographical bounding box (North, West, South, East).
    pub fn geo_bbox(mut self, n: f64, w: f64, s: f64, e: f64) -> Self {
        self.geo_bbox = Some([n, w, s, e]);
        self
    }

    /// Set and validate a geographic bounding box.
    pub fn try_geo_bbox(self, n: f64, w: f64, s: f64, e: f64) -> anyhow::Result<Self> {
        if [n, w, s, e].iter().any(|value| !value.is_finite())
            || !(-90.0..=90.0).contains(&n)
            || !(-90.0..=90.0).contains(&s)
            || !(-180.0..=180.0).contains(&w)
            || !(-180.0..=180.0).contains(&e)
            || n < s
            || w > e
        {
            anyhow::bail!("invalid geographic bounding box");
        }
        Ok(self.geo_bbox(n, w, s, e))
    }

    /// Add an AND constraint filter.
    pub fn and_filter(mut self, field: impl Into<String>, values: Vec<String>) -> Self {
        self.and_filters.insert(field.into(), values);
        self
    }

    /// Add an OR constraint filter.
    pub fn or_filter(mut self, field: impl Into<String>, values: Vec<String>) -> Self {
        self.or_filters.insert(field.into(), values);
        self
    }

    /// Add an exclude filter (without[field][]=value).
    pub fn without_filter(mut self, field: impl Into<String>, values: Vec<String>) -> Self {
        self.without_filters.insert(field.into(), values);
        self
    }

    /// Add a validated nested boolean filter expression.
    pub fn try_filter(mut self, filter: FilterExpr) -> anyhow::Result<Self> {
        filter.validate()?;
        self.filter_exprs.push(filter);
        Ok(self)
    }

    /// Build the canonical, repeated query parameter representation.
    pub fn query_params(&self) -> anyhow::Result<Vec<(String, String)>> {
        let mut query_params = vec![
            ("text".to_string(), self.text.clone()),
            ("page".to_string(), self.page.to_string()),
            ("per_page".to_string(), self.per_page.to_string()),
        ];

        if self.client.legacy_query_key_auth && !self.client.api_key.is_empty() {
            query_params.push(("api_key".to_string(), self.client.api_key.clone()));
        }
        if !self.fields.is_empty() {
            query_params.push(("fields".to_string(), self.fields.join(",")));
        }
        if !self.facets.is_empty() {
            query_params.push(("facets".to_string(), self.facets.join(",")));
            query_params.push(("facets_page".to_string(), self.facets_page.to_string()));
            query_params.push((
                "facets_per_page".to_string(),
                self.facets_per_page.to_string(),
            ));
            if self.exclude_filters_from_facets {
                query_params.push((
                    "exclude_filters_from_facets".to_string(),
                    "true".to_string(),
                ));
            }
        }
        query_params.extend(self.extra_params.iter().cloned());
        if let (Some(sort), Some(dir)) = (self.sort.clone(), self.direction.clone()) {
            query_params.push(("sort".to_string(), sort));
            query_params.push(("direction".to_string(), dir));
        }
        if let Some(bbox) = self.geo_bbox {
            query_params.push((
                "geo_bbox".to_string(),
                format!("{},{},{},{}", bbox[0], bbox[1], bbox[2], bbox[3]),
            ));
        }
        for (prefix, filters) in [
            ("and", &self.and_filters),
            ("or", &self.or_filters),
            ("without", &self.without_filters),
        ] {
            let mut entries: Vec<_> = filters.iter().collect();
            entries.sort_by_key(|(left, _)| (*left).clone());
            for (field, values) in entries {
                for value in values {
                    query_params.push((format!("{prefix}[{field}][]"), value.clone()));
                }
            }
        }
        for filter in &self.filter_exprs {
            filter.validate()?;
            filter.append_params(&[], &mut query_params);
        }
        query_params.sort();
        Ok(query_params)
    }

    /// Execute the query asynchronously and return parsed search results.
    pub async fn send(self) -> anyhow::Result<SearchResponse> {
        if !self.client.base_url.starts_with("https://")
            && !is_local_test_url(&self.client.base_url)
        {
            return Err(anyhow::anyhow!("DigitalNZ base URL must use HTTPS"));
        }
        let query_params = self.query_params()?;

        // Generate cache key
        let cache_key = self.cache_key(&query_params);

        if self.use_cache {
            if self.client.cache_ttl.is_none() {
                if let Ok(c) = self.client.cache.lock() {
                    if let Some(cached_resp) = c.get(&cache_key) {
                        debug!("Returning cached response for query");
                        return Ok(cached_resp.clone());
                    }
                }
            }
            if let Some(cache) = &self.client.persistent_cache {
                match cache.get_with_max_age(&cache_key, self.client.cache_ttl) {
                    Ok(Some(cached_resp)) => {
                        debug!(cache_path = ?cache.path(), "Returning persistent cached response for query");
                        if let Ok(mut c) = self.client.cache.lock() {
                            c.insert(cache_key, cached_resp.clone());
                        }
                        return Ok(cached_resp);
                    }
                    Ok(None) => {}
                    Err(err) => warn!(error = ?err, "Failed to read persistent cache"),
                }
            }
        }

        if self.client.offline {
            return Err(anyhow::anyhow!(
                "offline mode has no usable cached response for this query"
            ));
        }

        let safe_params: Vec<_> = query_params
            .iter()
            .map(|(key, value)| {
                if key == "api_key" {
                    (key.as_str(), "[REDACTED]")
                } else {
                    (key.as_str(), value.as_str())
                }
            })
            .collect();
        debug!(params = ?safe_params, "Executing query with exponential retries");

        // Retry loop parameters
        let max_retries = 3;
        let mut attempt = 0;
        let base_delay = Duration::from_millis(150);

        let response = loop {
            attempt += 1;
            let mut request = self.client.http_client.get(&self.client.base_url);
            if !self.client.api_key.is_empty() && !self.client.legacy_query_key_auth {
                request = request.header("Authentication-Token", &self.client.api_key);
            }
            match request.query(&query_params).send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        match resp.bytes().await {
                            Ok(payload) => match decode_search_response(
                                response_format(&self.client.base_url)?,
                                &payload,
                            ) {
                                Ok(parsed) => break parsed,
                                Err(e) => {
                                    if attempt >= max_retries {
                                        return Err(anyhow::Error::new(DnzError::Decode).context(e));
                                    }
                                }
                            },
                            Err(e) => {
                                if attempt >= max_retries {
                                    return Err(anyhow::Error::new(DnzError::Decode).context(e));
                                }
                            }
                        }
                    } else if (status.is_server_error()
                        || status == reqwest::StatusCode::TOO_MANY_REQUESTS)
                        && attempt < max_retries
                    {
                        let retry_after = retry_after_delay(resp.headers());
                        let jitter = rand::random::<u64>() % 100;
                        let delay = retry_after.unwrap_or(base_delay * 2_u32.pow(attempt))
                            + Duration::from_millis(jitter);
                        warn!(status = ?status, attempt = attempt, delay = ?delay, "Query failed with retriable status code");
                        tokio::time::sleep(delay).await;
                    } else {
                        return Err(anyhow::Error::new(DnzError::HttpStatus {
                            status: status.as_u16(),
                            retry_after: retry_after_delay(resp.headers()),
                        }));
                    }
                }
                Err(_e) if attempt < max_retries => {
                    let jitter = rand::random::<u64>() % 100;
                    let delay = base_delay * 2_u32.pow(attempt) + Duration::from_millis(jitter);
                    warn!(attempt = attempt, delay = ?delay, "Connection error during query; retrying...");
                    tokio::time::sleep(delay).await;
                }
                Err(_e) => return Err(anyhow::Error::new(DnzError::Transport)),
            }
        };

        if self.use_cache {
            if let Ok(mut c) = self.client.cache.lock() {
                c.insert(cache_key.clone(), response.clone());
            }
            if let Some(cache) = &self.client.persistent_cache {
                let provenance = CacheProvenance {
                    source_url: self.client.base_url.clone(),
                    auth_namespace: self.client.auth_cache_namespace(),
                };
                if let Err(err) = cache.put_with_provenance(&cache_key, &response, &provenance) {
                    warn!(error = ?err, "Failed to write persistent cache");
                }
                if let Some(limit) = self.client.cache_max_entries {
                    if let Err(err) = cache.prune_to_limit(limit) {
                        warn!(error = ?err, "Failed to enforce persistent cache limit");
                    }
                }
            }
        }

        Ok(response)
    }

    fn cache_key(&self, query_params: &[(String, String)]) -> String {
        let mut params_without_key: Vec<(&str, &str)> = query_params
            .iter()
            .filter(|(key, _)| key != "api_key")
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();
        params_without_key.sort_unstable();
        format!(
            "{:?}_auth={}_{:?}",
            self.client.base_url,
            self.client.auth_cache_namespace(),
            params_without_key
        )
    }
}

fn retry_after_delay(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.trim().parse::<u64>().ok())
        .map(|seconds| Duration::from_secs(seconds.min(60)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResponseFormat {
    Json,
    Xml,
    Rss,
}

fn response_format(base_url: &str) -> anyhow::Result<ResponseFormat> {
    let url = reqwest::Url::parse(base_url)?;
    let format = url
        .path_segments()
        .and_then(|mut segments| segments.next_back())
        .and_then(|segment| segment.rsplit_once('.').map(|(_, format)| format))
        .unwrap_or("json");
    match format {
        "json" => Ok(ResponseFormat::Json),
        "xml" => Ok(ResponseFormat::Xml),
        "rss" => Ok(ResponseFormat::Rss),
        other => Err(anyhow::Error::new(DnzError::UnsupportedFormat {
            format: other.to_string(),
        })),
    }
}

fn decode_search_response(format: ResponseFormat, body: &[u8]) -> anyhow::Result<SearchResponse> {
    match format {
        ResponseFormat::Rss => normalize_rss_search_response(body)
            .map_err(|error| anyhow::Error::new(DnzError::Decode).context(error)),
        ResponseFormat::Json => serde_json::from_slice(body)
            .map_err(|error| anyhow::Error::new(DnzError::Decode).context(error))
            .and_then(normalize_search_response)
            .map_err(|error| anyhow::Error::new(DnzError::Decode).context(error)),
        ResponseFormat::Xml => normalize_xml_search_response(body)
            .map_err(|error| anyhow::Error::new(DnzError::Decode).context(error)),
    }
}

fn decode_record_response(format: ResponseFormat, body: &[u8]) -> anyhow::Result<Record> {
    match format {
        ResponseFormat::Rss => normalize_rss_record_response(body)
            .map_err(|error| anyhow::Error::new(DnzError::Decode).context(error)),
        ResponseFormat::Json => serde_json::from_slice(body)
            .map_err(|error| anyhow::Error::new(DnzError::Decode).context(error))
            .and_then(normalize_record_response)
            .map_err(|error| anyhow::Error::new(DnzError::Decode).context(error)),
        ResponseFormat::Xml => normalize_xml_record_response(body)
            .map_err(|error| anyhow::Error::new(DnzError::Decode).context(error)),
    }
}

fn record_endpoint_url(base_url: &str, record_id: &str) -> anyhow::Result<String> {
    let mut url = reqwest::Url::parse(base_url)?;
    let mut segments: Vec<String> = url
        .path_segments()
        .map(|values| values.map(str::to_string).collect())
        .unwrap_or_default();
    let format = segments
        .last()
        .and_then(|segment| {
            segment
                .split_once('.')
                .map(|(_, format)| format.to_string())
        })
        .filter(|format| matches!(format.as_str(), "json" | "xml" | "rss"))
        .unwrap_or_else(|| "json".to_string());
    if segments
        .last()
        .is_some_and(|segment| segment.starts_with("records.") || segment == "records")
    {
        segments.pop();
    }
    segments.push("records".to_string());
    segments.push(format!("{record_id}.{format}"));

    {
        let mut path = url
            .path_segments_mut()
            .map_err(|_| anyhow::anyhow!("DigitalNZ base URL cannot accept path segments"))?;
        path.clear();
        path.extend(segments.iter().map(String::as_str));
    }
    Ok(url.to_string())
}

fn more_like_this_endpoint_url(base_url: &str, record_id: &str) -> anyhow::Result<String> {
    let record_url = record_endpoint_url(base_url, record_id)?;
    let mut url = reqwest::Url::parse(&record_url)?;
    let mut segments: Vec<String> = url
        .path_segments()
        .map(|values| values.map(str::to_string).collect())
        .unwrap_or_default();
    let record_segment = segments
        .pop()
        .ok_or_else(|| anyhow::anyhow!("DigitalNZ record URL has no record segment"))?;
    let (record_id_segment, format) = record_segment
        .split_once('.')
        .map(|(record_id, format)| (record_id.to_string(), format.to_string()))
        .unwrap_or((record_segment, "json".to_string()));
    segments.push(record_id_segment);
    segments.push(format!("more_like_this.{format}"));
    {
        let mut path = url
            .path_segments_mut()
            .map_err(|_| anyhow::anyhow!("DigitalNZ base URL cannot accept path segments"))?;
        path.clear();
        path.extend(segments.iter().map(String::as_str));
    }
    Ok(url.to_string())
}

fn is_local_test_url(value: &str) -> bool {
    value.starts_with("http://127.0.0.1:") || value.starts_with("http://localhost:")
}

fn is_protected_extra_param(key: &str) -> bool {
    matches!(
        key,
        "api_key"
            | "text"
            | "page"
            | "per_page"
            | "fields"
            | "facets"
            | "facets_page"
            | "facets_per_page"
            | "sort"
            | "direction"
            | "geo_bbox"
            | "exclude_filters_from_facets"
    )
}
