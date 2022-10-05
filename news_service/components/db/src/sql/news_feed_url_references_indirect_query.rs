use sqlx::PgPool;

use crate::queries::news_feed_url_indirect_references_query::NewsFeedUrlIndirectReferencesQuery;

pub async fn get_indirect_news_feed_url_references(
    pool: &PgPool,
    url_id: i32,
) -> Option<Vec<NewsFeedUrlIndirectReferencesQuery>> {
    let direct_news_feed_url_references_query_result = sqlx::query_as!(
        NewsFeedUrlIndirectReferencesQuery,
        r#"

        SELECT 
        t1.text,
        t2.text as referenced_tweet_text,
        t2.tweet_id as referenced_tweet_id,
        rt.referenced_tweet_kind,
        t1.created_at_str,
        u.username

    FROM
        news_referenced_tweet as rt   
        JOIN news_tweet as t1 ON t1.tweet_id = rt.tweet_id  
        JOIN news_tweet as t2 ON t2.tweet_id = rt.referenced_tweet_id 
        JOIN news_twitter_user as u ON t1.author_id = u.user_id   
        JOIN news_referenced_tweet_url as rtu ON rtu.tweet_id = rt.referenced_tweet_id 
        JOIN news_tweet_url as tu ON tu.id = rtu.url_id  
    WHERE
        rtu.url_id = $1
        AND tu.is_twitter_url = False
        AND tu.title IS NOT NULL
        AND t1.in_reply_to_user_id IS NULL            
     "#,
        url_id
    )
    .fetch_all(pool)
    .await;
    return match direct_news_feed_url_references_query_result {
        Ok(direct_news_feed_url_references) => Some(direct_news_feed_url_references),
        Err(_) => None,
    };
}
