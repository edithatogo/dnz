//! Automated query autopilot to harvest deep datasets by bypassing API caps.

use crate::client::Client;
use crate::models::Record;
use std::collections::HashSet;
use tracing::{info, warn};

/// High-level query coordinator mapping density-aware partitioned queries.
pub struct Autopilot {
    client: Client,
}

impl Autopilot {
    /// Create a new Autopilot engine instance.
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Autonomously harvest records past the 1,000 deep boundary by partitioning queries by year.
    pub async fn harvest_deep(&self, query_text: &str) -> anyhow::Result<Vec<Record>> {
        info!(query = query_text, "Starting agentic deep harvest pipeline");

        // Step 1: Query year facet density to evaluate how to partition
        let facet_response = self
            .client
            .search(query_text)
            .per_page(0)
            .facet("year")
            .facets_per_page(150)
            .send()
            .await?;

        let year_counts = facet_response
            .search
            .facets
            .get("year")
            .cloned()
            .unwrap_or_default();

        info!(
            years_found = year_counts.len(),
            "Evaluated query density facets"
        );

        let mut partitions = Vec::new();

        // Step 2: Build year partitions
        for (year, count) in &year_counts {
            if *count > 0 {
                partitions.push((year.clone(), *count));
            }
        }

        // Sort year partitions to execute chronologically
        partitions.sort_by(|a, b| a.0.cmp(&b.0));

        let mut all_records = Vec::new();
        let mut unique_ids = HashSet::new();

        // Step 3: Concurrently fetch partitioned ranges using a rate-limiting chunk gate
        // We throttle concurrency (chunk size of 3) to prevent triggering 429 rate limits
        for chunk in partitions.chunks(3) {
            let mut tasks = Vec::new();

            for (year, count) in chunk {
                let client_clone = self.client.clone();
                let q_text = query_text.to_string();
                let yr = year.clone();
                let record_count = *count;

                let handle = tokio::spawn(async move {
                    let mut records = Vec::new();
                    // Estimate page iteration limit
                    let max_pages = ((record_count + 99) / 100).min(10) as u32; // Limit to 1000 per partition (API limit)

                    info!(year = %yr, expected_records = record_count, pages = max_pages, "Fetching query partition segment");

                    for page in 1..=max_pages {
                        let res = client_clone
                            .search(&q_text)
                            .and_filter("year", vec![yr.clone()])
                            .page(page)
                            .per_page(100)
                            .send()
                            .await;

                        match res {
                            Ok(search_res) => {
                                let page_results = search_res.search.results;
                                let is_empty = page_results.is_empty();
                                records.extend(page_results);
                                if is_empty {
                                    break;
                                }
                            }
                            Err(e) => {
                                warn!(year = %yr, page = page, error = ?e, "Failed to fetch query partition page; skipping");
                                break;
                            }
                        }
                    }
                    records
                });

                tasks.push(handle);
            }

            // Step 4: Reconcile parallel results streams
            for task in tasks {
                if let Ok(partition_records) = task.await {
                    for rec in partition_records {
                        if unique_ids.insert(rec.id.clone()) {
                            all_records.push(rec);
                        }
                    }
                }
            }
        }

        info!(
            total_harvested = all_records.len(),
            "Finished deep harvest query loop"
        );
        Ok(all_records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn harvest_deep_returns_empty_when_year_facets_are_absent() {
        let mock_server = MockServer::start().await;
        let facet_body = serde_json::json!({
            "search": {
                "result_count": 0,
                "results": [],
                "facets": {}
            }
        });

        Mock::given(method("GET"))
            .and(query_param("text", "kauri"))
            .and(query_param("facets", "year"))
            .respond_with(ResponseTemplate::new(200).set_body_json(facet_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = Client::new("test_key").with_base_url(mock_server.uri());
        let records = Autopilot::new(client).harvest_deep("kauri").await.unwrap();

        assert!(records.is_empty());
    }

    #[tokio::test]
    async fn harvest_deep_fetches_year_partitions_and_deduplicates_records() {
        let mock_server = MockServer::start().await;
        let facet_body = serde_json::json!({
            "search": {
                "result_count": 3,
                "results": [],
                "facets": {
                    "year": {
                        "1900": 1,
                        "1901": 2,
                        "1902": 0
                    }
                }
            }
        });
        let year_1900_body = serde_json::json!({
            "search": {
                "result_count": 1,
                "results": [
                    { "id": "rec-1", "title": "Record One" }
                ],
                "facets": {}
            }
        });
        let year_1901_body = serde_json::json!({
            "search": {
                "result_count": 2,
                "results": [
                    { "id": "rec-1", "title": "Record One Duplicate" },
                    { "id": "rec-2", "title": "Record Two" }
                ],
                "facets": {}
            }
        });

        Mock::given(method("GET"))
            .and(query_param("text", "kauri"))
            .and(query_param("facets", "year"))
            .respond_with(ResponseTemplate::new(200).set_body_json(facet_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(query_param("text", "kauri"))
            .and(query_param("and[year][]", "1900"))
            .and(query_param("page", "1"))
            .and(query_param("per_page", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(year_1900_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(query_param("text", "kauri"))
            .and(query_param("and[year][]", "1901"))
            .and(query_param("page", "1"))
            .and(query_param("per_page", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(year_1901_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = Client::new("test_key").with_base_url(mock_server.uri());
        let mut ids: Vec<String> = Autopilot::new(client)
            .harvest_deep("kauri")
            .await
            .unwrap()
            .into_iter()
            .map(|record| record.id)
            .collect();
        ids.sort();

        assert_eq!(ids, vec!["rec-1", "rec-2"]);
    }
}
