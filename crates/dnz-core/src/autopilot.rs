//! Automated query autopilot to harvest deep datasets by bypassing API caps.

use crate::client::Client;
use crate::models::Record;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{info, warn};

/// Controls concurrency, request pacing, and resumability for a deep harvest.
#[derive(Debug, Clone)]
pub struct HarvestOptions {
    pub max_parallel_partitions: usize,
    pub request_delay: Duration,
    pub checkpoint_path: Option<PathBuf>,
}

/// One deterministic facet-density partition produced by the planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DensityPartition {
    pub values: Vec<String>,
    pub estimated_count: u64,
    pub indivisible: bool,
}

/// Recursively split sorted facet buckets until each partition is below the
/// requested density, preserving oversized singleton buckets as explicit
/// limitations rather than discarding or truncating them.
pub fn plan_density_partitions(
    facet_counts: &BTreeMap<String, u64>,
    max_records_per_partition: u64,
) -> Vec<DensityPartition> {
    let limit = max_records_per_partition.max(1);
    let entries: Vec<_> = facet_counts
        .iter()
        .map(|(value, count)| (value.clone(), *count))
        .collect();
    let mut partitions = Vec::new();
    split_density_partition(&entries, limit, &mut partitions);
    partitions
}

fn split_density_partition(
    entries: &[(String, u64)],
    limit: u64,
    output: &mut Vec<DensityPartition>,
) {
    if entries.is_empty() {
        return;
    }
    let total = entries.iter().map(|(_, count)| *count).sum::<u64>();
    if total <= limit || entries.len() == 1 {
        output.push(DensityPartition {
            values: entries.iter().map(|(value, _)| value.clone()).collect(),
            estimated_count: total,
            indivisible: entries.len() == 1 && total > limit,
        });
        return;
    }
    let midpoint = entries.len() / 2;
    split_density_partition(&entries[..midpoint], limit, output);
    split_density_partition(&entries[midpoint..], limit, output);
}

impl Default for HarvestOptions {
    fn default() -> Self {
        Self {
            max_parallel_partitions: 3,
            request_delay: Duration::ZERO,
            checkpoint_path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HarvestCheckpoint {
    query_text: String,
    completed_years: Vec<String>,
    records: Vec<Record>,
}

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
        self.harvest_deep_with_options(query_text, HarvestOptions::default())
            .await
    }

    /// Harvest with explicit rate/concurrency controls and an optional resume checkpoint.
    pub async fn harvest_deep_with_options(
        &self,
        query_text: &str,
        options: HarvestOptions,
    ) -> anyhow::Result<Vec<Record>> {
        info!(query = query_text, "Starting agentic deep harvest pipeline");

        let mut completed_years = HashSet::new();
        let mut all_records = Vec::new();
        if let Some(path) = &options.checkpoint_path {
            if path.exists() {
                let checkpoint: HarvestCheckpoint = serde_json::from_slice(&std::fs::read(path)?)?;
                if checkpoint.query_text != query_text {
                    anyhow::bail!("harvest checkpoint query does not match requested query");
                }
                completed_years.extend(checkpoint.completed_years);
                all_records = checkpoint.records;
            }
        }

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

        let mut unique_ids = HashSet::new();
        unique_ids.extend(all_records.iter().map(|record| record.id.clone()));

        // Step 3: Concurrently fetch partitioned ranges using a rate-limiting chunk gate
        // We throttle concurrency (chunk size of 3) to prevent triggering 429 rate limits
        let parallelism = options.max_parallel_partitions.max(1);
        for chunk in partitions
            .iter()
            .filter(|(year, _)| !completed_years.contains(year))
            .collect::<Vec<_>>()
            .chunks(parallelism)
        {
            let mut tasks = Vec::new();

            for (year, count) in chunk {
                let client_clone = self.client.clone();
                let q_text = query_text.to_string();
                let yr = year.clone();
                let record_count = *count;
                let request_delay = options.request_delay;

                let handle = tokio::spawn(async move {
                    let mut records = Vec::new();
                    let mut completed = true;
                    // Estimate page iteration limit
                    let max_pages = record_count.div_ceil(100) as u32;

                    info!(year = %yr, expected_records = record_count, pages = max_pages, "Fetching query partition segment");

                    for page in 1..=max_pages {
                        if !request_delay.is_zero() {
                            tokio::time::sleep(request_delay).await;
                        }
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
                                completed = false;
                                break;
                            }
                        }
                    }
                    (yr, records, completed)
                });

                tasks.push(handle);
            }

            // Step 4: Reconcile parallel results streams
            for task in tasks {
                if let Ok((year, partition_records, completed)) = task.await {
                    for rec in partition_records {
                        if unique_ids.insert(rec.id.clone()) {
                            all_records.push(rec);
                        }
                    }
                    if completed {
                        completed_years.insert(year);
                    }
                }
            }

            if let Some(path) = &options.checkpoint_path {
                write_checkpoint(path, query_text, &completed_years, &all_records)?;
            }
        }

        info!(
            total_harvested = all_records.len(),
            "Finished deep harvest query loop"
        );
        Ok(all_records)
    }
}

fn write_checkpoint(
    path: &PathBuf,
    query_text: &str,
    completed_years: &HashSet<String>,
    records: &[Record],
) -> anyhow::Result<()> {
    let mut completed_years: Vec<_> = completed_years.iter().cloned().collect();
    completed_years.sort();
    let checkpoint = HarvestCheckpoint {
        query_text: query_text.to_string(),
        completed_years,
        records: records.to_vec(),
    };
    let temporary = path.with_extension("json.tmp");
    std::fs::write(&temporary, serde_json::to_vec_pretty(&checkpoint)?)?;
    std::fs::rename(temporary, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use wiremock::matchers::{method, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn density_planner_splits_deterministically_and_flags_oversized_singletons() {
        let facets = BTreeMap::from([
            ("1900".to_string(), 7),
            ("1901".to_string(), 7),
            ("1902".to_string(), 30),
        ]);
        let partitions = plan_density_partitions(&facets, 10);

        assert_eq!(partitions[0].values, vec!["1900"]);
        assert_eq!(partitions[1].values, vec!["1901"]);
        assert_eq!(partitions[2].values, vec!["1902"]);
        assert!(partitions[2].indivisible);
        assert_eq!(partitions[2].estimated_count, 30);
    }

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
        let checkpoint_path = std::env::temp_dir().join(format!(
            "dnz-harvest-checkpoint-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut ids: Vec<String> = Autopilot::new(client)
            .harvest_deep_with_options(
                "kauri",
                HarvestOptions {
                    max_parallel_partitions: 1,
                    request_delay: Duration::ZERO,
                    checkpoint_path: Some(checkpoint_path.clone()),
                },
            )
            .await
            .unwrap()
            .into_iter()
            .map(|record| record.id)
            .collect();
        ids.sort();

        assert_eq!(ids, vec!["rec-1", "rec-2"]);
        let checkpoint: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&checkpoint_path).unwrap()).unwrap();
        assert_eq!(checkpoint["query_text"], "kauri");
        assert_eq!(checkpoint["completed_years"].as_array().unwrap().len(), 2);
        let _ = std::fs::remove_file(checkpoint_path);
    }
}
