use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileStats {
    pub created_count: i64,
    pub joined_count: i64,
    pub completed_count: i64,
}
