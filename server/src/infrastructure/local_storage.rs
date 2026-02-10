use crate::domain::value_objects::{base64_img::Base64Img, uploaded_img::UploadedImg};
use base64::Engine;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub async fn save_locally(base64_image: Base64Img) -> Result<UploadedImg> {
    let uploads_dir = "uploads";
    if !Path::new(uploads_dir).exists() {
        fs::create_dir_all(uploads_dir).context("failed to create uploads directory")?;
    }

    let data = base64_image.into_inner();
    
    // Extract mime type and actual base64 data
    // Format: data:image/png;base64,iVBORw0KGgoAAAANSUhEU...
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() != 2 {
        anyhow::bail!("invalid base64 image format");
    }

    let header = parts[0];
    let base64_data = parts[1];

    let ext = if header.contains("image/png") {
        "png"
    } else if header.contains("image/jpeg") || header.contains("image/jpg") {
        "jpg"
    } else {
        anyhow::bail!("unsupported image type");
    };

    let filename = format!("{}.{}", Uuid::new_v4(), ext);
    let file_path = format!("{}/{}", uploads_dir, filename);

    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_data)
        .context("failed to decode base64 image data")?;

    fs::write(&file_path, decoded_bytes).context("failed to write file to local storage")?;

    let local_url = format!("http://localhost:8000/uploads/{}", filename);

    Ok(UploadedImg {
        url: local_url,
        public_id: filename,
    })
}
