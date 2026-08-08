use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// News feed URL
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsFeedUrl {
    pub url_slug: String,
    pub url_id: i32,
    pub url_score: i32,
    pub num_references: i32,
    pub first_referenced_by: String,
    pub is_climate_related: Option<bool>,
    pub created_at: i64,
    pub created_at_str: String,
    pub updated_at: i64,
    pub bsky_posted_at: Option<i64>,
    pub bsky_posted_at_str: Option<String>,
}

// News feed URL - Url Slug only
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsFeedUrlSlug {
    pub url_slug: String,
}

// Metadata used by the static-site build to decide whether a rebuild is needed.
// Aggregate expressions are inferred as nullable by sqlx; the API layer normalizes to 0.
#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsFeedBuildMetadata {
    pub updated_at: Option<i64>,
    pub total_count: Option<i64>,
}
