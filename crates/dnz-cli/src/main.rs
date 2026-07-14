//! CLI entry point for DigitalNZ integration.

use anyhow::{anyhow, Context};
use clap::Parser;
use dnz_cli::{workspace_diagnostics, Cli, Commands, Format, LogFormat};
use dnz_core::{Client, GazetteExportConfig};
use std::{env, path::PathBuf};
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn parse_filter_pair(filter: &str) -> anyhow::Result<(String, String)> {
    let parts: Vec<&str> = filter.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(anyhow!(
            "Filter must be in 'field:value' format, got '{}'",
            filter
        ));
    }
    let field = parts[0].trim();
    let value = parts[1].trim();
    if field.is_empty() || value.is_empty() {
        return Err(anyhow!(
            "Filter must be in 'field:value' format, got '{}'",
            filter
        ));
    }
    Ok((field.to_string(), value.to_string()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let log_level = if args.verbose { "debug" } else { "info" };
    init_logging(args.log_format, log_level);

    info!("Initializing DigitalNZ CLI Command...");

    match args.command {
        Commands::Ping => {
            println!("{}", dnz_core::greeting());
        }
        Commands::Doctor => {
            let checks = workspace_diagnostics();
            let mut failed = 0;
            println!("DigitalNZ workspace diagnostics");
            for check in checks {
                let status = if check.ok { "ok" } else { "warn" };
                if !check.ok {
                    failed += 1;
                }
                println!("- {status}: {} - {}", check.name, check.detail);
            }
            if failed > 0 {
                std::process::exit(1);
            }
        }
        Commands::Search {
            text,
            page,
            limit,
            format,
            sort,
            direction,
            bbox,
            and_filters,
            or_filters,
        } => {
            let client = build_client(resolve_api_key(args.api_key)?, args.cache_path.clone())?;
            let mut query = client.search(text).page(page).per_page(limit);

            if let Some(s) = sort {
                query = query.sort(s, direction);
            }
            if let Some(b) = bbox {
                query = query.geo_bbox(b[0], b[1], b[2], b[3]);
            }

            for filter in and_filters {
                let (f, v) = parse_filter_pair(&filter)?;
                query = query.and_filter(f, vec![v]);
            }
            for filter in or_filters {
                let (f, v) = parse_filter_pair(&filter)?;
                query = query.or_filter(f, vec![v]);
            }

            let result = query
                .send()
                .await
                .context("Failed to run DigitalNZ search")?;
            match format {
                Format::Json => {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                Format::Text => {
                    println!("Results found: {}", result.search.result_count);
                    for rec in result.search.results {
                        println!("- [{}] {} ({:?})", rec.id, rec.title, rec.content_partner);
                    }
                }
                Format::Markdown => {
                    println!("# DigitalNZ Search Results");
                    println!("**Total Results Found:** {}\n", result.search.result_count);
                    println!("| ID | Title | Content Partner | Category |");
                    println!("|---|---|---|---|");
                    for rec in result.search.results {
                        let partner = rec
                            .content_partner
                            .as_ref()
                            .map(|v| v.join(", "))
                            .unwrap_or_default();
                        let cat = rec
                            .category
                            .as_ref()
                            .map(|v| v.join(", "))
                            .unwrap_or_default();
                        println!("| {} | {} | {} | {} |", rec.id, rec.title, partner, cat);
                    }
                }
            }
        }
        Commands::Facets {
            text,
            fields,
            page,
            limit,
            format,
        } => {
            let client = build_client(resolve_api_key(args.api_key)?, args.cache_path.clone())?;
            let mut query = client
                .search(text)
                .page(1)
                .per_page(0)
                .facets_page(page)
                .facets_per_page(limit);
            for f in fields {
                query = query.facet(f);
            }

            let result = query.send().await.context("Failed to harvest facets")?;
            match format {
                Format::Json => {
                    println!("{}", serde_json::to_string_pretty(&result.search.facets)?);
                }
                Format::Text | Format::Markdown => {
                    println!("# DigitalNZ Facet Counts");
                    for (field, terms) in result.search.facets {
                        println!("\n## Field: {}", field);
                        println!("| Term | Count |");
                        println!("|---|---|");
                        for (term, count) in terms {
                            println!("| {} | {} |", term, count);
                        }
                    }
                }
            }
        }
        Commands::Record { id, fields, format } => {
            let client = build_client(resolve_api_key(args.api_key)?, args.cache_path.clone())?;
            let record = client.record(id).fields(fields).send().await?;
            match format {
                Format::Json => println!("{}", serde_json::to_string_pretty(&record)?),
                Format::Text => println!("[{}] {}", record.id, record.title),
                Format::Markdown => {
                    println!(
                        "# DigitalNZ Record\n\n- **ID:** {}\n- **Title:** {}",
                        record.id, record.title
                    );
                }
            }
        }
        Commands::MoreLikeThis {
            id,
            page,
            limit,
            fields,
            format,
        } => {
            let client = build_client(resolve_api_key(args.api_key)?, args.cache_path.clone())?;
            let response = client
                .more_like_this(id)
                .page(page)
                .per_page(limit)
                .fields(fields)
                .send()
                .await?;
            match format {
                Format::Json => println!("{}", serde_json::to_string_pretty(&response)?),
                Format::Text | Format::Markdown => {
                    println!("Results found: {}", response.search.result_count);
                    for record in response.search.results {
                        println!("- [{}] {}", record.id, record.title);
                    }
                }
            }
        }
        Commands::GazetteExport {
            output,
            text,
            start_page,
            max_pages,
            limit,
            sort,
            direction,
        } => {
            let client = build_client(resolve_api_key(args.api_key)?, args.cache_path.clone())?;
            let mut config = GazetteExportConfig::new(output);
            config.text = text;
            config.start_page = start_page;
            config.max_pages = max_pages;
            config.per_page = limit;
            config.sort = if sort.trim().is_empty() {
                None
            } else {
                Some(sort)
            };
            config.direction = direction;

            let manifest = dnz_core::export_gazette(&client, config)
                .await
                .context("Failed to export New Zealand Gazette records")?;
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
    }

    Ok(())
}

fn resolve_api_key(api_key: Option<String>) -> anyhow::Result<String> {
    api_key
        .or_else(|| env::var("DIGITALNZ_API_KEY").ok())
        .ok_or_else(|| {
            anyhow!("API Key missing. Set DIGITALNZ_API_KEY environment variable or pass --api-key")
        })
}

fn build_client(api_key: String, cache_path: Option<PathBuf>) -> anyhow::Result<Client> {
    let client = Client::new(api_key);
    if let Some(path) = cache_path {
        client.with_cache_path(path)
    } else {
        Ok(client)
    }
}

fn init_logging(log_format: LogFormat, log_level: &str) {
    let filter = EnvFilter::new(log_level);
    match log_format {
        LogFormat::Text => tracing_subscriber::registry()
            .with(fmt::layer().with_writer(std::io::stderr))
            .with(filter)
            .init(),
        LogFormat::Json => tracing_subscriber::registry()
            .with(fmt::layer().json().with_writer(std::io::stderr))
            .with(filter)
            .init(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_filter_pair_splits_field_and_value() {
        let result = parse_filter_pair("category:Images").unwrap();
        assert_eq!(result, ("category".to_string(), "Images".to_string()));
    }

    #[test]
    fn parse_filter_pair_rejects_missing_colon() {
        let err = parse_filter_pair("invalid").unwrap_err();
        assert!(err.to_string().contains("field:value"));
    }

    #[test]
    fn parse_filter_pair_rejects_empty_value_side() {
        let err = parse_filter_pair("field:").unwrap_err();
        assert!(err.to_string().contains("field:value"));
    }

    #[test]
    fn parse_filter_pair_handles_multiple_colons() {
        let result = parse_filter_pair("field:value:extra").unwrap();
        assert_eq!(result, ("field".to_string(), "value:extra".to_string()));
    }

    #[test]
    fn build_client_initializes_persistent_cache() {
        let cache_path = std::env::temp_dir().join(format!(
            "dnz-cli-cache-{}-{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be after unix epoch")
                .as_nanos()
        ));

        let client = build_client("key".to_string(), Some(cache_path.clone()))
            .expect("client should initialize persistent cache");

        client.clear_cache();
        assert!(cache_path.is_file());

        let _ = std::fs::remove_file(cache_path);
    }
}
