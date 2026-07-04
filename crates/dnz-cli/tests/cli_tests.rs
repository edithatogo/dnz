//! CLI integration tests.

use clap::{CommandFactory, Parser};

#[test]
fn test_cli_metadata_clap() {
    // This parses clap metadata declarations to ensure no option conflicts.
    // Clap debug_assert fails at test run time if clap attributes are misconfigured.
    dnz_cli::Cli::command().debug_assert();
}

#[test]
fn test_cli_ping() {
    assert_eq!(dnz_core::greeting(), "Hello from dnz-core!");
}

#[test]
fn test_workspace_diagnostics_include_core_checks() {
    let checks = dnz_cli::workspace_diagnostics();
    assert!(checks.iter().any(|check| check.name == "workspace_path"));
    assert!(checks.iter().any(|check| check.name == "target_write"));
    assert!(checks.iter().any(|check| check.name == "digitalnz_api_key"));
}

#[test]
fn test_parse_bbox_accepts_four_coordinates() {
    let bbox = dnz_cli::parse_bbox("-34.5,172.1,-47.3,178.9").expect("bbox should parse");

    assert_eq!(bbox, [-34.5, 172.1, -47.3, 178.9]);
}

#[test]
fn test_parse_bbox_rejects_missing_coordinates() {
    let err = dnz_cli::parse_bbox("-34.5,172.1,-47.3").expect_err("bbox should reject 3 values");

    assert!(err.contains("exactly 4"));
}

#[test]
fn test_cli_parses_search_options() {
    let cli = dnz_cli::Cli::parse_from([
        "dnz",
        "--api-key",
        "test-key",
        "search",
        "kauri",
        "--page",
        "2",
        "--limit",
        "5",
        "--format",
        "json",
        "--sort",
        "date",
        "--direction",
        "desc",
        "--bbox",
        "-34.5,172.1,-47.3,178.9",
        "--and",
        "category:Images",
        "--or",
        "collection:Museum",
    ]);

    assert_eq!(cli.api_key.as_deref(), Some("test-key"));
    match cli.command {
        dnz_cli::Commands::Search {
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
            assert_eq!(text, "kauri");
            assert_eq!(page, 2);
            assert_eq!(limit, 5);
            assert_eq!(format, dnz_cli::Format::Json);
            assert_eq!(sort.as_deref(), Some("date"));
            assert_eq!(direction, "desc");
            assert_eq!(bbox, Some([-34.5, 172.1, -47.3, 178.9]));
            assert_eq!(and_filters, vec!["category:Images"]);
            assert_eq!(or_filters, vec!["collection:Museum"]);
        }
        other => panic!("expected search command, got {other:?}"),
    }
}

#[test]
fn test_cli_parses_facets_and_doctor_commands() {
    let facets = dnz_cli::Cli::parse_from([
        "dnz",
        "facets",
        "waka",
        "--fields",
        "category,collection",
        "--page",
        "3",
        "--limit",
        "12",
        "--format",
        "text",
    ]);

    match facets.command {
        dnz_cli::Commands::Facets {
            text,
            fields,
            page,
            limit,
            format,
        } => {
            assert_eq!(text, "waka");
            assert_eq!(fields, vec!["category", "collection"]);
            assert_eq!(page, 3);
            assert_eq!(limit, 12);
            assert_eq!(format, dnz_cli::Format::Text);
        }
        other => panic!("expected facets command, got {other:?}"),
    }

    let doctor = dnz_cli::Cli::parse_from(["dnz", "doctor"]);
    assert!(matches!(doctor.command, dnz_cli::Commands::Doctor));
}

#[test]
fn test_cli_parses_gazette_export_command() {
    let cli = dnz_cli::Cli::parse_from([
        "dnz",
        "gazette-export",
        "--output",
        "out/gazette",
        "--text",
        "notice",
        "--start-page",
        "2",
        "--max-pages",
        "3",
        "--limit",
        "50",
        "--sort",
        "date",
        "--direction",
        "desc",
    ]);

    match cli.command {
        dnz_cli::Commands::GazetteExport {
            output,
            text,
            start_page,
            max_pages,
            limit,
            sort,
            direction,
        } => {
            assert_eq!(output, std::path::PathBuf::from("out/gazette"));
            assert_eq!(text, "notice");
            assert_eq!(start_page, 2);
            assert_eq!(max_pages, Some(3));
            assert_eq!(limit, 50);
            assert_eq!(sort, "date");
            assert_eq!(direction, "desc");
        }
        other => panic!("expected gazette-export command, got {other:?}"),
    }
}
