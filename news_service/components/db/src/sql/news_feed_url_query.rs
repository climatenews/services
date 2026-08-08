use crate::queries::news_feed_url_query::NewsFeedUrlQuery;
use anyhow::Result;
use sqlx::PgPool;

pub async fn get_news_feed_urls(
    pool: &PgPool,
    timestamp: i64,
    limit: i64,
) -> Result<Vec<NewsFeedUrlQuery>, sqlx::Error> {
    sqlx::query_as!(
        NewsFeedUrlQuery,
        r#"
        SELECT
            nfu.url_slug, 
            nfu.url_id, 
            nfu.url_score,
            nfu.num_references,
            nfu.bsky_posted_at,
            u.handle as first_referenced_by_username,
            nfu.created_at,
            nfu.updated_at,
            pu.title,
            pu.description,
            pu.expanded_url_parsed,
            pu.expanded_url_host,
            pu.display_url,
            pu.preview_image_thumbnail_url,
            pu.preview_image_url
            
        FROM
            news_feed_url as nfu
            JOIN news_bsky_post_url as pu ON pu.url_id = nfu.url_id
            JOIN news_bsky_user as u ON u.did = nfu.first_referenced_by
        WHERE
            nfu.created_at > $1  
            AND nfu.is_climate_related = True
            AND pu.title IS NOT NULL
        ORDER BY
            url_score DESC
        LIMIT $2 
     "#,
        timestamp,
        limit
    )
    .fetch_all(pool)
    .await
}
pub async fn get_news_feed_url(
    pool: &PgPool,
    url_slug: String,
) -> Result<NewsFeedUrlQuery, sqlx::Error> {
    sqlx::query_as!(
        NewsFeedUrlQuery,
        r#"
        SELECT
            nfu.url_slug, 
            nfu.url_id, 
            nfu.url_score,
            nfu.num_references,
            nfu.bsky_posted_at,
            u.handle as first_referenced_by_username,
            nfu.created_at,
            nfu.updated_at,
            pu.title,
            pu.description,
            pu.expanded_url_parsed,
            pu.expanded_url_host,
            pu.display_url,
            pu.preview_image_thumbnail_url,
            pu.preview_image_url
            
        FROM
            news_feed_url as nfu
            JOIN news_bsky_post_url as pu ON pu.url_id = nfu.url_id
            JOIN news_bsky_user as u ON u.did = nfu.first_referenced_by
        WHERE
            nfu.url_slug = $1
            AND pu.title IS NOT NULL
     "#,
        url_slug
    )
    .fetch_one(pool)
    .await
}
