use crate::graphql::queries::news_feed_url_references::news_feed_url_references_query;
use crate::graphql::queries::news_feed_urls::news_feed_urls_query;
use crate::graphql::Query;
use async_graphql::{Context, FieldResult, Object};
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::queries::news_feed_url_references_query::NewsFeedUrlReferencesQuery;

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
    ) -> FieldResult<Vec<NewsFeedUrlReferencesQuery>> {
        news_feed_url_references_query(ctx, url_id).await
    }
}
