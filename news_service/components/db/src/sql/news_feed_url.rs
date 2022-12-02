use crate::models::news_feed_url::NewsFeedUrl;
use sqlx::{postgres::PgQueryResult, PgPool};

pub async fn insert_news_feed_url(
    pool: &PgPool,
    news_feed_url: NewsFeedUrl,
) -> Option<NewsFeedUrl> {
    let news_feed_url_result = sqlx::query_as!(
        NewsFeedUrl,
        r#"
            INSERT INTO news_feed_url ( 
                url_id, url_score, num_references, first_referenced_by, is_climate_related, created_at, created_at_str
             )
            VALUES ( $1, $2, $3, $4, $5, $6, $7)
            RETURNING 
                url_id, url_score, num_references, first_referenced_by, is_climate_related, created_at, created_at_str
            "#,
        news_feed_url.url_id,
        news_feed_url.url_score,
        news_feed_url.num_references,
        news_feed_url.first_referenced_by,
        news_feed_url.is_climate_related,
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
            SELECT url_id, url_score, num_references, first_referenced_by, is_climate_related, created_at, created_at_str
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

pub async fn update_news_feed_url_url_is_climate_related(
    pool: &PgPool,
    url_id: i32,
    is_climate_related: bool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE news_feed_url 
        SET is_climate_related = $1
        WHERE url_id = $2
        "#,
        is_climate_related,
        url_id
    )
    .execute(pool)
    .await
}

pub async fn find_top_news_feed_urls_without_is_climate_related_set(
    pool: &PgPool,
) -> Result<Vec<NewsFeedUrl>, sqlx::Error> {
    let query = sqlx::query_as!(
        NewsFeedUrl,
        r#"
            SELECT url_id, url_score, num_references, first_referenced_by, is_climate_related, created_at, created_at_str
            FROM news_feed_url
            WHERE 
                is_climate_related IS NULL 
                AND url_score > 10
            ORDER BY url_score DESC
            LIMIT 50
            
        "#
    );

    query.fetch_all(pool).await
}

pub async fn truncate_news_feed_url(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_feed_url RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
