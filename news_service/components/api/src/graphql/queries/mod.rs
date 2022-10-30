use self::news_feed_url::news_feed_url_query;
use self::news_feed_url_references::news_feed_url_references_query;
use crate::graphql::queries::news_feed_urls::news_feed_urls_query;
use crate::graphql::Query;
use async_graphql::{Context, FieldResult, Object};
use db::models::news_feed_url_reference::NewsFeedUrlReference;
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use sqlx::postgres::PgPool;

pub mod news_feed_url;
pub mod news_feed_url_references;
pub mod news_feed_urls;

pub const NEWS_FEED_URLS_NUM_DAYS: i64 = 3;
pub const NEWS_FEED_URLS_LIMIT: i64 = 20;

#[Object(extends)]
impl Query {
    async fn news_feed_urls<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<NewsFeedUrlQuery>> {
        let db_pool = ctx.data::<PgPool>()?;
        news_feed_urls_query(db_pool).await
    }

    async fn news_feed_url<'a>(
        &self,
        ctx: &'a Context<'_>,
        url_id: i32,
    ) -> FieldResult<NewsFeedUrlQuery> {
        let db_pool = ctx.data::<PgPool>()?;
        news_feed_url_query(db_pool, url_id).await
    }

    async fn news_feed_url_references<'a>(
        &self,
        ctx: &'a Context<'_>,
        url_id: i32,
    ) -> FieldResult<Vec<NewsFeedUrlReference>> {
        let db_pool = ctx.data::<PgPool>()?;
        news_feed_url_references_query(db_pool, url_id).await
    }
}
