use anyhow::{Ok, Result};
use async_trait::async_trait;
use chrono::{Duration, Utc};
// use diesel::{
//     ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, insert_into,
//     query_dsl::methods::{FilterDsl, SelectDsl},
// };
use diesel::{dsl::insert_into, prelude::*};
use diesel::result::{Error, DatabaseErrorKind};
use std::sync::Arc;

use crate::{
    config::config_loader::get_jwt_env,
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        repositories::brawlers::BrawlerRepository,
        value_objects::{
            base64_img::Base64Img,
            mission_model::MissionModel,
            profile_stats::ProfileStats,
            uploaded_img::UploadedImg,
        },
    },
    infrastructure::{
        cloudinary::{self, UploadImageOptions},
        database::{
            postgresql_connection::PgPoolSquad,
            schema::{brawlers, crew_memberships, missions},
        },
        jwt::{
            generate_token,
            jwt_model::{Claims, Passport},
        },
    },
};

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<Passport> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let user_id = match insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)
        {
            std::result::Result::Ok(id) => id,
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                anyhow::bail!("Username already exists");
            }
            Err(e) => return Err(e.into()),
        };

        let display_name = register_brawler_entity.display_name;

        let jwt_env = get_jwt_env()?;
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (Utc::now() + Duration::days(jwt_env.ttl)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };
        let token = generate_token(jwt_env.secret, &claims)?;
        Ok(Passport {
            token,
            display_name,
            avatar_url: None,
            id: user_id,
        })
    }

    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn upload_base64img(
        &self,
        user_id: i32,
        base64img: Base64Img,
        opt: UploadImageOptions,
    ) -> Result<UploadedImg> {
        let uploaded_img = cloudinary::upload(base64img, opt).await?;

        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(user_id))
            .set((
                brawlers::avatar_url.eq(uploaded_img.url.clone()),
                brawlers::avatar_public_id.eq(uploaded_img.public_id.clone()),
            ))
            .execute(&mut conn)?;

        Ok(uploaded_img)
    }

    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        // Use a raw SQL query to select the MissionModel fields including
        // the chief's display name and the crew count.
        let sql = r#"
SELECT
    missions.id,
    missions.name,
    missions.description,
    missions.status,
    missions.difficulty,
    missions.chief_id,
    brawlers.display_name AS chief_display_name,
    (SELECT COUNT(*) FROM crew_memberships WHERE crew_memberships.mission_id = missions.id) AS crew_count,
    missions.max_crew,
    missions.created_at,
    missions.updated_at
FROM missions
LEFT JOIN brawlers ON brawlers.id = missions.chief_id
WHERE missions.deleted_at IS NULL
    AND missions.chief_id = $1
ORDER BY missions.created_at DESC
        "#;

        let results = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(brawler_id)
            .load::<MissionModel>(&mut conn)?;

        Ok(results)
    }

    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .first::<i64>(&mut conn)?;

        let count = u32::try_from(result)?;

        Ok(count)
    }

    async fn get_profile_stats(&self, brawler_id: i32) -> Result<ProfileStats> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        use crate::domain::value_objects::mission_statuses::MissionStatuses;

        // Count missions created by the user
        let created_count = missions::table
            .filter(missions::chief_id.eq(brawler_id))
            .filter(missions::deleted_at.is_null())
            .count()
            .get_result::<i64>(&mut conn)?;

        // Count missions where the user is a crew member
        let joined_count = crew_memberships::table
            .filter(crew_memberships::brawler_id.eq(brawler_id))
            .count()
            .get_result::<i64>(&mut conn)?;

        // Count missions completed (either created or joined) where status is 'Completed'
        // This requires checking both missions table (for created) and crew_memberships joined with missions (for joined)
        
        // Simpler approach: Count completed missions where user is chief OR user is crew member
        // But crew_memberships doesn't have status. We need to join.
        
        let completed_as_chief = missions::table
            .filter(missions::chief_id.eq(brawler_id))
            .filter(missions::status.eq(MissionStatuses::Completed.to_string()))
            .filter(missions::deleted_at.is_null())
            .count()
            .get_result::<i64>(&mut conn)?;
            
        let completed_as_crew = crew_memberships::table
            .inner_join(missions::table)
            .filter(crew_memberships::brawler_id.eq(brawler_id))
            .filter(missions::status.eq(MissionStatuses::Completed.to_string()))
            .filter(missions::deleted_at.is_null())
            .count()
            .get_result::<i64>(&mut conn)?;
            
        // Note: A chief might also be in crew_memberships depending on implementation. 
        // If chief is NOT effectively in crew_memberships for their own mission, we sum them.
        // Assuming chief is separate from crew in this logic based on previous code.
        
        Ok(ProfileStats {
            created_count,
            joined_count,
            completed_count: completed_as_chief + completed_as_crew,
        })
    }

    async fn update_username(&self, brawler_id: i32, new_username: String) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set(brawlers::display_name.eq(new_username))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn find_by_id(&self, brawler_id: i32) -> Result<BrawlerEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let brawler = brawlers::table
            .filter(brawlers::id.eq(brawler_id))
            .first::<BrawlerEntity>(&mut conn)?;

        Ok(brawler)
    }
}
