use clap::Parser;

use crate::utils::verify_file;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, default_value = "output")]
    pub output: String,
    #[arg(long, default_value = "json")]
    pub format: OutputFormat,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
    Unknown,
}

impl From<&str> for OutputFormat {
    fn from(s: &str) -> Self {
        match s {
            "json" => OutputFormat::Json,
            "yaml" => OutputFormat::Yaml,
            "toml" => OutputFormat::Toml,
            _ => OutputFormat::Unknown,
        }
    }
}
