use crate::news_feed::algorithm::news_feed_v1::populate_news_feed_v1;
use crate::news_feed::user_tweets::get_all_user_tweets;
use crate::twitter::init_twitter_api;
use anyhow::Result;
use chrono::Local;
use log::info;
use sqlx::PgPool;

// TODO retry again on any fail
pub async fn hourly_cron_job(db_pool: &PgPool, is_initilizing: bool) -> Result<()> {
    info!("schedule_task event - {:?}", Local::now());
    let twitter_api = init_twitter_api();
    get_all_user_tweets(db_pool, &twitter_api, is_initilizing).await?;
    populate_news_feed_v1(db_pool).await?;
    Ok(())
}
