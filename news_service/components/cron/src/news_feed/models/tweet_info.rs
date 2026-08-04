// Basic post info used for news feed score calculation
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct TweetInfo {
    pub author_id: String,
    pub created_at: i64,
}
