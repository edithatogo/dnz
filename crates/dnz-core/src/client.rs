//! Client and query builder implementations for the DigitalNZ API.

use crate::models::SearchResponse;
use reqwest::Client as HttpClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{debug, warn};

/// Main client struct interacting with DigitalNZ endpoints.
#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    base_url: String,
    http_client: HttpClient,
    // Thread-safe query cache
    cache: Arc<Mutex<HashMap<String, SearchResponse>>>,
}

impl Client {
    /// Create a new Client.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.digitalnz.org/v3/records.json".to_string(),
            http_client: HttpClient::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Provide a custom API endpoint (useful for mock tests).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Clear cache entries.
    pub fn clear_cache(&self) {
        if let Ok(mut c) = self.cache.lock() {
            c.clear();
        }
    }

    /// Create a search query builder.
    pub fn search(&self, text: impl Into<String>) -> QueryBuilder {
        QueryBuilder::new(self.clone(), text.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn per_page_clamps_to_api_bounds() {
        let client = Client::new("test");

        assert_eq!(client.search("a").per_page(0).per_page, 1);
        assert_eq!(client.search("a").per_page(1).per_page, 1);
        assert_eq!(client.search("a").per_page(100).per_page, 100);
        assert_eq!(client.search("a").per_page(1_000).per_page, 100);
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
    sort: Option<String>,
    direction: Option<String>,
    geo_bbox: Option<[f64; 4]>,
    and_filters: HashMap<String, Vec<String>>,
    or_filters: HashMap<String, Vec<String>>,
    without_filters: HashMap<String, Vec<String>>,
    use_cache: bool,
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
            sort: None,
            direction: None,
            geo_bbox: None,
            and_filters: HashMap::new(),
            or_filters: HashMap::new(),
            without_filters: HashMap::new(),
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
        self.per_page = per_page.clamp(1, 100);
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
        self.facets_per_page = facets_per_page;
        self
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

    /// Execute the query asynchronously and return parsed search results.
    pub async fn send(self) -> anyhow::Result<SearchResponse> {
        let mut query_params = vec![
            ("api_key".to_string(), self.client.api_key.clone()),
            ("text".to_string(), self.text.clone()),
            ("page".to_string(), self.page.to_string()),
            ("per_page".to_string(), self.per_page.to_string()),
        ];

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
        }

        if let (Some(sort), Some(dir)) = (self.sort.clone(), self.direction.clone()) {
            query_params.push(("sort".to_string(), sort));
            query_params.push(("direction".to_string(), dir));
        }

        if let Some(bbox) = self.geo_bbox {
            let bbox_str = format!("{},{},{},{}", bbox[0], bbox[1], bbox[2], bbox[3]);
            query_params.push(("geo_bbox".to_string(), bbox_str));
        }

        // Handle AND filters (e.g. and[content_partner][]=value)
        for (field, values) in &self.and_filters {
            for val in values {
                query_params.push((format!("and[{}][]", field), val.clone()));
            }
        }

        // Handle OR filters (e.g. or[category][]=value)
        for (field, values) in &self.or_filters {
            for val in values {
                query_params.push((format!("or[{}][]", field), val.clone()));
            }
        }

        // Handle WITHOUT filters (e.g. without[category][]=value)
        for (field, values) in &self.without_filters {
            for val in values {
                query_params.push((format!("without[{}][]", field), val.clone()));
            }
        }

        // Generate cache key
        let cache_key = format!("{:?}_{:?}", self.client.base_url, query_params);

        if self.use_cache {
            if let Ok(c) = self.client.cache.lock() {
                if let Some(cached_resp) = c.get(&cache_key) {
                    debug!("Returning cached response for query");
                    return Ok(cached_resp.clone());
                }
            }
        }

        debug!(params = ?query_params, "Executing query with exponential retries");

        // Retry loop parameters
        let max_retries = 3;
        let mut attempt = 0;
        let base_delay = Duration::from_millis(150);

        let response = loop {
            attempt += 1;
            match self
                .client
                .http_client
                .get(&self.client.base_url)
                .query(&query_params)
                .send()
                .await
            {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        match resp.json::<SearchResponse>().await {
                            Ok(parsed) => break parsed,
                            Err(e) => {
                                if attempt >= max_retries {
                                    return Err(anyhow::anyhow!("JSON Parse Error: {}", e));
                                }
                            }
                        }
                    } else if (status.is_server_error()
                        || status == reqwest::StatusCode::TOO_MANY_REQUESTS)
                        && attempt < max_retries
                    {
                        let jitter = rand::random::<u64>() % 100;
                        let delay = base_delay * 2_u32.pow(attempt) + Duration::from_millis(jitter);
                        warn!(status = ?status, attempt = attempt, delay = ?delay, "Query failed with retriable status code");
                        tokio::time::sleep(delay).await;
                    } else {
                        return Err(anyhow::anyhow!(
                            "Query failed with HTTP status code: {}",
                            status
                        ));
                    }
                }
                Err(e) if attempt < max_retries => {
                    let jitter = rand::random::<u64>() % 100;
                    let delay = base_delay * 2_u32.pow(attempt) + Duration::from_millis(jitter);
                    warn!(error = ?e, attempt = attempt, delay = ?delay, "Connection error during query; retrying...");
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(anyhow::anyhow!("HTTP request failed: {}", e)),
            }
        };

        if self.use_cache {
            if let Ok(mut c) = self.client.cache.lock() {
                c.insert(cache_key, response.clone());
            }
        }

        Ok(response)
    }
}
