use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Direct & Indirect News feed URL shares
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedUrlShares")]
pub struct NewsFeedUrlSharesQuery {
    pub text: String,
    pub created_at_str: String,
    pub username: String,    
}
