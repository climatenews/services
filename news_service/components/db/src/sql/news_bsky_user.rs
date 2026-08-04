use crate::models::news_bsky_user::NewsBskyUser;
use sqlx::PgPool;

pub async fn insert_news_bsky_user(
    pool: &PgPool,
    user: NewsBskyUser,
) -> Result<NewsBskyUser, sqlx::Error> {
    sqlx::query_as!(
        NewsBskyUser,
        r#"
        INSERT INTO news_bsky_user ( did, handle, display_name, avatar_url, description, followers_count, follows_count, posts_count, user_score, last_post_cid, last_updated_at, last_checked_at )
        VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 )
        ON CONFLICT (did) DO UPDATE SET
            handle = EXCLUDED.handle,
            display_name = EXCLUDED.display_name,
            avatar_url = EXCLUDED.avatar_url,
            description = EXCLUDED.description,
            followers_count = EXCLUDED.followers_count,
            follows_count = EXCLUDED.follows_count,
            posts_count = EXCLUDED.posts_count,
            last_checked_at = EXCLUDED.last_checked_at
        RETURNING did, handle, display_name, avatar_url, description, followers_count, follows_count, posts_count, user_score, last_post_cid, last_updated_at, last_checked_at
        "#,
        user.did,
        user.handle,
        user.display_name,
        user.avatar_url,
        user.description,
        user.followers_count,
        user.follows_count,
        user.posts_count,
        user.user_score,
        user.last_post_cid,
        user.last_updated_at,
        user.last_checked_at,
    )
    .fetch_one(pool)
    .await
}

pub async fn find_all_news_bsky_users(pool: &PgPool) -> Result<Vec<NewsBskyUser>, sqlx::Error> {
    sqlx::query_as!(
        NewsBskyUser,
        r#"SELECT did, handle, display_name, avatar_url, description, followers_count, follows_count, posts_count, user_score, last_post_cid, last_updated_at, last_checked_at FROM news_bsky_user ORDER BY handle ASC"#
    )
    .fetch_all(pool)
    .await
}

pub async fn find_news_bsky_user_by_did(
    pool: &PgPool,
    did: String,
) -> Result<NewsBskyUser, sqlx::Error> {
    sqlx::query_as!(
        NewsBskyUser,
        r#"SELECT did, handle, display_name, avatar_url, description, followers_count, follows_count, posts_count, user_score, last_post_cid, last_updated_at, last_checked_at FROM news_bsky_user WHERE did = $1"#,
        did
    )
    .fetch_one(pool)
    .await
}

pub async fn update_news_bsky_user_last_checked_at(
    pool: &PgPool,
    did: String,
    last_checked_at: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE news_bsky_user SET last_checked_at = $1 WHERE did = $2"#,
        last_checked_at,
        did
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_news_bsky_user_last_updated_at(
    pool: &PgPool,
    did: String,
    last_post_cid: Option<String>,
    last_updated_at: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE news_bsky_user SET last_post_cid = $1, last_updated_at = $2 WHERE did = $3"#,
        last_post_cid,
        last_updated_at,
        did
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_news_bsky_user_stats(
    pool: &PgPool,
    did: String,
    user_score: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE news_bsky_user SET user_score = $1 WHERE did = $2"#,
        user_score,
        did
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn truncate_news_bsky_user(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_bsky_user RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}
