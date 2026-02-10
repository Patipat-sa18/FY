use crate::{
    config::{config_loader::get_cloudinary_env, config_model::CloudinaryEnv},
    domain::value_objects::{base64_img::Base64Img, uploaded_img::UploadedImg},
};
use anyhow::{Context, Ok, Result};
use chrono::Utc;
use reqwest::multipart::{Form, Part};
use sha1::{Digest, Sha1};
use std::collections::HashMap;

pub struct UploadImageOptions {
    pub folder: Option<String>,
    pub public_id: Option<String>,
    pub transformation: Option<String>,
}

fn form_builder(option: UploadImageOptions, cloud_env: &CloudinaryEnv) -> Result<Form> {
    let mut form = Form::new();
    let timestamp = Utc::now().timestamp_millis().to_string();

    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("timestamp".to_string(), timestamp.clone());
    
    if let Some(folder_name) = option.folder {
        params.insert("folder".to_string(), folder_name);
    }
    if let Some(public_id) = option.public_id {
        params.insert("public_id".to_string(), public_id);
    }
    if let Some(transformation) = option.transformation {
        params.insert("transformation".to_string(), transformation);
    }

    // Sort parameters alphabetically for signature calculation
    let mut sorted_keys: Vec<_> = params.keys().collect();
    sorted_keys.sort();

    let signature_string: String = sorted_keys
        .iter()
        .map(|&key| format!("{}={}", key, params.get(key).unwrap()))
        .collect::<Vec<_>>()
        .join("&");

    let mut hasher = Sha1::new();
    hasher.update(format!("{}{}", signature_string, cloud_env.api_secret));
    let signature = format!("{:x}", hasher.finalize());

    // Add all parameters to the form, including resource_type which is NOT signed
    for (key, value) in params {
        form = form.text(key, value);
    }
    form = form.text("resource_type", "image");
    form = form.text("signature", signature);
    form = form.text("api_key", cloud_env.api_key.clone());

    Ok(form)
}

pub async fn upload(base64_image: Base64Img, option: UploadImageOptions) -> Result<UploadedImg> {
    let cloud_env = get_cloudinary_env();

    // Fallback to local storage if Cloudinary environment variables are missing OR if they are placeholder values
    let is_configured = match &cloud_env {
        std::result::Result::Ok(env) => env.cloud_name != "demo" && env.api_key != "123456789",
        Err(_) => false,
    };

    if !is_configured {
        return super::local_storage::save_locally(base64_image).await;
    }

    let cloud_env = cloud_env.unwrap(); // Safe due to is_configured check
    let file = Part::text(base64_image.into_inner());
    let form = form_builder(option, &cloud_env)?;
    let multipart = form.part("file", file);
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.cloudinary.com/v1_1/{}/image/upload",
        cloud_env.cloud_name
    );

    let response = client
        .post(&url)
        .multipart(multipart)
        .send()
        .await
        .context(format!("upload to {}", url))?;

    let text = response.text().await?;
    let json: UploadedImg =
        serde_json::from_str(&text).context(format!("failed to parse:\n\n {}", text))?;
    Ok(json)
}
