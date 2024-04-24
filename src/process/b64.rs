use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use std::io::Read;

use crate::{
    cli::{Base64Format, Base64SubCommand},
    utils::get_reader,
    Process,
};

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
