use crate::domain::entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity>;
}
