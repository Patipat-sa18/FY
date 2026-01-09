use diesel::{
    prelude::QueryableByName,
    sql_types::{Integer, VarChar},
};
use serde::{Deserialize, Serialize};

use crate::domain::entities::brawlers::RegisterBrawlerEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterBrawlerModel {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

impl RegisterBrawlerModel {
    pub fn to_entity(&self) -> RegisterBrawlerEntity {
        RegisterBrawlerEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            display_name: self.display_name.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct BrawlerModel {
    #[diesel(sql_type=VarChar)]
    pub display_name: String,
    #[diesel(sql_type=VarChar)]
    pub avatar_url: String,
    #[diesel(sql_type=Integer)]
    pub mission_success_count: i32,
    #[diesel(sql_type=Integer)]
    pub mission_join_count: i32,
}
