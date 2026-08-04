use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsBskyUser {
    pub did: String,
    pub handle: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub followers_count: i32,
    pub follows_count: i32,
    pub posts_count: i32,
    pub user_score: Option<i32>,
    pub last_post_cid: Option<String>,
    pub last_updated_at: i64,
    pub last_checked_at: i64,
}
