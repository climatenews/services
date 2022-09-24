use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Direct referenced tweet URLS
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedUrl")]
pub struct NewsFeedUrlQuery {
    pub url_id: i32,
    pub url_score: i32,
    pub num_references: i32,
    pub created_at: i64,
    pub title: Option<String>,
    // TODO make non-null
    pub description: Option<String>,
    pub parsed_expanded_url: String,
}
