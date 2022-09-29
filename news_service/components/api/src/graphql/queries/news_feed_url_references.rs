use crate::graphql::errors::GqlError;
use async_graphql::{Context, ErrorExtensions, FieldResult};
use db::queries::news_feed_url_references_query::NewsFeedUrlReferencesQuery;
use db::sql::news_feed_url_references_direct_query::get_direct_news_feed_url_references;
use db::sql::news_feed_url_references_indirect_query::get_indirect_news_feed_url_references;
use sqlx::postgres::PgPool;

pub async fn news_feed_url_references_query<'a>(
    ctx: &'a Context<'_>,
    url_id: i32,
) -> FieldResult<Vec<NewsFeedUrlReferencesQuery>> {
    let pool = ctx.data::<PgPool>().unwrap();
    let direct_news_feed_url_references_result: Option<Vec<NewsFeedUrlReferencesQuery>> =
        get_direct_news_feed_url_references(&pool, url_id).await;
    let indirect_news_feed_url_references_result: Option<Vec<NewsFeedUrlReferencesQuery>> =
        get_indirect_news_feed_url_references(&pool, url_id).await;
    if let (Some(direct_news_feed_url_references), Some(indirect_news_feed_url_references)) = (
        direct_news_feed_url_references_result,
        indirect_news_feed_url_references_result,
    ) {
        let news_feed_url_references = [
            direct_news_feed_url_references,
            indirect_news_feed_url_references,
        ]
        .concat();
        Ok(news_feed_url_references)
    } else {
        Err(GqlError::NotFound.extend())
    }
}
