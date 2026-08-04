use crate::models::news_bsky_referenced_post_url::NewsBskyReferencedPostUrl;
use sqlx::PgPool;

pub async fn insert_news_bsky_referenced_post_url(
    pool: &PgPool,
    ref_url: NewsBskyReferencedPostUrl,
) -> Result<NewsBskyReferencedPostUrl, sqlx::Error> {
    let result = sqlx::query_as!(
        NewsBskyReferencedPostUrl,
        r#"
        INSERT INTO news_bsky_referenced_post_url ( post_uri, url_id )
        VALUES ( $1, $2 )
        ON CONFLICT (post_uri, url_id) DO NOTHING
        RETURNING post_uri, url_id
        "#,
        ref_url.post_uri,
        ref_url.url_id,
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.unwrap_or(ref_url))
}

pub async fn truncate_news_bsky_referenced_post_url(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_bsky_referenced_post_url RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}
