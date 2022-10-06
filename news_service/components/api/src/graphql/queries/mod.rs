use crate::graphql::queries::news_feed_url_direct_references::news_feed_url_direct_references_query;
use crate::graphql::queries::news_feed_url_indirect_references::news_feed_url_indirect_references_query;
use crate::graphql::queries::news_feed_urls::news_feed_urls_query;
use crate::graphql::Query;
use async_graphql::{Context, FieldResult, Object};
use db::queries::news_feed_url_indirect_references_query::NewsFeedUrlIndirectReferencesQuery;
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::{
    models::news_feed_url_reference::NewsFeedUrlReference,
    queries::news_feed_url_references_query::NewsFeedUrlDirectReferencesQuery,
};
use sqlx::postgres::PgPool;

use self::news_feed_url_references::news_feed_url_references_query;

pub mod news_feed_url_direct_references;
pub mod news_feed_url_indirect_references;
pub mod news_feed_url_references;
pub mod news_feed_urls;

#[Object(extends)]
impl Query {
    async fn news_feed_urls<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<NewsFeedUrlQuery>> {
        news_feed_urls_query(ctx).await
    }

    async fn news_feed_url_references<'a>(
        &self,
        ctx: &'a Context<'_>,
        url_id: i32,
    ) -> FieldResult<Vec<NewsFeedUrlReference>> {
        let pool = ctx.data::<PgPool>().unwrap();
        news_feed_url_references_query(pool, url_id).await
    }

    async fn news_feed_url_direct_references<'a>(
        &self,
        ctx: &'a Context<'_>,
        url_id: i32,
    ) -> FieldResult<Vec<NewsFeedUrlDirectReferencesQuery>> {
        news_feed_url_direct_references_query(ctx, url_id).await
    }

    async fn news_feed_url_indirect_references<'a>(
        &self,
        ctx: &'a Context<'_>,
        url_id: i32,
    ) -> FieldResult<Vec<NewsFeedUrlIndirectReferencesQuery>> {
        news_feed_url_indirect_references_query(ctx, url_id).await
    }
}
