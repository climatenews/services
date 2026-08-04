use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::constants::{NEWS_FEED_URLS_LIMIT, NEWS_FEED_URLS_NUM_DAYS};
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::sql::news_feed_url_query::get_news_feed_urls;
use db::util::time::past_days;
use sqlx::postgres::PgPool;

pub async fn news_feed_urls_query<'a>(db_pool: &PgPool) -> FieldResult<Vec<NewsFeedUrlQuery>> {
    let recent_timestamp = past_days(NEWS_FEED_URLS_NUM_DAYS).unix_timestamp();
    match get_news_feed_urls(db_pool, recent_timestamp, NEWS_FEED_URLS_LIMIT).await {
        Ok(news_feed_urls) => Ok(news_feed_urls),
        Err(_) => Err(GqlError::NotFound.extend()),
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
                create_fake_news_bsky_post_url, create_fake_news_bsky_user,
                create_fake_news_feed_url,
            },
        },
    };

    #[tokio::test]
    async fn get_news_feed_urls_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let created_at_timestamp = now_utc_timestamp();
        create_fake_news_bsky_post_url(&db_pool, created_at_timestamp).await;
        create_fake_news_bsky_user(&db_pool).await;
        create_fake_news_feed_url(
            &db_pool,
            String::from("example-title"),
            1,
            created_at_timestamp,
            true,
        )
        .await;

        let schema = create_fake_schema(db_pool);

        let resp = schema
            .execute(
                r#"
                query {
                    newsFeedUrls {
                        urlSlug
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
                        "urlSlug": String::from("example-title"),
                        "urlId": 1,
                        "urlScore": 90,
                        "numReferences": 2,
                        "firstReferencedByUsername": String::from("user1.bsky.social"),
                        "createdAt": now_utc_timestamp(),
                        "title": String::from("example title"),
                        "description": String::from("description"),
                        "expandedUrlParsed": String::from("https://example.com/link"),
                        "expandedUrlHost": String::from("example.com"),
                        "previewImageThumbnailUrl": String::from("preview_image_thumbnail_url"),
                        "previewImageUrl": String::from("preview_image_url"),
                        "displayUrl": String::from("example.com/link"),

                    }
                ],
            })
        );
    }
}
