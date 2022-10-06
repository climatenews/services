use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// News feed URL references
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedUrlReference")]
pub struct NewsFeedUrlReferencesQuery {
    pub url_id: i32,
    pub text: String,
    pub tweet_id: i64,
    pub author_id: i64,
    pub created_at_str: String,
    pub username: String,
    pub referenced_tweet_kind: String, // should be optional?
}
