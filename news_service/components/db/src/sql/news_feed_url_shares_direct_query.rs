use sqlx::PgPool;

use crate::queries::news_feed_url_shares_query::NewsFeedUrlSharesQuery;

pub async fn get_direct_news_feed_url_shares(pool: &PgPool, url_id: i32) -> Option<Vec<NewsFeedUrlSharesQuery>> {
    let direct_news_feed_url_shares_query_result = sqlx::query_as!(
        NewsFeedUrlSharesQuery,
        r#"
        SELECT 
            t.text,
            t.created_at_str,
            u.username
        FROM
            news_referenced_tweet as rt   
            JOIN news_tweet as t ON t.tweet_id = rt.tweet_id  
            JOIN news_twitter_user as u ON t.author_id = u.user_id   
            JOIN news_referenced_tweet_url as rtu ON rtu.tweet_id = rt.referenced_tweet_id 
            JOIN news_tweet_url as tu ON tu.id = rtu.url_id  
        WHERE
            rtu.url_id = $1
     "#,
     url_id
    )
    .fetch_all(pool)
    .await;
    return match direct_news_feed_url_shares_query_result {
        Ok(direct_news_feed_url_shares) => Some(direct_news_feed_url_shares),
        Err(_) => None,
    };
}
