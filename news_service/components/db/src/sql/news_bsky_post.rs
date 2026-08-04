use crate::models::news_bsky_post::NewsBskyPost;
use sqlx::PgPool;

pub async fn insert_news_bsky_post(
    pool: &PgPool,
    post: NewsBskyPost,
) -> Result<NewsBskyPost, sqlx::Error> {
    let result = sqlx::query_as!(
        NewsBskyPost,
        r#"
        INSERT INTO news_bsky_post ( post_uri, cid, text, author_did, reply_parent_uri, reply_root_uri, created_at, created_at_str )
        VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )
        ON CONFLICT (post_uri) DO NOTHING
        RETURNING post_uri, cid, text, author_did, reply_parent_uri, reply_root_uri, created_at, created_at_str
        "#,
        post.post_uri,
        post.cid,
        post.text,
        post.author_did,
        post.reply_parent_uri,
        post.reply_root_uri,
        post.created_at,
        post.created_at_str,
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.unwrap_or(post))
}

pub async fn find_news_bsky_post_by_uri(
    pool: &PgPool,
    post_uri: String,
) -> Result<Option<NewsBskyPost>, sqlx::Error> {
    let result = sqlx::query_as!(
        NewsBskyPost,
        r#"SELECT post_uri, cid, text, author_did, reply_parent_uri, reply_root_uri, created_at, created_at_str FROM news_bsky_post WHERE post_uri = $1"#,
        post_uri
    )
    .fetch_optional(pool)
    .await?;
    Ok(result)
}

pub async fn truncate_news_bsky_post(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_bsky_post RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}
