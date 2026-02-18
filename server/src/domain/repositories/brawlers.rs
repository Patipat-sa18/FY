use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        value_objects::{
            base64_img::Base64Img,
            mission_model::MissionModel,
            profile_stats::ProfileStats,
            uploaded_img::UploadedImg,
        },
    },
    infrastructure::{cloudinary::UploadImageOptions, jwt::jwt_model::Passport},
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<Passport>;
    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity>;
    async fn upload_base64img(
        &self,
        user_id: i32,
        base64img: Base64Img,
        opt: UploadImageOptions,
    ) -> Result<UploadedImg>;

    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>>;
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
    async fn get_profile_stats(&self, brawler_id: i32) -> Result<ProfileStats>;
    async fn update_username(&self, brawler_id: i32, new_username: String) -> Result<()>;
    async fn update_avatar_url(&self, brawler_id: i32, url: String) -> Result<()>;
    async fn find_by_id(&self, brawler_id: i32) -> Result<BrawlerEntity>;
}
