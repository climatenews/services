// use crate::graphql::errors::GqlError;
// use async_graphql::{ErrorExtensions, FieldResult};
// use db::queries::news_feed_url_query::NewsFeedUrlQuery;
// use db::sql::news_feed_url_query::get_news_feed_url;
// use sqlx::postgres::PgPool;

// pub async fn news_feed_status_query<'a>(
//     db_pool: &PgPool,
//     url_id: i32,
// ) -> FieldResult<NewsFeedUrlQuery> {
//     match get_news_feed_url(db_pool, url_id).await {
//         Ok(news_feed_url) => Ok(news_feed_url),
//         Err(_) => Err(GqlError::NotFound.extend()),
//     }
// }