use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsBskyFeedSource {
    pub source_uri: String,
    pub source_type: String,
    pub last_checked_at: i64,
}
