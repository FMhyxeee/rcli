use std::{io::Read, str::FromStr};

use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use clap::Parser;

use crate::{process::utils::get_reader, Process};

use super::utils::verify_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, default_value = "standard")]
    format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "url_safe" => Ok(Base64Format::UrlSafe),
            _ => Err("Invalid Base64 encoder engine".to_string()),
        }
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value = "standard")]
    format: Base64Format,
}

impl Process for Base64SubCommand {
    fn process(&self) -> Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input)?;

                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;
                let buf = buf.trim();

                let encode = match opts.format {
                    Base64Format::Standard => general_purpose::STANDARD.encode(buf),
                    Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.encode(buf),
                };
                println!("{}", encode);
            }

            // Decode might be a bit tricky, because we need to know the format of the input
            Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input)?;

                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;

                match opts.format {
                    Base64Format::Standard => {
                        let decoded = general_purpose::STANDARD.decode(buf)?;
                        println!("{}", String::from_utf8_lossy(&decoded));
                    }
                    Base64Format::UrlSafe => {
                        let decoded = general_purpose::URL_SAFE_NO_PAD.decode(buf)?;
                        println!("{}", String::from_utf8_lossy(&decoded));
                    }
                }
            }
        }
        Ok(())
    }
}
