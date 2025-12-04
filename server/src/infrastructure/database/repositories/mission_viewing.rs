use std::sync::Arc;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    domain::{
        entities::missions::MissionEntity, repositories::mission_viewing::MissionViewingRepository,
        value_objects::mission_filter::MissionFilter,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad,
        schema::{crew_memberships, missions},
    },
};
pub struct MissionViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionViewingRepository for MissionViewingPostgres {
    async fn crew_counting(&self, mission_id: i32) -> Result<i64> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let value = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .first::<i64>(&mut conn)?;

        let count = i64::try_from(value)?;
        Ok(count)
    }

    async fn get_one(&self, mission_id: i32) -> Result<MissionEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = missions::table
            .filter(missions::id.eq(mission_id))
            .filter(missions::deleted_at.is_null())
            .select(MissionEntity::as_select())
            .first::<MissionEntity>(&mut conn)?;

        Ok(result)
    }

    async fn get_all(&self, mission_filter: &MissionFilter) -> Result<Vec<MissionEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let mut query = missions::table
            .filter(missions::deleted_at.is_null())
            .into_boxed();

        if let Some(status) = &mission_filter.status {
            let status_string = status.to_string();
            query = query.filter(missions::status.eq(status_string));
        };
        if let Some(name) = &mission_filter.name {
            query = query.filter(missions::name.ilike(format!("%{}%", name)));
        };

        let value = query
            .select(MissionEntity::as_select())
            .order_by(missions::created_at.desc())
            .load::<MissionEntity>(&mut conn)?;

        Ok(value)
    }
}
