//! Asynchronous MCP server for DigitalNZ API.
//! Communication happens over standard input/output (stdio) streams.

use dnz_core::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::io;
use tokio::io::AsyncBufReadExt;
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
    id: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<serde_json::Value>,
    id: Option<serde_json::Value>,
}

fn get_tools_schema() -> serde_json::Value {
    serde_json::json!({
        "tools": [
            {
                "name": "search_digitalnz",
                "description": "Search DigitalNZ collection records for cultural heritage items, libraries, images, etc.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "text": {
                            "type": "string",
                            "description": "Keyword search term"
                        },
                        "page": {
                            "type": "number",
                            "description": "Page index (defaults to 1)"
                        },
                        "limit": {
                            "type": "number",
                            "description": "Page limit count (defaults to 20, max 100)"
                        },
                        "sort": {
                            "type": "string",
                            "description": "Sort field, e.g. date, title"
                        },
                        "direction": {
                            "type": "string",
                            "description": "Sort direction: asc or desc"
                        }
                    },
                    "required": ["text"]
                }
            },
            {
                "name": "get_digitalnz_facets",
                "description": "Harvest facet aggregates (counts of metadata terms like categories, partners) from DigitalNZ.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "text": {
                            "type": "string",
                            "description": "Keyword search context"
                        },
                        "fields": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Fields to facet by, e.g. ['category', 'collection']"
                        },
                        "page": {
                            "type": "number",
                            "description": "Facet result page (defaults to 1)"
                        },
                        "limit": {
                            "type": "number",
                            "description": "Facet terms count limit (defaults to 10)"
                        }
                    },
                    "required": ["text", "fields"]
                }
            }
        ]
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--export-schema".to_string()) {
        let schema = get_tools_schema();
        println!("{}", serde_json::to_string_pretty(&schema)?);
        return Ok(());
    }

    // Logging is targeted strictly to standard error (stderr) to prevent breaking the stdio JSON-RPC stream.
    init_logging();

    info!("Starting DigitalNZ MCP Server...");

    // Read API Key
    let api_key = match env::var("DIGITALNZ_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("DIGITALNZ_API_KEY environment variable is not set. MCP server running in unauthenticated state.");
            String::new()
        }
    };

    let client = build_client(api_key)?;
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut line = String::new();

    loop {
        line.clear();
        match stdin.read_line(&mut line).await {
            Ok(0) => {
                info!("EOF reached on stdin. Stopping MCP server.");
                break;
            }
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                match serde_json::from_str::<JsonRpcRequest>(trimmed) {
                    Ok(req) => {
                        let res = handle_request(&req, &client).await;
                        let response = match res {
                            Ok(result) => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: Some(result),
                                error: None,
                                id: req.id,
                            },
                            Err(err) => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(serde_json::json!({
                                    "code": -32603,
                                    "message": err.to_string()
                                })),
                                id: req.id,
                            },
                        };

                        if let Ok(resp_str) = serde_json::to_string(&response) {
                            println!("{}", resp_str);
                        }
                    }
                    Err(e) => {
                        error!(error = ?e, "Failed to parse JSON-RPC request");
                        let err_resp = JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            result: None,
                            error: Some(serde_json::json!({
                                "code": -32700,
                                "message": "Parse error"
                            })),
                            id: None,
                        };
                        if let Ok(resp_str) = serde_json::to_string(&err_resp) {
                            println!("{}", resp_str);
                        }
                    }
                }
            }
            Err(e) => {
                error!(error = ?e, "Error reading from stdin");
                break;
            }
        }
    }

    Ok(())
}

fn build_client(api_key: String) -> anyhow::Result<Client> {
    let client = Client::new(api_key);
    if let Ok(path) = env::var("DNZ_CACHE_PATH") {
        client.with_cache_path(path)
    } else {
        Ok(client)
    }
}

