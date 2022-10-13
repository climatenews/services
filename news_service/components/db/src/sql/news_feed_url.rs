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
                url_id, url_score, num_references, first_referenced_by, created_at, created_at_str
             )
            VALUES ( $1, $2, $3, $4, $5, $6)
            RETURNING 
                url_id, url_score, num_references, first_referenced_by, created_at, created_at_str
            "#,
        news_feed_url.url_id,
        news_feed_url.url_score,
        news_feed_url.num_references,
        news_feed_url.first_referenced_by,
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

pub async fn find_news_feed_url_by_url_id(pool: &PgPool, url_id: i32) -> Option<NewsFeedUrl> {
    let query = sqlx::query_as!(
        NewsFeedUrl,
        r#"
            SELECT url_id, url_score, num_references, first_referenced_by, created_at, created_at_str
            FROM news_feed_url
            WHERE url_id = $1;
        "#,
        url_id
    );

    let news_feed_url_result = query.fetch_one(pool).await;
    match news_feed_url_result {
        Ok(news_feed_url) => Some(news_feed_url),
        Err(_) => None,
    }
}

pub async fn reset_news_feed_url_url_scores(pool: &PgPool) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE news_feed_url 
        SET url_score = 0
        "#,
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn update_news_feed_url_url_score_and_num_references(
    pool: &PgPool,
    url_id: i32,
    url_score: i32,
    num_references: i32,
) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE news_feed_url 
        SET url_score = $1, num_references = $2
        WHERE url_id = $3
            "#,
        url_score,
        num_references,
        url_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn find_all_news_feed_urls(pool: &PgPool) -> Option<Vec<NewsFeedUrl>> {
    let query = sqlx::query_as!(
        NewsFeedUrl,
        r#"
            SELECT url_id, url_score, num_references, first_referenced_by, created_at, created_at_str
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
