use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsBskyPostUrl {
    pub url_id: i32,
    pub url: String,
    pub expanded_url: String,
    pub expanded_url_parsed: String,
    pub expanded_url_host: String,
    pub display_url: Option<String>,
    pub is_bsky_url: Option<bool>,
    pub is_english: Option<bool>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub preview_image_thumbnail_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub created_at: i64,
    pub created_at_str: String,
}
