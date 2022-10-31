use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::{
    models::news_feed_url_reference::NewsFeedUrlReference,
    queries::news_feed_url_references_query::NewsFeedUrlReferencesQuery,
    sql::news_feed_url_references_query::get_news_feed_url_references,
};
use sqlx::postgres::PgPool;

pub async fn news_feed_url_references_query<'a>(
    pool: &PgPool,
    url_id: i32,
) -> FieldResult<Vec<NewsFeedUrlReference>> {
    let news_feed_url_references_result: Option<Vec<NewsFeedUrlReferencesQuery>> =
        get_news_feed_url_references(pool, url_id).await;

    if let Some(news_feed_url_references_query) = news_feed_url_references_result {
        let mut news_feed_url_references: Vec<NewsFeedUrlReference> = vec![];
        for news_feed_url_reference_query in news_feed_url_references_query {
            let retweeted_by_usernames: Vec<String> = vec![];

            let author_username = match news_feed_url_reference_query.username {
                Some(author_username) => author_username,
                None => "TODO lookup".to_string(),
            };

            let news_feed_url_reference = NewsFeedUrlReference {
                url_id: news_feed_url_reference_query.url_id,
                tweet_id: news_feed_url_reference_query.tweet_id.to_string(),
                tweet_text: news_feed_url_reference_query.text,
                tweet_created_at_str: news_feed_url_reference_query.created_at_str,
                author_username,
                retweeted_by_usernames,
            };
            news_feed_url_references.push(news_feed_url_reference);
        }
        Ok(news_feed_url_references)
    } else {
        Err(GqlError::NotFound.extend())
    }
}

// #[tokio::test]
// async fn get_news_feed_urls_test() {}
