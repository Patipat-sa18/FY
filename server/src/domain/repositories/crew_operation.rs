use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::crew_memberships::CrewMemberShips;

#[async_trait]
pub trait CrewOperationRepository {
    async fn join(&self, crew_member_ships: CrewMemberShips) -> Result<()>;
    async fn leave(&self, crew_member_ships: CrewMemberShips) -> Result<()>;
}
