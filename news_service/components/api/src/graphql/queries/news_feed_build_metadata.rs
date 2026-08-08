use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult, SimpleObject};
use db::models::news_feed_url::NewsFeedBuildMetadata;
use db::sql::news_feed_url::get_news_feed_build_metadata;
use sqlx::postgres::PgPool;

// Non-null GraphQL wrapper around the aggregate build metadata.
#[derive(SimpleObject)]
#[graphql(name = "NewsFeedBuildMetadata")]
pub struct NewsFeedBuildMetadataGql {
    pub updated_at: i64,
    pub total_count: i64,
}

impl From<NewsFeedBuildMetadata> for NewsFeedBuildMetadataGql {
    fn from(metadata: NewsFeedBuildMetadata) -> Self {
        NewsFeedBuildMetadataGql {
            updated_at: metadata.updated_at.unwrap_or(0),
            total_count: metadata.total_count.unwrap_or(0),
        }
    }
}

// Change cursor + count for the static-site build to decide whether a rebuild is needed.
pub async fn news_feed_build_metadata_query<'a>(
    db_pool: &PgPool,
) -> FieldResult<NewsFeedBuildMetadataGql> {
    match get_news_feed_build_metadata(db_pool).await {
        Ok(build_metadata) => Ok(build_metadata.into()),
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
    async fn get_news_feed_build_metadata_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let created_at_timestamp = now_utc_timestamp();

        create_fake_news_feed_url(
            &db_pool,
            String::from("metadata-test"),
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
                    newsFeedBuildMetadata {
                        updatedAt
                        totalCount
                    }
                }
                "#,
            )
            .await;
        assert_eq!(
            resp.data,
            value!({
                "newsFeedBuildMetadata":
                    {
                        "updatedAt": created_at_timestamp,
                        "totalCount": 1,
                    }
                ,
            })
        );
    }
}
