use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::sql::news_feed_url_query::get_news_feed_urls;
use db::util::time::past_days;
use sqlx::postgres::PgPool;

pub const NEWS_FEED_URLS_NUM_DAYS: i64 = 3;
pub const NEWS_FEED_URLS_LIMIT: i64 = 20;


pub async fn news_feed_urls_query<'a>(db_pool: &PgPool) -> FieldResult<Vec<NewsFeedUrlQuery>> {
    let recent_timestamp = past_days(NEWS_FEED_URLS_NUM_DAYS).unix_timestamp();
    let news_feed_urls_result: Option<Vec<NewsFeedUrlQuery>> = get_news_feed_urls(db_pool, recent_timestamp, NEWS_FEED_URLS_LIMIT).await;
    match news_feed_urls_result {
        Some(news_feed_urls) => Ok(news_feed_urls),
        None => Err(GqlError::NotFound.extend()),
    }
}

#[cfg(test)]
mod tests {

    use crate::graphql::{Query, ClimateActionSchema};
    use async_graphql::{value, EmptyMutation, EmptySubscription};
    use db::models::news_feed_url::NewsFeedUrl;
    use db::models::news_tweet_url::NewsTweetUrl;
    use db::models::news_twitter_user::NewsTwitterUser;
    use db::sql::news_twitter_user::{insert_news_twitter_user, truncate_news_twitter_user};
    use db::sql::news_feed_url::{insert_news_feed_url, truncate_news_feed_url};
    use db::sql::news_tweet_url::{insert_news_tweet_url, truncate_news_tweet_url};
    use db::util::convert::now_utc_timestamp;
    use db::{init_env, init_test_db_pool};
    use sqlx::PgPool;

    #[tokio::test]
    async fn get_news_feed_urls_test() {

        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        create_fake_news_tweet_url(&db_pool).await;
        let created_at_timestamp  = now_utc_timestamp() - 100; // to ensure news_feed_url is recent
        create_fake_news_feed_url(&db_pool, created_at_timestamp).await;
        create_fake_news_twitter_user(&db_pool).await;

        let schema = create_fake_schema(db_pool);

        let resp = schema
            .execute(
                r#"
                query {
                    newsFeedUrls {
                        urlId
                        urlScore
                        numReferences
                        firstReferencedByUsername
                        createdAt
                        title
                        description
                        expandedUrlParsed
                        expandedUrlHost
                        previewImageThumbnailUrl
                        previewImageUrl
                        displayUrl
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
                        "firstReferencedByUsername": String::from("username"),
                        "createdAt": created_at_timestamp,
                        "title": String::from("title"),
                        "description": String::from("description"),
                        "expandedUrlParsed": String::from("expanded_url_parsed"),
                        "expandedUrlHost": String::from("expanded_url_host"),
                        "previewImageThumbnailUrl": String::from("preview_image_thumbnail_url"),
                        "previewImageUrl": String::from("preview_image_url"),
                        "displayUrl": String::from("display_url"),

                    }
                ],
            })
        );
    }

    fn create_fake_schema(db_pool: PgPool) -> async_graphql::Schema<Query, EmptyMutation, EmptySubscription>{
        ClimateActionSchema::build(Query, EmptyMutation, EmptySubscription)
        .data(db_pool)
        .finish()
    }
        

    async fn create_fake_news_tweet_url(db_pool: &PgPool){
        truncate_news_tweet_url(&db_pool).await.unwrap();
        let news_tweet_url = NewsTweetUrl {
            url: String::from("https://t.co/4HPNrqOnZj"),
            expanded_url: String::from("expanded_url"),
            expanded_url_parsed: String::from("expanded_url_parsed"),
            expanded_url_host: String::from("expanded_url_host"),
            display_url: String::from("display_url"),
            is_twitter_url: false,
            is_english: true,
            preview_image_url: Some(String::from("preview_image_url")),
            preview_image_thumbnail_url: Some(String::from("preview_image_thumbnail_url")),
            title: String::from("title"),
            description: String::from("description"),
            created_at: now_utc_timestamp(),
            created_at_str: String::from("created_at_str"),
        };
        insert_news_tweet_url(&db_pool, news_tweet_url)
            .await
            .unwrap();

    }

    async fn create_fake_news_feed_url(db_pool: &PgPool, created_at_timestamp: i64){
        truncate_news_feed_url(&db_pool).await.unwrap();
        let news_feed_url = NewsFeedUrl {
            url_id: 1,
            url_score: 90,
            num_references: 2,
            first_referenced_by: 1,
            is_climate_related: true,
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_feed_url(&db_pool, news_feed_url).await.unwrap();
    }

    async fn create_fake_news_twitter_user(db_pool: &PgPool){
        truncate_news_twitter_user(&db_pool).await.unwrap();
        let news_twitter_user = NewsTwitterUser {
            user_id: 1,
            username: String::from("username"),
            profile_image_url: Some(String::from("profile_image_url")),
            description: Some(String::from("description")),
            verified: Some(true),
            followers_count: 100,
            listed_count: 100,
            user_referenced_tweets_count: None,
            user_score: None,
            last_tweet_id: None,
            last_updated_at: 0,
            last_checked_at: 0,
        };
        insert_news_twitter_user(db_pool, news_twitter_user).await.unwrap();
    }
}