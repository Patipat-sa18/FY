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
        let pool = Arc::clone(&self.db_pool);

        let (user_id, display_name) = tokio::task::spawn_blocking(move || -> Result<(i32, String)> {
            let mut connection = pool.get().map_err(|e| anyhow::anyhow!(e))?;
            let display_name = register_brawler_entity.display_name.clone();
            
            let id = match insert_into(brawlers::table)
                .values(&register_brawler_entity)
                .returning(brawlers::id)
                .get_result::<i32>(&mut connection)
            {
                std::result::Result::Ok(id) => id,
                Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                    return Err(anyhow::anyhow!("Username already exists"));
                }
                Err(e) => return Err(e.into()),
            };
            
            Ok((id, display_name))
        })
        .await??;

        let jwt_env = tokio::task::spawn_blocking(get_jwt_env).await??;
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
        let pool = Arc::clone(&self.db_pool);

        let result = tokio::task::spawn_blocking(move || -> Result<BrawlerEntity> {
            let mut connection = pool.get().map_err(|e| anyhow::anyhow!(e))?;
            brawlers::table
                .filter(brawlers::username.eq(username))
                .select(BrawlerEntity::as_select())
                .first::<BrawlerEntity>(&mut connection)
                .map_err(|e| anyhow::anyhow!(e))
        })
        .await??;

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
        let pool = Arc::clone(&self.db_pool);

        let results = tokio::task::spawn_blocking(move || -> Result<Vec<MissionModel>> {
            let mut conn = pool.get().map_err(|e| anyhow::anyhow!(e))?;
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

            diesel::sql_query(sql)
                .bind::<diesel::sql_types::Int4, _>(brawler_id)
                .load::<MissionModel>(&mut conn)
                .map_err(|e| anyhow::anyhow!(e))
        })
        .await??;

        Ok(results)
    }

    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let pool = Arc::clone(&self.db_pool);

        let result = tokio::task::spawn_blocking(move || -> Result<i64> {
            let mut conn = pool.get().map_err(|e| anyhow::anyhow!(e))?;
            crew_memberships::table
                .filter(crew_memberships::mission_id.eq(mission_id))
                .count()
                .first::<i64>(&mut conn)
                .map_err(|e| anyhow::anyhow!(e))
        })
        .await??;

        let count = u32::try_from(result)?;

        Ok(count)
    }

    async fn get_profile_stats(&self, brawler_id: i32) -> Result<ProfileStats> {
        let pool = Arc::clone(&self.db_pool);

        let stats = tokio::task::spawn_blocking(move || -> Result<ProfileStats> {
            let mut conn = pool.get().map_err(|e| anyhow::anyhow!(e))?;
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

            Ok(ProfileStats {
                created_count,
                joined_count,
                completed_count: completed_as_chief + completed_as_crew,
            })
        })
        .await??;

        Ok(stats)
    }

    async fn update_username(&self, brawler_id: i32, new_username: String) -> Result<()> {
        let pool = Arc::clone(&self.db_pool);

        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut conn = pool.get().map_err(|e| anyhow::anyhow!(e))?;
            diesel::update(brawlers::table)
                .filter(brawlers::id.eq(brawler_id))
                .set(brawlers::display_name.eq(new_username))
                .execute(&mut conn)?;
            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn update_avatar_url(&self, brawler_id: i32, url: String) -> Result<()> {
        let pool = Arc::clone(&self.db_pool);

        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut conn = pool.get().map_err(|e| anyhow::anyhow!(e))?;
            diesel::update(brawlers::table)
                .filter(brawlers::id.eq(brawler_id))
                .set(brawlers::avatar_url.eq(url))
                .execute(&mut conn)?;
            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn find_by_id(&self, brawler_id: i32) -> Result<BrawlerEntity> {
        let pool = Arc::clone(&self.db_pool);

        let brawler = tokio::task::spawn_blocking(move || -> Result<BrawlerEntity> {
            let mut conn = pool.get().map_err(|e| anyhow::anyhow!(e))?;
            brawlers::table
                .filter(brawlers::id.eq(brawler_id))
                .first::<BrawlerEntity>(&mut conn)
                .map_err(|e| anyhow::anyhow!(e))
        })
        .await??;

        Ok(brawler)
    }
}
