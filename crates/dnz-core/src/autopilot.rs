//! Automated query autopilot to harvest deep datasets by bypassing API caps.

use crate::client::Client;
use crate::models::Record;
use std::collections::{HashMap, HashSet};
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
        let facet_response = self.client
            .search(query_text)
            .per_page(0)
            .facet("year")
            .facets_per_page(150)
            .send()
            .await?;

        let year_counts = facet_response.search.facets
            .get("year")
            .cloned()
            .unwrap_or_default();

        info!(years_found = year_counts.len(), "Evaluated query density facets");

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
                                records.extend(search_res.search.results);
                                if search_res.search.results.is_empty() {
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

        info!(total_harvested = all_records.len(), "Finished deep harvest query loop");
        Ok(all_records)
    }
}
