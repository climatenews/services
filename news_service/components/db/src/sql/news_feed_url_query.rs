use sqlx::PgPool;

use crate::queries::news_feed_url_query::NewsFeedUrlQuery;

pub async fn get_news_feed_urls(
    pool: &PgPool,
    timestamp: i64,
    limit: i64,
) -> Option<Vec<NewsFeedUrlQuery>> {
    //TODO ensure links were created in past week
    let news_feed_url_query_result = sqlx::query_as!(
        NewsFeedUrlQuery,
        r#"
        SELECT
            nfu.url_id, 
            nfu.url_score,
            nfu.num_references,
            u.username as first_referenced_by_username,
            nfu.created_at,
            tu.title,
            tu.description,
            tu.expanded_url_parsed,
            tu.expanded_url_host,
            tu.display_url,
            tu.preview_image_thumbnail_url,
            tu.preview_image_url
            
        FROM
            news_feed_url as nfu
            JOIN news_tweet_url as tu ON tu.id = nfu.url_id
            JOIN news_twitter_user as u ON u.user_id = nfu.first_referenced_by
        WHERE
            nfu.created_at > $1  
            -- AND tu.is_climate_related = True  
        ORDER BY
            url_score DESC
            -- num_references DESC
        LIMIT $2 
     "#,
        timestamp,
        limit
    )
    .fetch_all(pool)
    .await;
    match news_feed_url_query_result {
        Ok(news_feed_urls) => Some(news_feed_urls),
        Err(_) => None,
    }
}
