use crate::queries::news_user_referenced_tweet_query::NewsUserReferencedTweetQuery;
use sqlx::PgPool;

// Used for user score calculation
// Find all user tweets that have been quoted or retweeted by others users
// i.e user = MichaelEMann
//   1) GretaThunberg retweeted MichaelEMann
//   2) DrShepherd2013 quoted MichaelEMann
pub async fn get_news_user_referenced_tweet_query(
    pool: &PgPool,
    user_id: i64,
) -> Option<Vec<NewsUserReferencedTweetQuery>> {
    let news_user_referenced_tweet_query_result = sqlx::query_as!(
        NewsUserReferencedTweetQuery,
        r#"
        SELECT
            t1.author_id,
            t1.tweet_id,
            t2.author_id as referenced_author_id,
            t2.tweet_id as referenced_tweet_id,
            rt.referenced_tweet_kind
        FROM
            news_referenced_tweet as rt
            JOIN news_tweet as t1 ON t1.tweet_id = rt.tweet_id
            JOIN news_tweet as t2 ON t2.tweet_id = rt.referenced_tweet_id
            
        WHERE 
            t1.author_id != t2.author_id               
            AND (rt.referenced_tweet_kind = 'retweeted' OR rt.referenced_tweet_kind = 'quoted')
            AND t2.author_id = $1
            "#,
        user_id
    )
    .fetch_all(pool)
    .await;
    return match news_user_referenced_tweet_query_result {
        Ok(news_user_referenced_tweets) => Some(news_user_referenced_tweets),
        Err(_) => None,
    };
}

pub async fn get_all_news_user_referenced_tweet_query(
    pool: &PgPool,
) -> Option<Vec<NewsUserReferencedTweetQuery>> {
    let news_user_referenced_tweet_query_result = sqlx::query_as!(
        NewsUserReferencedTweetQuery,
        r#"
        SELECT
            t1.author_id,
            t1.tweet_id,
            t2.author_id as referenced_author_id,
            t2.tweet_id as referenced_tweet_id,
            rt.referenced_tweet_kind
        FROM
            news_referenced_tweet as rt
            JOIN news_tweet as t1 ON t1.tweet_id = rt.tweet_id
            JOIN news_tweet as t2 ON t2.tweet_id = rt.referenced_tweet_id
            
        WHERE 
            t1.author_id != t2.author_id               
            AND (rt.referenced_tweet_kind = 'retweeted' OR rt.referenced_tweet_kind = 'quoted')
            "#
    )
    .fetch_all(pool)
    .await;
    return match news_user_referenced_tweet_query_result {
        Ok(news_user_referenced_tweets) => Some(news_user_referenced_tweets),
        Err(_) => None,
    };
}
