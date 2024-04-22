use clap::Parser;

use crate::process::{Base64SubCommand, CsvOpts, GenPassOpts};

#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
pub struct CliOpts {
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
}
