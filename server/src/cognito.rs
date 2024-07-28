use anyhow::{Context, Result};
use jsonwebtoken::{decode, decode_header, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct CognitoValidator {
    region: String,
    user_pool_id: String,
    issuer: String,
    client_id: String,
    jwks: Jwks,
}

impl CognitoValidator {
    pub async fn new(region: &str, user_pool_id: &str, client_id: &str) -> Result<Self> {
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
            client_id: client_id.to_string(),
            jwks,
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenData<Claims>> {
        // Decode the header to get the key id (kid)
        let header = decode_header(token).context("Failed to decode JWT header")?;
        let kid = header.kid.context("Missing 'kid' in JWT header")?;
        let alg = header.alg;

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
        let mut validation = Validation::new(alg);
        validation.set_audience(&[&self.client_id]);
        validation.set_issuer(&[&self.issuer]);

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub username: String,
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

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_cognito_validator() {
        use crate::cognito::CognitoValidator;

        let region = "us-east-1";
        let user_pool_id = "us-east-1_Qbzi9lvVB";
        let client_id = "5p99s5nl7nha5tfnpik3r0rb7j";
        let validator = CognitoValidator::new(region, user_pool_id, client_id)
            .await
            .unwrap();

        let token = "eyJraWQiOiJzTEY0dDBVb05QZmNDY1J1QXZENHVRRWx2bGxVWndBR1I3S0hLVVhpM3pFPSIsImFsZyI6IlJTMjU2In0.eyJzdWIiOiJkNzU1OGQ0Yi0zNGM1LTQyZTEtODVlMi0zOTRhNmFlMDNjZDYiLCJpc3MiOiJodHRwczpcL1wvY29nbml0by1pZHAudXMtZWFzdC0xLmFtYXpvbmF3cy5jb21cL3VzLWVhc3QtMV9RYnppOWx2VkIiLCJ2ZXJzaW9uIjoyLCJjbGllbnRfaWQiOiI1cDk5czVubDduaGE1dGZucGlrM3IwcmI3aiIsIm9yaWdpbl9qdGkiOiI5ZTM2NjhlNC1mYzQzLTQxNTktYThlMi1mZDViNzEyNDgzYjMiLCJ0b2tlbl91c2UiOiJhY2Nlc3MiLCJzY29wZSI6ImF3cy5jb2duaXRvLnNpZ25pbi51c2VyLmFkbWluIG9wZW5pZCBwcm9maWxlIiwiYXV0aF90aW1lIjoxNzIxNDYyNjQzLCJleHAiOjE3MjE0NjYyNDMsImlhdCI6MTcyMTQ2MjY0MywianRpIjoiNjIxYTcyNWEtMWZiZC00NzIyLTg4YzQtZDk4NTBlYWUwNzcwIiwidXNlcm5hbWUiOiJtaWd1b2xpYW5nIn0.rVtHAWfpZr5-oIswCHbpHGeUzAzxQwFbgIjDEjAmA7tvaRDticn95n1amWt0B_946EgN_HyTMkQ6YRX1Muifu15Q60Y3yxDcZ0qG2UAMqthgf-XmyPPd4l9BfadufDzxDvGLan4TC81_OAZQyW6tui7_lQwAI71vf2DNcJQMuXJJkzFSftX0dQURs3mi9Uzn6kf44IWj_RLKHkJDFuBmiOuwENx2AvzGHla9J-VHDmv29Qr63NN6o2Squ1RiRmLmO0UTsnUuqlB1bVf2AE47ZsneISFCPbbbmJSH7P7qYYi35_wEDjCLd2B53yXrSOco0WRFcFlXdprfh2KAu2mIgg";

        let result = validator.verify_token(&token);
        assert!(result.is_ok());
    }
}
