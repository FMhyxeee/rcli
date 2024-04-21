use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, author = "hyx", long_about = None)]
pub struct CliOpts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert csv to other formats")]
    Csv(CsvOpts),
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

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = varify_input_file)]
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

fn varify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File not found, please check your file if exist!")
    }
}
