//! Core client integration tests using wiremock

use dnz_core::{Client, DnzError};
use wiremock::matchers::{header, method, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mock_search_request() {
    let mock_server = MockServer::start().await;

    let test_body = serde_json::json!({
        "search": {
            "result_count": 1,
            "results": [
                {
                    "id": "12345",
                    "title": "Kauri tree photo",
                    "description": "A beautiful photo of a giant kauri tree in Northland.",
                    "collection": ["GLAM-Photos"],
                    "content_partner": ["Te Papa"],
                    "category": ["Images"]
                }
            ],
            "facets": {}
        }
    });

    Mock::given(method("GET"))
        .and(header("Authentication-Token", "test_key"))
        .and(query_param("text", "kauri"))
        .and(query_param("page", "1"))
        .and(query_param("per_page", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(test_body))
        .mount(&mock_server)
        .await;

    let client = Client::new("test_key").with_base_url(mock_server.uri());
    let response = client.search("kauri").send().await.unwrap();

    assert_eq!(response.search.result_count, 1);
    assert_eq!(response.search.results[0].id, "12345");
    assert_eq!(response.search.results[0].title, "Kauri tree photo");
    assert_eq!(
        response.search.results[0].content_partner.as_ref().unwrap()[0],
        "Te Papa"
    );
}

#[tokio::test]
async fn test_mock_search_facets_and_sorting() {
    let mock_server = MockServer::start().await;

    let test_body = serde_json::json!({
        "search": {
            "result_count": 0,
            "results": [],
            "facets": {
                "category": {
                    "Images": 5,
                    "Videos": 2
                }
            }
        }
    });

    Mock::given(method("GET"))
        .and(header("Authentication-Token", "test_key"))
        .and(query_param("text", "tui"))
        .and(query_param("facets", "category"))
        .and(query_param("sort", "date"))
        .and(query_param("direction", "desc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(test_body))
        .mount(&mock_server)
        .await;

    let client = Client::new("test_key").with_base_url(mock_server.uri());
    let response = client
        .search("tui")
        .facet("category")
        .sort("date", "desc")
        .send()
        .await
        .unwrap();

    let category_facets = response.search.facets.get("category").unwrap();
    assert_eq!(category_facets.get("Images"), Some(&5));
    assert_eq!(category_facets.get("Videos"), Some(&2));
}

#[tokio::test]
async fn test_mock_search_fields_and_excludes() {
    let mock_server = MockServer::start().await;

    let test_body = serde_json::json!({
        "search": {
            "result_count": 1,
            "results": [
                {
                    "id": "789",
                    "title": "Short Item"
                }
            ],
            "facets": {}
        }
    });

    Mock::given(method("GET"))
        .and(header("Authentication-Token", "test_key"))
        .and(query_param("text", "mock"))
        .and(query_param("fields", "id,title"))
        .and(query_param("without[category][]", "Videos"))
        .respond_with(ResponseTemplate::new(200).set_body_json(test_body))
        .mount(&mock_server)
        .await;

    let client = Client::new("test_key").with_base_url(mock_server.uri());
    let response = client
        .search("mock")
        .fields(vec!["id".to_string(), "title".to_string()])
        .without_filter("category", vec!["Videos".to_string()])
        .send()
        .await
        .unwrap();

    assert_eq!(response.search.results[0].id, "789");
    assert_eq!(response.search.results[0].title, "Short Item");
}

#[tokio::test]
async fn http_errors_are_structured_and_secret_safe() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(404)
                .insert_header("Retry-After", "999")
                .set_body_string("private-token-should-not-escape"),
        )
        .mount(&mock_server)
        .await;

    let error = Client::new("private-token-should-not-escape")
        .with_base_url(mock_server.uri())
        .search("missing")
        .send()
        .await
        .expect_err("404 should be returned as a structured error");
    let structured = error
        .downcast_ref::<DnzError>()
        .expect("error should preserve DnzError");
    assert_eq!(structured.status(), Some(404));
    // Retry-After is bounded by the client contract, so the deliberately
    // excessive fixture value is preserved at the 60-second safety cap.
    assert_eq!(
        structured.retry_after(),
        Some(std::time::Duration::from_secs(60))
    );
    assert!(!error
        .to_string()
        .contains("private-token-should-not-escape"));
}
