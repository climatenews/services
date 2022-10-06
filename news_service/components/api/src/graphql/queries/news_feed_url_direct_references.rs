use crate::graphql::errors::GqlError;
use async_graphql::{Context, ErrorExtensions, FieldResult};
use db::queries::news_feed_url_references_query::NewsFeedUrlDirectReferencesQuery;
use db::sql::news_feed_url_references_query::get_direct_news_feed_url_references;
use sqlx::postgres::PgPool;

pub async fn news_feed_url_direct_references_query<'a>(
    ctx: &'a Context<'_>,
    url_id: i32,
) -> FieldResult<Vec<NewsFeedUrlDirectReferencesQuery>> {
    let pool = ctx.data::<PgPool>().unwrap();
    let direct_news_feed_url_references_result: Option<Vec<NewsFeedUrlDirectReferencesQuery>> =
        get_direct_news_feed_url_references(&pool, url_id).await;
    if let Some(direct_news_feed_url_references) = direct_news_feed_url_references_result {
        Ok(direct_news_feed_url_references)
    } else {
        Err(GqlError::NotFound.extend())
    }
}
