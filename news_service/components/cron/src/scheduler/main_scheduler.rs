use crate::bluesky::init_bluesky_agent;
use crate::news_feed::algorithm::news_feed_v1::populate_news_feed_v1;
use crate::news_feed::user_tweets_bsky::get_all_bsky_posts;
use crate::slack::send_main_cron_message;
use anyhow::Result;
use db::models::news_cron_job::{CronType, NewsCronJob};
use db::sql::news_cron_job::{
    insert_news_cron_job, update_news_cron_job_completed_at, update_news_cron_job_error,
};
use db::util::convert::{datetime_to_str, now_utc_datetime};
use db::util::db::init_db_result;
use db::util::time::now_formated;
use log::{error, info};
use sqlx::PgPool;
use tokio::time::{sleep, Duration};

pub async fn start_main_scheduler() {
    loop {
        send_main_cron_message(format!("main_scheduler started - {:?}", now_formated())).await;
        match init_main_cron_job().await {
            Ok(_) => {
                send_main_cron_message(format!(
                    "init_main_cron_job success - {:?}",
                    now_formated()
                ))
                .await;
            }
            Err(err) => {
                send_main_cron_message(format!("init_main_cron_job error - {:?}", err)).await;
            }
        }
        // Sleep for 10 minutes
        sleep(Duration::from_secs(10 * 60)).await;
    }
}

pub async fn init_main_cron_job() -> Result<()> {
    let db_pool = init_db_result().await?;
    // cron job continuous loop
    match start_main_cron_job(&db_pool).await {
        Ok(_) => {
            send_main_cron_message(format!("main_cron_job ended - {:?}", now_formated())).await;
        }
        Err(err) => {
            send_main_cron_message(format!("main_cron_job failed - {:?}", err)).await;
        }
    }
    db_pool.close().await;
    Ok(())
}

pub async fn start_main_cron_job(db_pool: &PgPool) -> anyhow::Result<()> {
    let start_datetime = now_utc_datetime();
    let news_cron_job = NewsCronJob {
        cron_type: CronType::Main.to_string(),
        started_at: start_datetime.unix_timestamp(),
        started_at_str: datetime_to_str(start_datetime),
        completed_at: None,
        completed_at_str: None,
        error: None,
    };

    let news_cron_job_db = insert_news_cron_job(db_pool, news_cron_job).await?;
    match main_cron_job(&db_pool).await {
        Ok(_) => {
            let completed_datetime = now_utc_datetime();
            update_news_cron_job_completed_at(
                &db_pool,
                news_cron_job_db.id,
                completed_datetime.unix_timestamp(),
                datetime_to_str(completed_datetime),
            )
            .await?;
        }
        Err(err) => {
            error!("main_cron_job failed: {:?}", err);
            update_news_cron_job_error(&db_pool, news_cron_job_db.id, err.to_string()).await?;
            send_main_cron_message(format!("main_cron_job failed: {:?}", err)).await;
        }
    }
    Ok(())
}

pub async fn main_cron_job(db_pool: &PgPool) -> Result<()> {
    info!("main_cron_job started - {:?}", now_formated());

    // Bluesky flow
    match init_bluesky_agent().await {
        Ok(bsky_agent) => {
            if let Err(err) = get_all_bsky_posts(db_pool, &bsky_agent).await {
                error!("get_all_bsky_posts error - {:?}", err);
            }
        }
        Err(err) => {
            error!("init_bluesky_agent error - {:?}", err);
        }
    }

    populate_news_feed_v1(db_pool).await?;
    Ok(())
}
