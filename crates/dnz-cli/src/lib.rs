//! CLI library mapping for DigitalNZ query parsers.

use clap::{Parser, Subcommand, ValueEnum};
use std::{env, fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "dnz", author, version, about = "DigitalNZ API Integration CLI")]
pub struct Cli {
    /// Verbose logging output directed to stderr.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// DigitalNZ API Key. Defaults to reading the DIGITALNZ_API_KEY env variable.
    #[arg(short, long, global = true)]
    pub api_key: Option<String>,

    /// SQLite cache file. Defaults to DNZ_CACHE_PATH when set.
    #[arg(long, global = true, env = "DNZ_CACHE_PATH")]
    pub cache_path: Option<PathBuf>,

    /// Log output format.
    #[arg(long, global = true, value_enum, default_value_t = LogFormat::Text)]
    pub log_format: LogFormat,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search DigitalNZ records.
    Search {
        /// Text keyword to search for.
        text: String,

        /// Result page index.
        #[arg(short, long, default_value_t = 1)]
        page: u32,

        /// Count of records per page.
        #[arg(short, long, default_value_t = 20)]
        limit: u32,

        /// Output format.
        #[arg(short, long, value_enum, default_value_t = Format::Markdown)]
        format: Format,

        /// Field name to sort by (e.g. date, title, category).
        #[arg(short, long)]
        sort: Option<String>,

        /// Sort direction.
        #[arg(short, long, default_value = "asc")]
        direction: String,

        /// Geographical bounding box coordinates (North,West,South,East).
        #[arg(long, value_parser = parse_bbox, allow_hyphen_values = true)]
        bbox: Option<[f64; 4]>,

        /// Filter in field:value format (can be specified multiple times).
        #[arg(long = "and")]
        and_filters: Vec<String>,

        /// Filter in field:value format (can be specified multiple times).
        #[arg(long = "or")]
        or_filters: Vec<String>,
    },

    /// Harvest facet terms.
    Facets {
        /// Text keyword query context.
        text: String,

        /// Comma-separated fields to facet by (e.g. category,collection).
        #[arg(long, value_delimiter = ',')]
        fields: Vec<String>,

        /// Facet term page.
        #[arg(short, long, default_value_t = 1)]
        page: u32,

        /// Max number of facet terms returned per page.
        #[arg(short, long, default_value_t = 10)]
        limit: u32,

        /// Output format.
        #[arg(short, long, value_enum, default_value_t = Format::Markdown)]
        format: Format,
    },

