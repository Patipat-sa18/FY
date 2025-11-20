use crate::infrastructure::database::schema::brawlers;
use chrono::NaiveDateTime;
use diesel::{Selectable, prelude::*};

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = brawlers)]
pub struct BrawlerEntity {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = brawlers)]
pub struct RegisterBrawlerEntity {
    pub username: String,
    pub password: String,
}
