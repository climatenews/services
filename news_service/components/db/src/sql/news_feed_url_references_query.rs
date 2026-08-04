use crate::queries::news_feed_url_references_query::{
    NewsFeedUrlReferenceRepost, NewsFeedUrlReferencesQuery,
};
use sqlx::PgPool;

pub async fn get_news_feed_url_references(
    pool: &PgPool,
    url_slug: String,
) -> Result<Vec<NewsFeedUrlReferencesQuery>, sqlx::Error> {
    sqlx::query_as!(
        NewsFeedUrlReferencesQuery,
        r#"
        SELECT 
            rpu.url_id,
            p.text,
            p.post_uri,
            p.author_did,
            u.handle as "author_handle?",
            p.created_at,
            p.created_at_str
        FROM
            news_bsky_referenced_post_url as rpu 
            JOIN news_bsky_post_url as pu ON pu.url_id = rpu.url_id
            JOIN news_bsky_post as p ON p.post_uri = rpu.post_uri
            JOIN news_feed_url as nfu ON nfu.url_id = rpu.url_id
            LEFT JOIN news_bsky_user as u ON p.author_did = u.did
        WHERE
            nfu.url_slug = $1
            AND pu.is_bsky_url = False
            AND pu.title IS NOT NULL
            AND p.reply_parent_uri IS NULL
        ORDER BY
            p.created_at DESC
        "#,
        url_slug
    )
    .fetch_all(pool)
    .await
}

// Reposted-by handles for a set of post URIs
// news_bsky_reference rows with ref_kind = 'repost' store the reposter DID
// in ref_post_uri, joined here to news_bsky_user for the handle.
pub async fn get_news_feed_url_reposted_by_handles(
    pool: &PgPool,
    post_uris: &Vec<String>,
) -> Result<Vec<NewsFeedUrlReferenceRepost>, sqlx::Error> {
    if post_uris.is_empty() {
        return Ok(vec![]);
    }
    sqlx::query_as!(
        NewsFeedUrlReferenceRepost,
        r#"
        SELECT 
            nbr.post_uri,
            u.handle as "reposted_by_handle?"
        FROM
            news_bsky_reference as nbr
            JOIN news_bsky_user as u ON u.did = nbr.ref_post_uri
        WHERE
            nbr.ref_kind = 'repost'
            AND nbr.post_uri = ANY($1)
        "#,
        post_uris
    )
    .fetch_all(pool)
    .await
}
