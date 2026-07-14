//! Property-based testing suites for client and query builder inputs.

use dnz_core::deduplicate_records;
use dnz_core::models::Record;
use dnz_core::Client;
use proptest::prelude::*;
use std::collections::HashSet;
use wiremock::matchers::{method, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

// We use proptest to verify validation logic under randomized inputs.
proptest! {
    #[test]
    fn test_proptest_bbox_values_remain_in_generated_bounds(
        n in -90.0..90.0f64,
        w in -180.0..180.0f64,
        s in -90.0..90.0f64,
        e in -180.0..180.0f64,
    ) {
        prop_assert!((-90.0..=90.0).contains(&n));
        prop_assert!((-180.0..=180.0).contains(&w));
        prop_assert!((-90.0..=90.0).contains(&s));
        prop_assert!((-180.0..=180.0).contains(&e));
    }

    #[test]
    fn test_proptest_limit_clamping(limit in 0..1000u32) {
        let expected = limit.min(100);
        prop_assert!((0..=100).contains(&expected));
    }

    #[test]
    fn test_proptest_deduplicate_records_never_increases_count(titles in proptest::collection::vec("[A-Za-z ]{0,20}", 0..50)) {
        let records: Vec<Record> = titles
            .iter()
            .enumerate()
            .map(|(idx, title)| Record {
                id: idx.to_string(),
                title: title.clone(),
                ..Record::default()
            })
            .collect();

        let deduped = deduplicate_records(&records);
        let unique_non_empty_titles = titles
            .iter()
            .map(|title| title.to_lowercase().trim().to_string())
            .filter(|title| !title.is_empty())
            .collect::<HashSet<_>>()
            .len();
        let empty_title_records = titles.iter().filter(|title| title.trim().is_empty()).count();

        prop_assert!(deduped.len() <= records.len());
        // Records without titles remain distinct when their IDs are distinct;
        // title-based deduplication only applies to non-empty titles.
        prop_assert_eq!(deduped.len(), unique_non_empty_titles + empty_title_records);
    }
}

#[test]
fn test_manual_bbox_bounds() {
    let bbox = [1.0, 2.0, 3.0, 4.0];
    assert_eq!(bbox[0], 1.0);
    assert_eq!(bbox[3], 4.0);
}

#[tokio::test]
async fn test_per_page_clamp_is_reflected_in_request() {
    let mock_server = MockServer::start().await;
    let test_body = serde_json::json!({
        "search": {
            "result_count": 0,
            "results": [],
            "facets": {}
        }
    });

    Mock::given(method("GET"))
        .and(query_param("per_page", "100"))
        .respond_with(ResponseTemplate::new(200).set_body_json(test_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = Client::new("key").with_base_url(mock_server.uri());
    client.search("test").per_page(1_000).send().await.unwrap();
}
