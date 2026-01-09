use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedImg {
    pub url: String,
    pub public_id: String,
}

impl UploadedImg {
    pub fn new(url: String, public_id: String) -> Self {
        Self { url, public_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadBase64Img {
    pub base64_string: String,
}
