use crate::domain::{
    entities::crew_memberships::CrewMemberShips,
    repositories::{
        crew_operation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
    },
    value_objects::mission_statuses::MissionStatuses,
};
use anyhow::Result;
use std::sync::Arc;

pub struct CrewOperationUseCase<T1, T2>
where
    T1: CrewOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    crew_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
}

impl<T1, T2> CrewOperationUseCase<T1, T2>
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(crew_operation_repository: Arc<T1>, mission_viewing_repository: Arc<T2>) -> Self {
        Self {
            crew_operation_repository,
            mission_viewing_repository,
        }
    }

    pub async fn join(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let mission = self.mission_viewing_repository.get_one(mission_id).await?;

        if mission.chief_id == brawler_id {
            return Err(anyhow::anyhow!(
                "The Chief can not join in his own mission as a crew member!!"
            ));
        }

        if self
            .mission_viewing_repository
            .is_crew_member(mission_id, brawler_id)
            .await?
        {
            return Err(anyhow::anyhow!("You have already joined this mission!!"));
        }

        if mission.status != MissionStatuses::Open.to_string() {
            return Err(anyhow::anyhow!("Mission is not joinable"));
        }

        if mission.crew_count >= mission.max_crew as i64 {
            return Err(anyhow::anyhow!("Mission is full"));
        }

        self.crew_operation_repository
            .join(CrewMemberShips {
                mission_id,
                brawler_id,
            })
            .await?;

        Ok(())
    }

    pub async fn leave(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let mission = self.mission_viewing_repository.get_one(mission_id).await?;

        let leaving_condition = mission.status == MissionStatuses::Open.to_string();
        if !leaving_condition {
            return Err(anyhow::anyhow!("Mission is not leavable"));
        }
        self.crew_operation_repository
            .leave(CrewMemberShips {
                mission_id,
                brawler_id,
            })
            .await?;

        Ok(())
    }
}
