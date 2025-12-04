use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MissionOperationRepository {
    async fn to_progress(&self, mission_id: i32, chief_id: i32) -> Result<i32>;
    async fn to_completed(&self, mission_id: i32, chief_id: i32) -> Result<i32>;
    async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<i32>;
}
