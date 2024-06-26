use std::{collections::HashMap, io::Read};

use anyhow::{bail, Ok, Result};
use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use super::gen_pass::process_genpass;
use crate::cli::TextSignFormat;

pub trait TextSigner {
    // signer could sign any input data
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    // verify could verify any input data
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
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
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes() == sig)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (sig[..]).try_into()?;
        let signature = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
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
    key: &[u8], // (ptr, length)
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
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    verifier.verify(reader, sig)
}

pub fn process_text_key_generate(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

pub struct ChaCha20Poly1305DD;

impl ChaCha20Poly1305DD {
    pub fn encrypt(key_reader: &mut dyn Read, content: &mut dyn Read) -> Result<Vec<u8>> {
        // 256-bits key
        // 96-bits nonce

        let mut key_buf = Vec::new();
        key_reader.read_to_end(&mut key_buf)?;

        // check the length of buf
        if key_buf.len() != (32 + 12) {
            bail!("The length of input data is too short");
        }

        let key = &key_buf[..32];
        let nonce = &key_buf[32..32 + 12];

        let key = GenericArray::from_slice(key);
        let nonce = GenericArray::from_slice(nonce);

        let mut content_buf = Vec::new();
        content.read_to_end(&mut content_buf)?;

        let cipher = ChaCha20Poly1305::new(key);
        let ciphertext = cipher.encrypt(nonce, content_buf.as_slice()).unwrap();
        Ok(ciphertext)
    }

    pub fn decrypt(key_reader: &mut dyn Read, content: &mut dyn Read) -> Result<Vec<u8>> {
        // 256-bits key
        // 96-bits nonce

        let mut key_buf = Vec::new();
        key_reader.read_to_end(&mut key_buf)?;

        // check the length of buf
        if key_buf.len() != (32 + 12) {
            bail!("The length of input data is too short");
        }

        let key = &key_buf[..32];
        let nonce = &key_buf[32..32 + 12];

        let key = GenericArray::from_slice(key);
        let nonce = GenericArray::from_slice(nonce);

        let cipher = ChaCha20Poly1305::new(key);

        let mut content_buf = Vec::new();
        content.read_to_end(&mut content_buf)?;

        let plaintext = cipher.decrypt(nonce, content_buf.as_slice()).unwrap();
        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {

    use crate::process::text::process_text_verify;

    use super::TextSignFormat;
    use anyhow::Result;
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    const KEY: &[u8] = include_bytes!("../../fixtures/blake3.txt");
    const CHACHA20_POLY1305_DD: &[u8] = include_bytes!("../../fixtures/chacha20ploy1305.key");

    #[test]
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
    fn test_process_text_verify() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = "33Ypo4rveYpWmJKAiGnnse-wHQhMVujjmcVkV4Tl43k";
        let sig = URL_SAFE_NO_PAD.decode(sig)?;
        let ret = process_text_verify(&mut reader, KEY, &sig, format)?;
        assert!(ret);
        Ok(())
    }

    #[test]
    #[allow(clippy::useless_asref)]
    fn test_encrypt_decrypt() -> Result<()> {
        let key = CHACHA20_POLY1305_DD;
        let content = "hello".as_bytes();

        let ciphertext =
            super::ChaCha20Poly1305DD::encrypt(&mut key.as_ref(), &mut content.as_ref())?;
        let plaintext =
            super::ChaCha20Poly1305DD::decrypt(&mut key.as_ref(), &mut ciphertext.as_slice())?;
        assert_eq!(content, plaintext.as_slice());
        Ok(())
    }
}
