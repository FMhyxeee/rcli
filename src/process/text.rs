use std::{collections::HashMap, fs, io::Read, path::PathBuf};

use anyhow::{bail, Error, Ok, Result};
use clap::Parser;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::Process;

use super::{
    gen_pass::process_genpass,
    utils::{get_reader, verify_file, verify_path},
};

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a key pair")]
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(s: &str) -> Result<TextSignFormat, Error> {
    let format = match s {
        "blake3" => TextSignFormat::Blake3,
        "ed25519" => TextSignFormat::Ed25519,
        _ => bail!("Invalid format"),
    };
    Ok(format)
}

trait TextSigner {
    // signer could sign any input data
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerifier {
    // verify could verify any input data
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let s = blake3::hash(&buf);
        let hash = s.as_bytes();
        Ok(hash == sig)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = ed25519_dalek::Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to [u8; 32]
        let key: [u8; 32] = key[..32].try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl Ed25519Signer {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());

        Ok(map)
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };
    signer.sign(reader)
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::try_new(key)?;
            verifier.verify(reader, sig)
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::try_new(key)?;
            verifier.verify(reader, sig)
        }
    }
}

impl Process for TextSubcommand {
    fn process(&self) -> Result<()> {
        match self {
            TextSubcommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = fs::read(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                println!("{:?}", sig);
            }
            TextSubcommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = fs::read(&opts.key)?;
                let sig = fs::read(&opts.sig)?;
                let result = process_text_verify(&mut reader, &key, &sig, opts.format)?;
                println!("{}", result);
            }
            TextSubcommand::Generate(opts) => {
                let map = match opts.format {
                    TextSignFormat::Blake3 => Blake3::generate()?,
                    TextSignFormat::Ed25519 => Ed25519Signer::generate()?,
                };
                for (path, key) in map {
                    let path = opts.output_path.join(path);
                    fs::write(path, key)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::process::text::process_text_verify;

    use super::TextSignFormat;
    use anyhow::Result;
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

    const KEY: &[u8] = include_bytes!("../../fixtures/blake3.txt");

    #[test]
    #[ignore]
    fn test_process_text_sign() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();

        let format = TextSignFormat::Blake3;
        let sig = super::process_text_sign(&mut reader, KEY, format)?;
        let result = super::process_text_verify(&mut reader1, KEY, &sig, format)?;
        assert!(result);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_process_text_verify() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = "33Ypo4rveYpWmJKAiGnnse-wHQhMVujjmcVkV4Tl43k";
        let sig = URL_SAFE_NO_PAD.decode(sig)?;
        let ret = process_text_verify(&mut reader, KEY, &sig, format)?;
        assert!(ret);
        Ok(())
    }
}
