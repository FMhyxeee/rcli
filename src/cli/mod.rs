mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use clap::Parser;
use std::path::{Path, PathBuf};

pub use base64::*;
pub use csv::*;
pub use genpass::*;
pub use http::*;
pub use text::*;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert csv to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "text sign/verify")]
    Text(TextSubcommand),
    #[command(about = "http server")]
    Http(HttpServeOpts),
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if the input is "-", we should read from stdin
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("The input file does not exist")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("The path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use crate::verify_file;

    #[test]
    fn test_varify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(
            verify_file("input.csv"),
            Err("The input file does not exist")
        );
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
    }
}
