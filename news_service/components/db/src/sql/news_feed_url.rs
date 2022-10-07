use crate::models::news_feed_url::NewsFeedUrl;
use sqlx::PgPool;

pub async fn insert_news_feed_url(
    pool: &PgPool,
    news_feed_url: NewsFeedUrl,
) -> Option<NewsFeedUrl> {
    let news_feed_url_result = sqlx::query_as!(
        NewsFeedUrl,
        r#"
            INSERT INTO news_feed_url ( 
                url_id, url_score, num_references, created_at, created_at_str
             )
            VALUES ( $1, $2, $3, $4, $5)
            RETURNING 
                url_id, url_score, num_references, created_at, created_at_str
            "#,
        news_feed_url.url_id,
        news_feed_url.url_score,
        news_feed_url.num_references,
        news_feed_url.created_at,
        news_feed_url.created_at_str,
    )
    .fetch_one(pool)
    .await;
    match news_feed_url_result {
        Ok(news_feed_url) => Some(news_feed_url),
        Err(_) => None,
    }
}

pub async fn find_all_news_feed_urls(pool: &PgPool) -> Option<Vec<NewsFeedUrl>> {
    let query = sqlx::query_as!(
        NewsFeedUrl,
        r#"
            SELECT url_id, url_score, num_references, created_at, created_at_str
            FROM news_feed_url
        "#
    );

    let news_feed_urls_result = query.fetch_all(pool).await;
    match news_feed_urls_result {
        Ok(news_feed_urls) => Some(news_feed_urls),
        Err(_) => None,
    }
}

pub async fn truncate_news_feed_url(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_feed_url RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
