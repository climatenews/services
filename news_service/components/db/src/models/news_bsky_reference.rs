use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsBskyReference {
    pub post_uri: String,
    pub ref_post_uri: String,
    pub ref_kind: String,
}
