use crate::news_feed::cron_job::get_all_user_tweets;
use crate::news_feed::news_feed::populate_news_feed;
// use crate::referenced_twitter_users::get_referenced_twitter_users;
use crate::twitter::init_twitter_api;
use chrono::Local;
use sqlx::PgPool;
use log::info;

pub async fn hourly_cron_job(db_pool: &PgPool) {
    info!("schedule_task event - {:?}", Local::now());
    let twitter_api = init_twitter_api();
    get_all_user_tweets(db_pool, &twitter_api).await;
    populate_news_feed(db_pool).await;
    // get_referenced_twitter_users(&db_pool, &twitter_api).await;
}
