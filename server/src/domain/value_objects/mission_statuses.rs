use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum MissionStatuses {
    #[default]
    Open,
    InProgress,
    Completed,
    Failed,
}

impl Display for MissionStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissionStatuses::Open => write!(f, "Open"),
            MissionStatuses::InProgress => write!(f, "InProgress"),
            MissionStatuses::Completed => write!(f, "Completed"),
            MissionStatuses::Failed => write!(f, "Failed"),
        }
    }
}
