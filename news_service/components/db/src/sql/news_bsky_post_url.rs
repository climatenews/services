use crate::models::news_bsky_post_url::NewsBskyPostUrl;
use sqlx::PgPool;

pub async fn insert_news_bsky_post_url(
    pool: &PgPool,
    url: NewsBskyPostUrl,
) -> Result<NewsBskyPostUrl, sqlx::Error> {
    sqlx::query_as!(
        NewsBskyPostUrl,
        r#"
        INSERT INTO news_bsky_post_url ( url, expanded_url, expanded_url_parsed, expanded_url_host, display_url, is_bsky_url, is_english, title, description, preview_image_thumbnail_url, preview_image_url, created_at, created_at_str )
        VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13 )
        RETURNING url_id, url, expanded_url, expanded_url_parsed, expanded_url_host, display_url, is_bsky_url, is_english, title, description, preview_image_thumbnail_url, preview_image_url, created_at, created_at_str
        "#,
        url.url,
        url.expanded_url,
        url.expanded_url_parsed,
        url.expanded_url_host,
        url.display_url,
        url.is_bsky_url,
        url.is_english,
        url.title,
        url.description,
        url.preview_image_thumbnail_url,
        url.preview_image_url,
        url.created_at,
        url.created_at_str,
    )
    .fetch_one(pool)
    .await
}

pub async fn find_news_bsky_post_url_by_expanded_url_parsed(
    pool: &PgPool,
    expanded_url_parsed: String,
) -> Result<Option<NewsBskyPostUrl>, sqlx::Error> {
    let result = sqlx::query_as!(
        NewsBskyPostUrl,
        r#"SELECT url_id, url, expanded_url, expanded_url_parsed, expanded_url_host, display_url, is_bsky_url, is_english, title, description, preview_image_thumbnail_url, preview_image_url, created_at, created_at_str FROM news_bsky_post_url WHERE expanded_url_parsed = $1"#,
        expanded_url_parsed
    )
    .fetch_optional(pool)
    .await?;
    Ok(result)
}

pub async fn find_news_bsky_post_url_by_id(
    pool: &PgPool,
    url_id: i32,
) -> Result<NewsBskyPostUrl, sqlx::Error> {
    sqlx::query_as!(
        NewsBskyPostUrl,
        r#"SELECT url_id, url, expanded_url, expanded_url_parsed, expanded_url_host, display_url, is_bsky_url, is_english, title, description, preview_image_thumbnail_url, preview_image_url, created_at, created_at_str FROM news_bsky_post_url WHERE url_id = $1"#,
        url_id
    )
    .fetch_one(pool)
    .await
}

// Fill in metadata (title, description, language, preview images) for URLs that
// were inserted without a link card
pub async fn update_news_bsky_post_url_metadata(
    pool: &PgPool,
    url_id: i32,
    title: Option<String>,
    description: Option<String>,
    is_english: Option<bool>,
    preview_image_thumbnail_url: Option<String>,
    preview_image_url: Option<String>,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE news_bsky_post_url
        SET title = $2, description = $3, is_english = $4,
            preview_image_thumbnail_url = $5, preview_image_url = $6
        WHERE url_id = $1
        "#,
        url_id,
        title,
        description,
        is_english,
        preview_image_thumbnail_url,
        preview_image_url,
    )
    .execute(pool)
    .await
}

pub async fn truncate_news_bsky_post_url(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_bsky_post_url RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}
