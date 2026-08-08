use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::sql::news_feed_url::find_news_feed_url_slugs;
use sqlx::postgres::PgPool;

// All climate-related URL slugs in stable (deterministic) order for static builds.
// `limit` and `offset` are optional; omitted values return all slugs.
pub async fn news_feed_url_slugs_query<'a>(
    db_pool: &PgPool,
    limit: Option<i32>,
    offset: Option<i32>,
) -> FieldResult<Vec<String>> {
    let limit_i64 = limit.map(|l| l as i64);
    let offset_i64 = offset.map(|o| o as i64);

    match find_news_feed_url_slugs(db_pool, limit_i64, offset_i64).await {
        Ok(news_feed_urls) => Ok(news_feed_urls
            .iter()
            .map(|nfu| nfu.url_slug.to_string())
            .collect()),
        Err(_) => Err(GqlError::NotFound.extend()),
    }
}

#[cfg(test)]
mod tests {

    use crate::graphql::test_util::create_fake_schema;
    use async_graphql::value;
    use db::{
        init_env, init_test_db_pool,
        util::{convert::now_utc_timestamp, test::test_util::create_fake_news_feed_url},
    };

    #[tokio::test]
    async fn get_news_feed_url_slugs_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let created_at_timestamp = now_utc_timestamp();

        create_fake_news_feed_url(
            &db_pool,
            String::from("slug-test"),
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
                    newsFeedUrlSlugs
                }
                "#,
            )
            .await;
        assert_eq!(
            resp.data,
            value!({
                "newsFeedUrlSlugs": [
                    String::from("slug-test")
                ],
            })
        );
    }
}
