use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Direct News feed URL references
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedUrlDirectReference")]
pub struct NewsFeedUrlDirectReferencesQuery {
    pub text: String,
    pub created_at_str: String,
    pub username: String,
}
