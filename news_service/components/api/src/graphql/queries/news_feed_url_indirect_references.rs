use crate::graphql::errors::GqlError;
use async_graphql::{Context, ErrorExtensions, FieldResult};
use db::queries::news_feed_url_indirect_references_query::NewsFeedUrlIndirectReferencesQuery;
use db::sql::news_feed_url_references_indirect_query::get_indirect_news_feed_url_references;
use sqlx::postgres::PgPool;

pub async fn news_feed_url_indirect_references_query<'a>(
    ctx: &'a Context<'_>,
    url_id: i32,
) -> FieldResult<Vec<NewsFeedUrlIndirectReferencesQuery>> {
    let pool = ctx.data::<PgPool>().unwrap();
    let indirect_news_feed_url_references_result: Option<Vec<NewsFeedUrlIndirectReferencesQuery>> =
        get_indirect_news_feed_url_references(&pool, url_id).await;
    if let Some(indirect_news_feed_url_references) = indirect_news_feed_url_references_result {
        Ok(indirect_news_feed_url_references)
    } else {
        Err(GqlError::NotFound.extend())
    }
}