    /// Fetch metadata for one DigitalNZ record.
    Record {
        /// DigitalNZ record identifier.
        id: String,

        /// Comma-separated fields to request.
        #[arg(long, value_delimiter = ',')]
        fields: Vec<String>,

        /// Output format.
        #[arg(short, long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },

    /// Fetch records related to one DigitalNZ record.
    MoreLikeThis {
        /// DigitalNZ record identifier.
        id: String,

        /// Result page index.
        #[arg(short, long, default_value_t = 1)]
        page: u32,

        /// Count of records per page.
        #[arg(short, long, default_value_t = 20)]
        limit: u32,

        /// Comma-separated fields to request.
        #[arg(long, value_delimiter = ',')]
        fields: Vec<String>,

        /// Output format.
        #[arg(short, long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },

    /// Export New Zealand Gazette records as raw pages, JSONL records, and a manifest.
    GazetteExport {
        /// Output directory for records.jsonl, manifest.json, and raw pages.
        #[arg(short, long)]
        output: PathBuf,

        /// Optional keyword query within New Zealand Gazette records.
        #[arg(long, default_value = "")]
        text: String,

        /// First result page to export.
        #[arg(long, default_value_t = 1)]
        start_page: u32,

        /// Maximum number of pages to export.
        #[arg(long)]
        max_pages: Option<u32>,

        /// Count of records per page.
        #[arg(short, long, default_value_t = 100)]
        limit: u32,

        /// Field name to sort by for deterministic paging.
        #[arg(long, default_value = "date")]
        sort: String,

        /// Sort direction.
        #[arg(long, default_value = "asc")]
        direction: String,
    },

    /// Verification check.
    Ping,

    /// Check local workspace prerequisites and common Windows path issues.
    Doctor,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Json,
    Markdown,
    Text,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogFormat {
    Text,
    Json,
}

pub fn parse_bbox(s: &str) -> Result<[f64; 4], String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 4 {
        return Err("Bounding box must contain exactly 4 comma-separated floats".to_string());
    }
    let n = parts[0].parse::<f64>().map_err(|e| e.to_string())?;
    let w = parts[1].parse::<f64>().map_err(|e| e.to_string())?;
    let s_val = parts[2].parse::<f64>().map_err(|e| e.to_string())?;
    let e = parts[3].parse::<f64>().map_err(|e| e.to_string())?;
    Ok([n, w, s_val, e])
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticCheck {
    pub name: &'static str,
    pub ok: bool,
    pub detail: String,
}

pub fn workspace_diagnostics() -> Vec<DiagnosticCheck> {
    vec![
        check_workspace_path(),
        check_target_write(),
        check_api_key(),
        check_path_tool("cargo"),
        check_path_tool("rustc"),
        check_path_tool("link.exe"),
    ]
}

fn check_workspace_path() -> DiagnosticCheck {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let display = cwd.display().to_string();
    let has_space = display.contains(' ');
    let is_onedrive = display.to_ascii_lowercase().contains("onedrive");
    DiagnosticCheck {
        name: "workspace_path",
        ok: !has_space,
        detail: if has_space {
            format!("path contains spaces: {display}")
        } else if is_onedrive {
            format!("path is under OneDrive: {display}")
        } else {
            display
        },
    }
}

fn check_target_write() -> DiagnosticCheck {
    let target_dir = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target"));
    let probe_dir = target_dir.join(".dnz-doctor");
    let probe_file = probe_dir.join("write-test.tmp");
    let write_result = fs::create_dir_all(&probe_dir)
        .and_then(|_| fs::write(&probe_file, b"ok"))
        .and_then(|_| fs::remove_file(&probe_file));
    DiagnosticCheck {
        name: "target_write",
        ok: write_result.is_ok(),
        detail: write_result
            .map(|_| format!("writable: {}", target_dir.display()))
            .unwrap_or_else(|err| format!("not writable: {} ({err})", target_dir.display())),
    }
}

fn check_api_key() -> DiagnosticCheck {
    let present = env::var_os("DIGITALNZ_API_KEY").is_some();
    DiagnosticCheck {
        name: "digitalnz_api_key",
        ok: true,
        detail: if present {
            "DIGITALNZ_API_KEY is set".to_string()
        } else {
            "DIGITALNZ_API_KEY is not set; live API commands need --api-key or env".to_string()
        },
    }
}

fn check_path_tool(tool: &'static str) -> DiagnosticCheck {
    let found = find_on_path(tool);
    DiagnosticCheck {
        name: match tool {
            "cargo" => "cargo_on_path",
            "rustc" => "rustc_on_path",
            "link.exe" => "linker_on_path",
            _ => "tool_on_path",
        },
        ok: found.is_some(),
        detail: found
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| format!("{tool} not found on PATH")),
    }
}

fn find_on_path(tool: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths).find_map(|dir| {
        let candidate = dir.join(tool);
        if candidate.is_file() {
            Some(candidate)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bbox_valid() {
        let result = parse_bbox("1.0,2.0,3.0,4.0");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_parse_bbox_invalid_count() {
        let result = parse_bbox("1,2,3");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 4"));
    }

    #[test]
    fn test_parse_bbox_invalid_number() {
        let result = parse_bbox("1,abc,3,4");
        assert!(result.is_err());
    }

    #[test]
    fn test_workspace_diagnostics_returns_checks() {
        let checks = workspace_diagnostics();

        let names: Vec<&str> = checks.iter().map(|c| c.name).collect();
        assert!(names.contains(&"workspace_path"));
        assert!(names.contains(&"target_write"));
        assert!(names.contains(&"digitalnz_api_key"));
        assert!(names.contains(&"cargo_on_path"));
        assert!(names.contains(&"rustc_on_path"));
        assert!(names.contains(&"linker_on_path"));
        assert_eq!(checks.len(), 6);
    }

    #[test]
    fn test_cli_parses_global_cache_and_json_logging() {
        let cli = Cli::parse_from([
            "dnz",
            "--cache-path",
            "cache.sqlite",
            "--log-format",
            "json",
            "ping",
        ]);

        assert_eq!(cli.cache_path, Some(PathBuf::from("cache.sqlite")));
        assert_eq!(cli.log_format, LogFormat::Json);
        assert!(matches!(cli.command, Commands::Ping));
    }
}
