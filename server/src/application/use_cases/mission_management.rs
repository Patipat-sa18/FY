use std::sync::Arc;

use crate::domain::{
    repositories::{
        mission_management::MissionManagementRepository, mission_viewing::MissionViewingRepository,
    },
    value_objects::mission_model::{AddMissionModel, EditMissionModel},
};

pub struct MissionManagementUseCase<T1, T2>
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    mission_management_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
}

use anyhow::Result;
impl<T1, T2> MissionManagementUseCase<T1, T2>
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(
        mission_management_repository: Arc<T1>,
        mission_viewing_repository: Arc<T2>,
    ) -> Self {
        Self {
            mission_management_repository,
            mission_viewing_repository,
        }
    }

    pub async fn add(&self, chief_id: i32, add_mission_model: AddMissionModel) -> Result<i32> {
        let insert_mission_entity = add_mission_model.to_entity(chief_id);

        let result = self
            .mission_management_repository
            .add(insert_mission_entity)
            .await?;

        Ok(result)
    }

    pub async fn edit(
        &self,
        mission_id: i32,
        chief_id: i32,
        edit_mission_model: EditMissionModel,
    ) -> Result<i32> {
        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;
        if crew_count > 0 {
            return Err(anyhow::anyhow!(
                "Mission has been taken by brawler for now!"
            ));
        }

        let edit_mission_entity = edit_mission_model.to_entity(chief_id);

        let result = self
            .mission_management_repository
            .edit(mission_id, edit_mission_entity)
            .await?;

        Ok(result)
    }

    pub async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;
        if crew_count > 0 {
            return Err(anyhow::anyhow!(
                "Mission has been taken by brawler for now!"
            ));
        }

        self.mission_management_repository
            .remove(mission_id, chief_id)
            .await?;
        Ok(())
    }
}
