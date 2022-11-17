use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Cron job information
#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsCronJob {
    pub started_at: i64,
    pub started_at_str: String,
    pub completed_at: Option<i64>,
    pub completed_at_str: Option<String>,
    pub error: Option<String>,
}

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsCronJobWithId {
    pub id: i32,
    pub started_at: i64,
    pub started_at_str: String,
    pub completed_at: Option<i64>,
    pub completed_at_str: Option<String>,
    pub error: Option<String>,
}