use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::{
    entities::missions::{AddMissionEntity, EditMissionEntity},
    value_objects::mission_statuses::MissionStatuses,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MissionModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub chief_id: i32,
    pub crew_count: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddMissionModel {
    pub name: String,
    pub description: Option<String>,
}

impl AddMissionModel {
    pub fn to_entity(&self, chief_id: i32) -> AddMissionEntity {
        AddMissionEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            status: MissionStatuses::Open.to_string(),
            chief_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditMissionModel {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl EditMissionModel {
    pub fn to_entity(&self, chief_id: i32) -> EditMissionEntity {
        EditMissionEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            chief_id,
        }
    }
}
