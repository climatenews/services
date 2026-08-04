// Combined News feed URL references
#[derive(Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsFeedUrlReference {
    pub url_id: i32,
    pub post_uri: String,
    pub post_text: String,
    pub post_created_at_str: String,
    pub author_handle: String,
    pub reposted_by_handles: Vec<String>,
}
