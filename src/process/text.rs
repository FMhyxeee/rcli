use anyhow::{bail, Error, Result};
use clap::Parser;

use crate::Process;

use super::utils::verify_file;

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSingFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSingFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSingFormat {
    Blake3,
    Ed25519,
}

fn parse_format(s: &str) -> Result<TextSingFormat, Error> {
    let format = match s {
        "blake3" => TextSingFormat::Blake3,
        "ed25519" => TextSingFormat::Ed25519,
        _ => bail!("Invalid format"),
    };
    Ok(format)
}

impl Process for TextSubcommand {
    fn process(&self) -> Result<()> {
        match self {
            TextSubcommand::Sign(opts) => match opts.format {
                TextSingFormat::Blake3 => {
                    println!("Sign with blake3");
                }
                TextSingFormat::Ed25519 => {
                    println!("Sign with ed25519");
                }
            },
            TextSubcommand::Verify(opts) => {
                println!("{:?}", opts);
            }
        }
        Ok(())
    }
}
