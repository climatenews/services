use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsBskyReferencedPostUrl {
    pub post_uri: String,
    pub url_id: i32,
}
