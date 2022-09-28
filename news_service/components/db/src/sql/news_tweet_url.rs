use crate::models::news_tweet_url::{NewsTweetUrl, NewsTweetUrlWithId};
use sqlx::PgPool;

pub async fn insert_news_tweet_url(
    pool: &PgPool,
    news_tweet_url: NewsTweetUrl,
) -> Option<NewsTweetUrlWithId> {
    let news_tweet_url_result = sqlx::query_as!(
        NewsTweetUrlWithId,
        r#"
            INSERT INTO news_tweet_url ( 
                url, expanded_url, expanded_url_parsed, expanded_url_host, display_url, is_twitter_url, title, description, created_at, created_at_str
             )
            VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING 
            id, url, expanded_url, expanded_url_parsed, expanded_url_host, display_url, is_twitter_url, title, description, created_at, created_at_str
            "#,
        news_tweet_url.url,
        news_tweet_url.expanded_url,
        news_tweet_url.expanded_url_parsed,
        news_tweet_url.expanded_url_host,
        news_tweet_url.display_url,
        news_tweet_url.is_twitter_url,
        news_tweet_url.title,
        news_tweet_url.description,
        news_tweet_url.created_at,
        news_tweet_url.created_at_str,
    )
    .fetch_one(pool)
    .await;

    return match news_tweet_url_result {
        Ok(news_tweet_url) => Some(news_tweet_url),
        Err(_) => None,
    };
}

// TODO: Log sqlx errors + ignore RowNotFound errors
pub async fn find_news_tweet_urls_by_expanded_url_parsed(
    pool: &PgPool,
    expanded_url_parsed: String,
) -> Option<NewsTweetUrlWithId> {
    let news_tweet_url_result = sqlx::query_as!(
        NewsTweetUrlWithId,
        r#"
            SELECT 
                id, url, expanded_url, expanded_url_parsed, expanded_url_host, display_url, is_twitter_url, title, description, created_at, created_at_str
            FROM news_tweet_url   
            WHERE expanded_url_parsed = $1         
            "#,
            expanded_url_parsed
    )
    .fetch_one(pool)
    .await;
    return match news_tweet_url_result {
        Ok(news_tweet_url) => Some(news_tweet_url),
        Err(_) => None,
    };
}

pub async fn truncate_news_tweet_url(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_tweet_url RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
