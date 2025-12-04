use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl, dsl::delete, insert_into};
use std::sync::Arc;

use crate::{
    domain::{
        entities::crew_memberships::CrewMemberShips,
        repositories::crew_operation::CrewOperationRepository,
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::crew_memberships},
};

pub struct CrewOperationPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewOperationPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewOperationRepository for CrewOperationPostgres {
    async fn join(&self, crew_member_ships: CrewMemberShips) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        insert_into(crew_memberships::table)
            .values(crew_member_ships)
            .execute(&mut conn)?;
        Ok(())
    }

    async fn leave(&self, crew_member_ships: CrewMemberShips) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_member_ships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_member_ships.mission_id))
            .execute(&mut conn)?;
        Ok(())
    }
}
