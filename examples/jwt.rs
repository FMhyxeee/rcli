use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

fn main() -> Result<()> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");
    claims.insert("aud", "device1");
    claims.insert("exp", "14d");
    let token_str = claims.sign_with_key(&key)?;
    println!("{}", token_str);

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key)?;
    assert_eq!(claims["sub"], "someone");

    Ok(())
}
