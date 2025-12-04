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
        let max_crew_per_mission = std::env::var("MAX_CREW_PER_MISSION")
            .expect("missing value")
            .parse()?;

        let mission = self.mission_viewing_repository.get_one(mission_id).await?;

        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;

        let mission_status_condition = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();
        if !mission_status_condition {
            return Err(anyhow::anyhow!("Mission is not joinable"));
        }
        let crew_count_condition = crew_count < max_crew_per_mission;
        if !crew_count_condition {
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

        let leaving_condition = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();
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
