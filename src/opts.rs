use clap::Parser;

use crate::process::{CsvOpts, GenPassOpts};

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
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassOpts),
}
