use std::{fs, io::Read};

use anyhow::{bail, Error, Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};

use crate::Process;

use super::utils::{get_reader, verify_file};

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

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let s = blake3::hash(&buf);
        let hash = s.as_bytes();
        Ok(hash == sig)
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = ed25519_dalek::Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Process for TextSubcommand {
    fn process(&self) -> Result<()> {
        match self {
            TextSubcommand::Sign(opts) => {
                let mut reader = get_reader(opts.input.as_str())?;
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;

                let signed = match opts.format {
                    TextSingFormat::Blake3 => {
                        let key = fs::read(&opts.key)?;
                        let key: [u8; 32] = key[..32].try_into()?;
                        let signer = Blake3 { key };
                        signer.sign(&mut reader)?
                    }
                    TextSingFormat::Ed25519 => {
                        todo!("Ed25519")
                    }
                };
                let signed = URL_SAFE_NO_PAD.encode(signed);
                println!("{}", signed);
            }
            TextSubcommand::Verify(opts) => {
                println!("{:?}", opts);
            }
        }
        Ok(())
    }
}
