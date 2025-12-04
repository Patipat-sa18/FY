use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::missions::{AddMissionEntity, EditMissionEntity};

#[async_trait]
pub trait MissionManagementRepository {
    async fn add(&self, add_mission_entity: AddMissionEntity) -> Result<i32>;
    async fn edit(&self, mission_id: i32, edit_mission_entity: EditMissionEntity) -> Result<i32>;
    async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()>;
}
