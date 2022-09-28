use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Direct & Indirect News feed URL references
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedUrlReferences")]
pub struct NewsFeedUrlReferencesQuery {
    pub text: String,
    pub created_at_str: String,
    pub username: String,    
}
