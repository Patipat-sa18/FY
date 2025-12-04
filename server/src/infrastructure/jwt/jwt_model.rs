use serde::{Deserialize, Serialize};

pub struct Passport {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}
