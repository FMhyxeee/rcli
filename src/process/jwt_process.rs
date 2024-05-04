use std::collections::BTreeMap;

use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;

pub async fn sign(key: &str, sub: &str, aud: &str, exp: &str) -> Result<()> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes())?;
    let mut claims = std::collections::BTreeMap::new();
    claims.insert("sub", sub);
    claims.insert("aud", aud);
    claims.insert("exp", exp);
    let token_str = claims.sign_with_key(&key)?;
    println!("{}", token_str);

    Ok(())
}

pub async fn verify(key: &str, token: &str) -> Result<()> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes())?;

    let token: Token<Header, BTreeMap<String, String>, _> = token.verify_with_key(&key)?;
    let header = token.header();
    let claims = token.claims();
    println!("{:?}", header);
    println!("{:?}", claims);
    Ok(())
}
