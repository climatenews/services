use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::{
    models::news_feed_url_reference::NewsFeedUrlReference,
    queries::news_feed_url_references_query::get_news_feed_url_references_with_metadata,
};
use sqlx::postgres::PgPool;

pub async fn news_feed_url_references_query<'a>(
    pool: &PgPool,
    url_slug: String,
) -> FieldResult<Vec<NewsFeedUrlReference>> {
    match get_news_feed_url_references_with_metadata(pool, url_slug).await {
        Ok(news_feed_url_references) => Ok(news_feed_url_references),
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
                create_fake_news_bsky_post, create_fake_news_bsky_post_url,
                create_fake_news_bsky_referenced_post_url, create_fake_news_bsky_references,
                create_fake_news_bsky_user, create_fake_news_feed_url,
            },
        },
    };

    #[tokio::test]
    async fn get_news_feed_url_references_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let created_at_timestamp = now_utc_timestamp();

        create_fake_news_bsky_user(&db_pool).await;
        create_fake_news_bsky_post(&db_pool, created_at_timestamp).await;
        create_fake_news_bsky_post_url(&db_pool, created_at_timestamp).await;
        create_fake_news_bsky_referenced_post_url(&db_pool).await;
        create_fake_news_bsky_references(&db_pool).await;
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
                    newsFeedUrlReferences(urlSlug: "example-title") {
                        postUri
                        postText
                        postCreatedAtStr
                        authorHandle
                        repostedByHandles 
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
                        "postUri": String::from("at://did:plc:user1/app.bsky.feed.post/rkey1"),
                        "postText": String::from("post_text"),
                        "postCreatedAtStr": String::from("created_at_str"),
                        "authorHandle": String::from("user1.bsky.social"),
                        "repostedByHandles": [String::from("@reposter.bsky.social")],
                    },
                    {
                        "postUri": String::from("at://did:plc:user1/app.bsky.feed.post/rkey3"),
                        "postText": String::from("quoted_post_text"),
                        "postCreatedAtStr": String::from("created_at_str"),
                        "authorHandle": String::from("user1.bsky.social"),
                        "repostedByHandles": [],
                    }
                ],
            })
        );
    }
}
