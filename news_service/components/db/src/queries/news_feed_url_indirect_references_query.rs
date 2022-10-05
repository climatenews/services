use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Indirect News feed URL references
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedUrlIndirectReference")]
pub struct NewsFeedUrlIndirectReferencesQuery {
    pub text: String,
    pub referenced_tweet_text: String,
    pub referenced_tweet_id: i64,
    pub referenced_tweet_kind: String,
    pub created_at_str: String,
    pub username: String,
}
