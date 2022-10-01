use crate::models::news_twitter_user::NewsTwitterUser;
use sqlx::PgPool;

pub async fn insert_news_twitter_user(
    pool: &PgPool,
    news_twitter_user: NewsTwitterUser,
) -> Option<NewsTwitterUser> {
    let news_twitter_user_result = sqlx::query_as!(
        NewsTwitterUser,
        r#"
            INSERT INTO news_twitter_user ( 
                user_id, username, profile_image_url, description, verified, followers_count, listed_count, user_referenced_tweets_count, user_score, last_tweet_id, last_updated_at
             )
            VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING 
            user_id, username, profile_image_url, description, verified, followers_count, listed_count, user_referenced_tweets_count, user_score, last_tweet_id, last_updated_at
            "#,
        news_twitter_user.user_id,
        news_twitter_user.username,
        news_twitter_user.profile_image_url,
        news_twitter_user.description,
        news_twitter_user.verified,
        news_twitter_user.followers_count,
        news_twitter_user.listed_count,
        news_twitter_user.user_referenced_tweets_count,
        news_twitter_user.user_score,
        news_twitter_user.last_tweet_id,
        news_twitter_user.last_updated_at,
    )
    .fetch_one(pool)
    .await;
    match news_twitter_user_result {
        Ok(news_twitter_user) => Some(news_twitter_user),
        Err(_) => None,
    }
}

pub async fn find_all_news_twitter_users(pool: &PgPool) -> Option<Vec<NewsTwitterUser>> {
    let query = sqlx::query_as!(
        NewsTwitterUser,
        r#"
            SELECT user_id, username, profile_image_url, description, verified, followers_count, listed_count, user_referenced_tweets_count, user_score, last_tweet_id, last_updated_at
            FROM news_twitter_user
        "#
    );

    let news_twitter_users_result = query.fetch_all(pool).await;
    return match news_twitter_users_result {
        Ok(news_twitter_users) => Some(news_twitter_users),
        Err(_) => None,
    };
}

pub async fn find_news_twitter_user_by_user_id(
    pool: &PgPool,
    user_id: i64,
) -> Option<NewsTwitterUser> {
    let query = sqlx::query_as!(
        NewsTwitterUser,
        r#"
            SELECT user_id, username, profile_image_url, description, verified, followers_count, listed_count, user_referenced_tweets_count, user_score, last_tweet_id, last_updated_at
            FROM news_twitter_user
            WHERE user_id = $1;
        "#,
        user_id
    );

    let news_twitter_user_result = query.fetch_one(pool).await;
    return match news_twitter_user_result {
        Ok(news_twitter_user) => Some(news_twitter_user),
        Err(_) => None,
    };
}

pub async fn find_news_twitter_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Option<NewsTwitterUser> {
    let query = sqlx::query_as!(
        NewsTwitterUser,
        r#"
            SELECT user_id, username, profile_image_url, description, verified, followers_count, listed_count, user_referenced_tweets_count, user_score, last_tweet_id, last_updated_at
            FROM news_twitter_user
            WHERE username = $1;
        "#,
        username
    );

    let news_twitter_user_result = query.fetch_one(pool).await;
    return match news_twitter_user_result {
        Ok(news_twitter_user) => Some(news_twitter_user),
        Err(_) => None,
    };
}

pub async fn update_news_twitter_user_last_tweet_id(
    pool: &PgPool,
    user_id: i64,
    last_tweet_id: i64,
    last_updated_at: i64,
) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE news_twitter_user 
            SET last_tweet_id = $1, last_updated_at = $2
            WHERE user_id = $3
            "#,
        last_tweet_id,
        last_updated_at,
        user_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

//  TODO rename?
pub async fn update_news_twitter_user_stats(
    pool: &PgPool,
    user_id: i64,
    user_referenced_tweets_count: i32,
    user_score: i32,
) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE news_twitter_user 
            SET 
                user_referenced_tweets_count = $1,
                user_score = $2 
            WHERE user_id = $3
            "#,
        user_referenced_tweets_count,
        user_score,
        user_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn truncate_news_twitter_user(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_twitter_user RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
