use sqlx::PgPool;

use crate::queries::news_feed_url_query::NewsFeedUrlQuery;

pub async fn get_news_feed_urls(pool: &PgPool) -> Option<Vec<NewsFeedUrlQuery>> {
    //TODO ensure links were created in past week
    let news_feed_url_query_result = sqlx::query_as!(
        NewsFeedUrlQuery,
        r#"
        SELECT
            nfu.url_id, 
            nfu.url_score,
            nfu.num_references,
            nfu.created_at,
            tu.title,
            tu.description,
            tu.expanded_url_parsed,
            tu.expanded_url_host
        FROM
            news_feed_url as nfu
            JOIN news_tweet_url as tu ON tu.id = nfu.url_id
            
        ORDER BY
            url_score DESC
        LIMIT 20 
     "#
    )
    .fetch_all(pool)
    .await;
    return match news_feed_url_query_result {
        Ok(news_feed_urls) => Some(news_feed_urls),
        Err(_) => None,
    };
}
