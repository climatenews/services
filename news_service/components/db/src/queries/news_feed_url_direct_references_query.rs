use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Direct News feed URL references
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedUrlDirectReference")]
pub struct NewsFeedUrlDirectReferencesQuery {
    pub text: String,
    pub tweet_id: i64,
    pub author_id: i64,
    pub created_at_str: String,
    pub username: String,
    pub url_id: i32,
}
