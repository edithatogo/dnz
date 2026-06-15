//! CLI library mapping for DigitalNZ query parsers.

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "dnz", author, version, about = "DigitalNZ API Integration CLI")]
pub struct Cli {
    /// Verbose logging output directed to stderr.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// DigitalNZ API Key. Defaults to reading the DIGITALNZ_API_KEY env variable.
    #[arg(short, long, global = true)]
    pub api_key: Option<String>,

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
        #[arg(long, value_parser = parse_bbox)]
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
        #[arg(short, long, value_delimiter = ',')]
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

    /// Verification check.
    Ping,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Json,
    Markdown,
    Text,
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
