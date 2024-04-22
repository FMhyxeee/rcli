use std::io::Read;

use base64::{engine::general_purpose, Engine};
use clap::Parser;
use anyhow::Result;

use crate::Process;

use super::varify_input_file;





#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = varify_input_file, default_value = "-")]
    pub input: String,

}


#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = varify_input_file, default_value = "-")]
    pub input: String,

}

impl Process for Base64SubCommand {
    fn process(&self) -> Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => {
                // if the input is "-", we should read from stdin
                if opts.input == "-" {
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer)?;
                    let encoded = general_purpose::STANDARD.encode(buffer.as_bytes());
                    println!("{}", encoded);
                } else {
                    let encoded = general_purpose::STANDARD.encode(&opts.input);
                    println!("{}", encoded);
                }
                let encoded = general_purpose::STANDARD.encode(&opts.input);
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = general_purpose::STANDARD.decode(&opts.input)?;
                println!("{}", String::from_utf8(decoded)?);
            }
        }
        Ok(())
    }
}
