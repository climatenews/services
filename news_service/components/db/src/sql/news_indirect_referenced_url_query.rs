use crate::queries::news_referenced_url_query::NewsReferencedUrlQuery;
use sqlx::PgPool;

// Indirect referenced tweet URLS since timestamp
// Used by the news feed
// e.g
//   1) GretaThunberg retweeted a tweet that contained a link to theguardian.com
//   2) DrShepherd2013 quoted a tweet that contained a link to nature.com
pub async fn get_news_indirect_referenced_urls(
    pool: &PgPool,
    timestamp: i64,
) -> Option<Vec<NewsReferencedUrlQuery>> {
    let news_indirect_referenced_url_query_result = sqlx::query_as!(
        NewsReferencedUrlQuery,
        r#"
        SELECT 
            rtu.url_id,
            t.author_id,
            u.user_score,
            t.created_at,
            t.created_at_str
        FROM news_referenced_tweet as rt   
        JOIN news_tweet as t ON t.tweet_id = rt.tweet_id  
        JOIN news_twitter_user as u ON t.author_id = u.user_id   
        JOIN news_referenced_tweet_url as rtu ON rtu.tweet_id = rt.referenced_tweet_id 
        JOIN news_tweet_url as tu ON tu.id = rtu.url_id  
            
        WHERE
            tu.is_twitter_url = False
            AND t.in_reply_to_user_id IS NULL
            AND tu.title IS NOT NULL
            AND (rt.referenced_tweet_kind = 'retweeted' OR rt.referenced_tweet_kind = 'quoted')
            AND t.created_at > $1
        ORDER BY  
            t.created_at DESC
     "#,
        timestamp
    )
    .fetch_all(pool)
    .await;
    return match news_indirect_referenced_url_query_result {
        Ok(news_direct_referenced_urls) => Some(news_direct_referenced_urls),
        Err(_) => None,
    };
}
