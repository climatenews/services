use crate::models::news_bsky_feed_source::NewsBskyFeedSource;
use sqlx::PgPool;

pub async fn insert_news_bsky_feed_source(
    pool: &PgPool,
    source: NewsBskyFeedSource,
) -> Result<NewsBskyFeedSource, sqlx::Error> {
    sqlx::query_as!(
        NewsBskyFeedSource,
        r#"
        INSERT INTO news_bsky_feed_source ( source_uri, source_type, last_checked_at )
        VALUES ( $1, $2, $3 )
        ON CONFLICT (source_uri) DO UPDATE SET last_checked_at = EXCLUDED.last_checked_at
        RETURNING source_uri, source_type, last_checked_at
        "#,
        source.source_uri,
        source.source_type,
        source.last_checked_at,
    )
    .fetch_one(pool)
    .await
}

pub async fn find_all_news_bsky_feed_sources(
    pool: &PgPool,
) -> Result<Vec<NewsBskyFeedSource>, sqlx::Error> {
    sqlx::query_as!(
        NewsBskyFeedSource,
        r#"SELECT source_uri, source_type, last_checked_at FROM news_bsky_feed_source"#
    )
    .fetch_all(pool)
    .await
}
