pub mod authentication_model;
pub mod jwt_model;

use anyhow::{Ok, Result};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub fn generate_token(secret: String, claims: &jwt_model::Claims) -> Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_token(secret: String, token: String) -> Result<jwt_model::Claims> {
    let token = decode::<jwt_model::Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token.claims)
}
