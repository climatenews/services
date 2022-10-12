use crate::models::news_twitter_list::NewsTwitterList;
use sqlx::PgPool;

pub async fn insert_news_twitter_list(
    pool: &PgPool,
    news_twitter_list: NewsTwitterList,
) -> Option<NewsTwitterList> {
    let news_twitter_list_result = sqlx::query_as!(
        NewsTwitterList,
        r#"
            INSERT INTO news_twitter_list ( 
                list_id, last_checked_at
             )
            VALUES ( $1, $2)
            RETURNING 
                list_id, last_checked_at
            "#,
        news_twitter_list.list_id,
        news_twitter_list.last_checked_at,
    )
    .fetch_one(pool)
    .await;
    match news_twitter_list_result {
        Ok(news_twitter_list) => Some(news_twitter_list),
        Err(_) => None,
    }
}


pub async fn find_news_twitter_list_by_list_id(
    pool: &PgPool,
    list_id: i64,
) -> Option<NewsTwitterList> {
    let query = sqlx::query_as!(
        NewsTwitterList,
        r#"
            SELECT list_id, last_checked_at
            FROM news_twitter_list
            WHERE list_id = $1;
        "#,
        list_id
    );

    let news_twitter_list_result = query.fetch_one(pool).await;
    match news_twitter_list_result {
        Ok(news_twitter_list) => Some(news_twitter_list),
        Err(_) => None,
    }
}


pub async fn update_news_twitter_list_last_checked_at(
    pool: &PgPool,
    list_id: i64,
    last_checked_at: i64,
) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE news_twitter_list 
            SET last_checked_at = $1
            WHERE list_id = $2
            "#,
            last_checked_at,
        list_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}


pub async fn truncate_news_twitter_list(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_twitter_list RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
