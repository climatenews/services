use crate::queries::news_referenced_url_query::NewsReferencedUrlQuery;
use sqlx::PgPool;

// Referenced Bluesky post URLs since timestamp
// Used by the news feed
// e.g
//   1) KatharineHayhoe shared a link to theguardian.com
//   2) Bill McKibben shared a link to nature.com
//   3) A post shared a link that was reposted by a followed account
pub async fn get_news_referenced_urls(
    pool: &PgPool,
    timestamp: i64,
) -> Option<Vec<NewsReferencedUrlQuery>> {
    let news_referenced_url_query_result = sqlx::query_as!(
        NewsReferencedUrlQuery,
        r#"
        SELECT 
            rpu.url_id,
            p.author_did as author_id,
            u.user_score,
            p.created_at,
            p.created_at_str
        FROM
            news_bsky_referenced_post_url as rpu 
            JOIN news_bsky_post_url as pu ON pu.url_id = rpu.url_id
            JOIN news_bsky_post as p ON p.post_uri = rpu.post_uri
            LEFT JOIN news_bsky_user as u ON p.author_did = u.did
            
        WHERE
            pu.is_bsky_url = False
            AND pu.is_english = True
            AND pu.title IS NOT NULL
            AND p.reply_parent_uri IS NULL
        AND p.created_at > $1
        ORDER BY  
            p.created_at DESC
     "#,
        timestamp
    )
    .fetch_all(pool)
    .await;
    match news_referenced_url_query_result {
        Ok(news_referenced_urls) => Some(news_referenced_urls),
        Err(_) => None,
    }
}
