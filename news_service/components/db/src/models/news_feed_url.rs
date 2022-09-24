use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// News feed URL
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsFeedUrl {
    pub url_id: i32,
    pub url_score: i32,
    pub num_references: i32,
    pub created_at: i64,
    pub created_at_str: String,
}