fn init_logging() {
    let filter = EnvFilter::new(env::var("DNZ_LOG").unwrap_or_else(|_| "info".to_string()));
    if env::var("DNZ_LOG_FORMAT")
        .map(|value| value.eq_ignore_ascii_case("json"))
        .unwrap_or(false)
    {
        tracing_subscriber::registry()
            .with(fmt::layer().json().with_writer(io::stderr))
            .with(filter)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(fmt::layer().with_writer(io::stderr))
            .with(filter)
            .init();
    }
}

async fn handle_request(
    req: &JsonRpcRequest,
    client: &Client,
) -> anyhow::Result<serde_json::Value> {
    match req.method.as_str() {
        "initialize" => Ok(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "dnz-mcp",
                "version": "0.1.0"
            }
        })),
        "tools/list" | "listTools" => Ok(get_tools_schema()),
        "tools/call" | "callTool" => {
            let params = req
                .params
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Missing params"))?;
            let tool_name = params
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing tool name"))?;
            let tool_arguments = params
                .get("arguments")
                .ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

            match tool_name {
                "search_digitalnz" => {
                    let text = tool_arguments
                        .get("text")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| anyhow::anyhow!("Missing argument: text"))?;
                    let page = tool_arguments
                        .get("page")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(1) as u32;
                    let limit = tool_arguments
                        .get("limit")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(20) as u32;

                    let mut query = client.search(text).page(page).per_page(limit);
                    if let Some(sort) = tool_arguments.get("sort").and_then(|v| v.as_str()) {
                        let dir = tool_arguments
                            .get("direction")
                            .and_then(|v| v.as_str())
                            .unwrap_or("asc");
                        query = query.sort(sort, dir);
                    }

                    let search_res = query.send().await?;
                    Ok(serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": format!("Found {} results:\n{}", search_res.search.result_count, serde_json::to_string_pretty(&search_res.search.results)?)
                            }
                        ]
                    }))
                }
                "get_digitalnz_facets" => {
                    let text = tool_arguments
                        .get("text")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| anyhow::anyhow!("Missing argument: text"))?;
                    let fields_val = tool_arguments
                        .get("fields")
                        .ok_or_else(|| anyhow::anyhow!("Missing argument: fields"))?;
                    let fields: Vec<String> = serde_json::from_value(fields_val.clone())?;
                    let page = tool_arguments
                        .get("page")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(1) as u32;
                    let limit = tool_arguments
                        .get("limit")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(10) as u32;

                    let mut query = client
                        .search(text)
                        .page(1)
                        .per_page(0)
                        .facets_page(page)
                        .facets_per_page(limit);
                    for f in fields {
                        query = query.facet(f);
                    }

                    let search_res = query.send().await?;
                    Ok(serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": serde_json::to_string_pretty(&search_res.search.facets)?
                            }
                        ]
                    }))
                }
                _ => Err(anyhow::anyhow!("Tool not found: {}", tool_name)),
            }
        }
        _ => Err(anyhow::anyhow!("Method not found: {}", req.method)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn request(method: &str, params: Option<serde_json::Value>) -> JsonRpcRequest {
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: Some(json!(1)),
        }
    }

    fn text_content(response: &serde_json::Value) -> &str {
        response["content"][0]["text"]
            .as_str()
            .expect("tool response should include text content")
    }

    #[test]
    fn tools_schema_lists_expected_tools() {
        let schema = get_tools_schema();
        let tool_names: Vec<&str> = schema["tools"]
            .as_array()
            .expect("tools schema should contain tools array")
            .iter()
            .map(|tool| tool["name"].as_str().expect("tool should have a name"))
            .collect();

        assert!(tool_names.contains(&"search_digitalnz"));
        assert!(tool_names.contains(&"get_digitalnz_facets"));
        assert_eq!(
            schema["tools"][0]["inputSchema"]["required"],
            json!(["text"])
        );
    }

    #[tokio::test]
    async fn initialize_returns_mcp_server_info() {
        let client = Client::new("test");
        let response = handle_request(&request("initialize", None), &client)
            .await
            .expect("initialize should succeed");

        assert_eq!(response["protocolVersion"], "2024-11-05");
        assert_eq!(response["serverInfo"]["name"], "dnz-mcp");
        assert!(response["capabilities"]["tools"].is_object());
    }

    #[tokio::test]
    async fn tool_call_reports_missing_and_unknown_tools() {
        let client = Client::new("test");

        let missing_params = handle_request(&request("tools/call", None), &client)
            .await
            .expect_err("missing params should be rejected");
        assert!(missing_params.to_string().contains("Missing params"));

        let missing_name = handle_request(&request("tools/call", Some(json!({}))), &client)
            .await
            .expect_err("missing tool name should be rejected");
        assert!(missing_name.to_string().contains("Missing tool name"));

        let unknown_tool = handle_request(
            &request(
                "tools/call",
                Some(json!({
                    "name": "unknown_tool",
                    "arguments": {}
                })),
            ),
            &client,
        )
        .await
        .expect_err("unknown tool should be rejected");
        assert!(unknown_tool.to_string().contains("Tool not found"));
    }

    #[tokio::test]
    async fn search_tool_calls_digitalnz_client() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(query_param("api_key", "test-key"))
            .and(query_param("text", "kauri"))
            .and(query_param("page", "2"))
            .and(query_param("per_page", "5"))
            .and(query_param("sort", "date"))
            .and(query_param("direction", "desc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "search": {
                    "result_count": 1,
                    "results": [
                        {
                            "id": "abc",
                            "title": "Kauri gum record",
                            "description": null
                        }
                    ],
                    "facets": {}
                }
            })))
            .mount(&server)
            .await;

        let client = Client::new("test-key").with_base_url(server.uri());
        let response = handle_request(
            &request(
                "tools/call",
                Some(json!({
                    "name": "search_digitalnz",
                    "arguments": {
                        "text": "kauri",
                        "page": 2,
                        "limit": 5,
                        "sort": "date",
                        "direction": "desc"
                    }
                })),
            ),
            &client,
        )
        .await
        .expect("search tool should return mocked results");

        let text = text_content(&response);
        assert!(text.contains("Found 1 results"));
        assert!(text.contains("Kauri gum record"));
    }

    #[tokio::test]
    async fn facets_tool_calls_digitalnz_client() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(query_param("api_key", "test-key"))
            .and(query_param("text", "kauri"))
            .and(query_param("page", "1"))
            .and(query_param("per_page", "1"))
            .and(query_param("facets", "category,collection"))
            .and(query_param("facets_page", "2"))
            .and(query_param("facets_per_page", "3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "search": {
                    "result_count": 0,
                    "results": [],
                    "facets": {
                        "category": {
                            "Images": 7
                        },
                        "collection": {
                            "Museum": 2
                        }
                    }
                }
            })))
            .mount(&server)
            .await;

        let client = Client::new("test-key").with_base_url(server.uri());
        let response = handle_request(
            &request(
                "tools/call",
                Some(json!({
                    "name": "get_digitalnz_facets",
                    "arguments": {
                        "text": "kauri",
                        "fields": ["category", "collection"],
                        "page": 2,
                        "limit": 3
                    }
                })),
            ),
            &client,
        )
        .await
        .expect("facets tool should return mocked facets");

        let text = text_content(&response);
        assert!(text.contains("\"Images\": 7"));
        assert!(text.contains("\"Museum\": 2"));
    }

    #[test]
    fn build_client_initializes_env_cache_path() {
        let cache_path = std::env::temp_dir().join(format!(
            "dnz-mcp-cache-{}-{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be after unix epoch")
                .as_nanos()
        ));

        std::env::set_var("DNZ_CACHE_PATH", &cache_path);
        let client = build_client("key".to_string()).expect("client should initialize cache");
        client.clear_cache();
        std::env::remove_var("DNZ_CACHE_PATH");

        assert!(cache_path.is_file());

        let _ = std::fs::remove_file(cache_path);
    }
}
