use std::fs;

use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

use anyhow::Result;

fn main() -> Result<()> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let key = key.to_vec();
    let nonce = nonce.to_vec();

    let data = key.into_iter().chain(nonce).collect::<Vec<u8>>();
    fs::write("fixtures/chacha20ploy1305.key", data)?;

    Ok(())
}
