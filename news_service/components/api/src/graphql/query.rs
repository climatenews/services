use super::errors::GqlError;
use async_graphql::{Context, ErrorExtensions, FieldResult, Object};

use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::sql::news_feed_url_query::get_news_feed_urls;
use sqlx::postgres::PgPool;

pub struct Query;

#[Object(extends)]
impl Query {
    async fn news_feed_urls<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<NewsFeedUrlQuery>> {
        let pool = ctx.data::<PgPool>().unwrap();
        let news_feed_urls_result: Option<Vec<NewsFeedUrlQuery>> = get_news_feed_urls(&pool).await;
        return match news_feed_urls_result {
            Some(news_feed_urls) => Ok(news_feed_urls),
            None => Err(GqlError::NotFound.extend()),
        };
    }
}

#[tokio::test]
async fn get_news_feed_urls_test() {
    use crate::graphql::ClimateActionSchema;
    use async_graphql::{value, EmptyMutation, EmptySubscription};
    use db::models::news_feed_url::NewsFeedUrl;
    use db::models::news_tweet_url::NewsTweetUrl;
    use db::sql::news_feed_url::{insert_news_feed_url, truncate_news_feed_url};
    use db::sql::news_tweet_url::{insert_news_tweet_url, truncate_news_tweet_url};
    use db::util::convert::now_utc_timestamp;
    use db::{init_env, init_test_db_pool};

    init_env();
    let db_pool = init_test_db_pool().await.unwrap();
    truncate_news_feed_url(&db_pool).await.unwrap();
    truncate_news_tweet_url(&db_pool).await.unwrap();

    let news_tweet_url = NewsTweetUrl {
        url: String::from("https://t.co/4HPNrqOnZj"),
        expanded_url: String::from("www.climateactioncolective.net"),
        parsed_expanded_url: String::from("www.climateactioncolective.net"),
        display_url: String::from("www.climateactioncolective.net"),
        is_twitter_url: false,
        title: Some(String::from("Climate Action Collective title")),
        description: Some(String::from("Climate Action Collective description")),
        created_at: now_utc_timestamp(),
        created_at_str: String::from(""),
    };
    insert_news_tweet_url(&db_pool, news_tweet_url)
        .await
        .unwrap();

    let news_feed_url = NewsFeedUrl {
        url_id: 1,
        url_score: 90,
        num_references: 2,
        created_at: now_utc_timestamp(),
        created_at_str: String::from(""),
    };
    insert_news_feed_url(&db_pool, news_feed_url).await.unwrap();

    let schema = ClimateActionSchema::build(Query, EmptyMutation, EmptySubscription)
        .data(db_pool)
        .finish();

    let resp = schema
        .execute(
            r#"
            query {
                newsFeedUrls {
                  urlId
                  urlScore
                  numReferences
                  title
                  description
                  parsedExpandedUrl
                }
              }
            "#,
        )
        .await;
    assert_eq!(
        resp.data,
        value!({
            "newsFeedUrls": [
                {
                    "urlId": 1,
                    "urlScore": 90,
                    "numReferences": 2,
                    "title": String::from("Climate Action Collective title"),
                    "description": String::from("Climate Action Collective description"),

                }
            ],
        })
    );
}
