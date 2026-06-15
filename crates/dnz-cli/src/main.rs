//! CLI entry point for DigitalNZ integration.

use anyhow::{anyhow, Context};
use clap::Parser;
use dnz_cli::{parse_bbox, workspace_diagnostics, Cli, Commands, Format};
use dnz_core::Client;
use std::env;
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
    Ok((parts[0].to_string(), parts[1].to_string()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::new(log_level))
        .init();

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
            let client = Client::new(resolve_api_key(args.api_key)?);
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
            let client = Client::new(resolve_api_key(args.api_key)?);
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
