use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::error::Error;
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Jwk {
    kty: String,
    use_: String,
    kid: String,
    x5c: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Jwks {
    keys: Vec<Jwk>,
}

pub async fn get_jwks(jwks_url: &str) -> Result<Jwks, Box<dyn Error>> {
    let res = reqwest::get(jwks_url).await?;
    let jwks = res.json::<Jwks>().await?;
    Ok(jwks)
}