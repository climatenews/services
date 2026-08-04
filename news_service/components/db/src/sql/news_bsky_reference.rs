use crate::models::news_bsky_reference::NewsBskyReference;
use sqlx::PgPool;

pub async fn insert_news_bsky_reference(
    pool: &PgPool,
    reference: NewsBskyReference,
) -> Result<NewsBskyReference, sqlx::Error> {
    sqlx::query_as!(
        NewsBskyReference,
        r#"
        INSERT INTO news_bsky_reference ( post_uri, ref_post_uri, ref_kind )
        VALUES ( $1, $2, $3 )
        ON CONFLICT (post_uri, ref_post_uri, ref_kind) DO NOTHING
        RETURNING post_uri, ref_post_uri, ref_kind
        "#,
        reference.post_uri,
        reference.ref_post_uri,
        reference.ref_kind,
    )
    .fetch_one(pool)
    .await
}

pub async fn truncate_news_bsky_reference(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_bsky_reference RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}
