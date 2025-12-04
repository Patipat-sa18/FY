use serde::{Deserialize, Serialize};

use crate::domain::value_objects::mission_statuses::MissionStatuses;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MissionFilter {
    pub name: Option<String>,
    pub status: Option<MissionStatuses>,
}
