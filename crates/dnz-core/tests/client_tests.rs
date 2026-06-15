//! Core client integration tests using wiremock

use dnz_core::Client;
use wiremock::matchers::{method, query_param};
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
        .and(query_param("api_key", "test_key"))
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
        .and(query_param("api_key", "test_key"))
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
        .and(query_param("api_key", "test_key"))
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

