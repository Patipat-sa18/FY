use crate::{
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_objects::{
            base64_img::Base64Img, brawler_model::RegisterBrawlerModel, uploaded_img::UploadedImg,
        },
    },
    infrastructure::{argon2::hash, cloudinary::UploadImageOptions, jwt::jwt_model::Passport},
};
use anyhow::{Ok, Result};
use std::sync::Arc;

pub struct BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn register(
        &self,
        mut register_brawler_model: RegisterBrawlerModel,
    ) -> Result<Passport> {
        let hashed_password = hash(register_brawler_model.password.clone())?;

        register_brawler_model.password = hashed_password;

        let register_entity = register_brawler_model.to_entity();

        let passport = self.brawler_repository.register(register_entity).await?;

        Ok(passport)
    }

    pub async fn upload_base64img(
        &self,
        user_id: i32,
        base64string: String,
    ) -> Result<UploadedImg> {
        let opt = UploadImageOptions {
            folder: Some("avatar".to_string()),
            public_id: Some(user_id.to_string()),
            transformation: Some("c_scale,w_256".to_string()),
        };

        let base64img = Base64Img::new(base64string)?;

        let uploaded = self
            .brawler_repository
            .upload_base64img(user_id, base64img, opt)
            .await?;

        Ok(uploaded)
    }
}
