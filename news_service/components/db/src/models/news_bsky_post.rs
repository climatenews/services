use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsBskyPost {
    pub post_uri: String,
    pub cid: String,
    pub text: String,
    pub author_did: String,
    pub reply_parent_uri: Option<String>,
    pub reply_root_uri: Option<String>,
    pub created_at: i64,
    pub created_at_str: String,
}
