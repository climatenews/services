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
                None => match news_feed_url_reference_query.referenced_username{
                    Some(referenced_username) => referenced_username,
                    None => "".to_string()
                },
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

#[cfg(test)]
mod tests {
    
    use crate::graphql::test_util::create_fake_schema;
    use async_graphql::value;
    use db::{
        init_env, init_test_db_pool,
        util::{
            convert::now_utc_timestamp,
            test::test_util::{
                create_fake_news_tweet_url, create_fake_news_twitter_user, create_fake_news_referenced_tweet_url, create_fake_news_tweet,
            },
        },
    };

    #[tokio::test]
    async fn get_news_feed_url_references_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let created_at_timestamp = now_utc_timestamp();

        create_fake_news_twitter_user(&db_pool).await;
        create_fake_news_tweet(&db_pool, created_at_timestamp).await;
        create_fake_news_tweet_url(&db_pool, created_at_timestamp).await;
        create_fake_news_referenced_tweet_url(&db_pool).await;

        let schema = create_fake_schema(db_pool);

        let resp = schema
            .execute(
                r#"
                query {
                    newsFeedUrlReferences(urlId: 1) {
                        tweetId
                        tweetText
                        tweetCreatedAtStr
                        authorUsername
                        retweetedByUsernames 
                      }
                }
                "#,
            )
            .await;
        assert_eq!(
            resp.data,
            value!({
                "newsFeedUrlReferences": [
                    {
                        "tweetId": String::from("1"),
                        "tweetText": String::from("tweet_text"),
                        "tweetCreatedAtStr": String::from("created_at_str"),
                        "authorUsername": String::from("username"),
                        "retweetedByUsernames": [],

                    }
                ],
            })
        );
    }
}

