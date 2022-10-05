use sqlx::PgPool;

use crate::queries::news_feed_url_direct_references_query::NewsFeedUrlDirectReferencesQuery;

// TODO add user_score
pub async fn get_direct_news_feed_url_references(
    pool: &PgPool,
    url_id: i32,
) -> Option<Vec<NewsFeedUrlDirectReferencesQuery>> {
    let direct_news_feed_url_references_query_result = sqlx::query_as!(
        NewsFeedUrlDirectReferencesQuery,
        r#"
        SELECT 
            t.text,
            t.tweet_id,
            t.author_id,
            t.created_at_str,
            u.username,
            rtu.url_id
        FROM
            news_referenced_tweet_url as rtu 
            JOIN news_tweet_url as tu ON tu.id = rtu.url_id
            JOIN news_tweet as t ON t.tweet_id = rtu.tweet_id
            JOIN news_twitter_user as u ON t.author_id = u.user_id	
            LEFT JOIN news_referenced_tweet as rt ON t.tweet_id = rt.tweet_id
        WHERE
            rtu.url_id = $1
            AND tu.is_twitter_url = False
            AND tu.title IS NOT NULL
            AND t.in_reply_to_user_id IS NULL
            AND rt.referenced_tweet_kind IS NULL
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
