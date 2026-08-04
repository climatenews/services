use crate::models::news_feed_url_reference::NewsFeedUrlReference;
use crate::sql::news_feed_url_references_query::{
    get_news_feed_url_references, get_news_feed_url_reposted_by_handles,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use std::collections::HashMap;

// News feed URL references
#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsFeedUrlReferencesQuery {
    pub url_id: i32,
    pub text: String,
    pub post_uri: String,
    pub author_did: String,
    pub author_handle: Option<String>,
    pub created_at: i64,
    pub created_at_str: String,
}

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsFeedUrlReferenceRepost {
    pub post_uri: String,
    pub reposted_by_handle: Option<String>,
}

// Assemble the GraphQL NewsFeedUrlReference list for a news feed URL
pub async fn get_news_feed_url_references_with_metadata(
    pool: &PgPool,
    url_slug: String,
) -> Result<Vec<NewsFeedUrlReference>, sqlx::Error> {
    let news_feed_url_references = get_news_feed_url_references(pool, url_slug).await?;

    let post_uris: Vec<String> = news_feed_url_references
        .iter()
        .map(|r| r.post_uri.clone())
        .collect();
    let reposted_by = get_news_feed_url_reposted_by_handles(pool, &post_uris).await?;

    let mut reposted_by_map: HashMap<String, Vec<String>> = HashMap::new();
    for repost in reposted_by {
        if let Some(handle) = repost.reposted_by_handle {
            reposted_by_map
                .entry(repost.post_uri)
                .or_default()
                .push(format!("@{}", handle));
        }
    }

    Ok(news_feed_url_references
        .into_iter()
        .map(|news_feed_url_reference| NewsFeedUrlReference {
            url_id: news_feed_url_reference.url_id,
            post_uri: news_feed_url_reference.post_uri.clone(),
            post_text: news_feed_url_reference.text.clone(),
            post_created_at_str: news_feed_url_reference.created_at_str.clone(),
            author_handle: news_feed_url_reference
                .author_handle
                .unwrap_or(news_feed_url_reference.author_did.clone()),
            reposted_by_handles: reposted_by_map
                .remove(&news_feed_url_reference.post_uri)
                .unwrap_or_default(),
        })
        .collect())
}
