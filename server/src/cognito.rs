use anyhow::{Context, Result};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct CognitoValidator {
    region: String,
    user_pool_id: String,
    issuer: String,
    jwks: Jwks,
}

impl CognitoValidator {
    pub async fn new(region: &str, user_pool_id: &str) -> Result<Self> {
        let issuer = format!(
            "https://cognito-idp.{}.amazonaws.com/{}",
            region, user_pool_id
        );
        let jwks_url = format!("{}/.well-known/jwks.json", issuer);
        let jwks = fetch_jwks(&jwks_url)
            .await
            .context("Failed to fetch JWKS")?;
        Ok(Self {
            region: region.to_string(),
            user_pool_id: user_pool_id.to_string(),
            issuer,
            jwks,
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenData<Claims>> {
        // Decode the header to get the key id (kid)
        let header = decode_header(token).context("Failed to decode JWT header")?;
        let kid = header.kid.context("Missing 'kid' in JWT header")?;

        // Find the corresponding JWK
        let jwk = self
            .jwks
            .keys
            .iter()
            .find(|&key| key.kid == kid)
            .context("No matching JWK found for 'kid'")?;

        // Convert the JWK to a decoding key
        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .context("Failed to create decoding key from JWK")?;

        // Define validation criteria
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["your_audience_here"]);
        validation.set_issuer(&["your_issuer_here"]);

        // Decode and validate the token
        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .context("Failed to decode and validate JWT")?;

        Ok(token_data)
    }
}

async fn fetch_jwks(jwks_url: &str) -> Result<Jwks> {
    let response = reqwest::get(jwks_url)
        .await
        .context("Failed to fetch JWKS")?;
    let jwks = response
        .json::<Jwks>()
        .await
        .context("Failed to deserialize JWKS response")?;
    Ok(jwks)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct Jwk {
    alg: String,
    e: String,
    kid: String,
    kty: String,
    n: String,
    r#use: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}
