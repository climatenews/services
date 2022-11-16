use crate::news_feed::hourly_cron_job::hourly_cron_job;
use actix_web::{get, App, HttpResponse, HttpServer, Result};
use chrono::Local;
use chrono::Utc;
use db::init_env;
use db::models::news_cron_job::NewsCronJob;
use db::sql::news_cron_job::insert_news_cron_job;
use db::sql::news_cron_job::update_news_cron_job_completed_at;
use db::sql::news_cron_job::update_news_cron_job_error;
use db::util::convert::datetime_to_str;
use db::util::convert::now_utc_datetime;
use db::util::db::init_db;
use log::error;
use log::info;
use sqlx::PgPool;
use std::env;
use tokio_schedule::{every, Job};

pub mod language;
pub mod news_feed;
pub mod openai;
pub mod twitter;
pub mod util;

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("success".to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();
    // Start scheduler on a new thread
    actix_rt::spawn(async move {
        start_scheduler().await;
    });

    let host = env::var("CRON_HOST").expect("HOST is not set");
    let port = env::var("CRON_PORT").expect("PORT is not set");

    // Start Web server
    HttpServer::new(|| App::new().service(health))
        .bind(format!("{}:{}", host, port))
        .unwrap_or_else(|_| panic!("Couldn't bind to port {}", port))
        .run()
        .await
}

pub async fn start_scheduler() {
    info!("start_scheduler - {:?}", Local::now());
    let db_pool = init_db().await;

    if let Err(err) = start_cron_job(&db_pool).await {
        error!("start_cron_job failed: {:?}", err);
    }

    let scheduler = every(2).hours().in_timezone(&Utc).perform(|| async {
        if let Err(err) = start_cron_job(&db_pool).await {
            error!("start_cron_job failed: {:?}", err);
        }
    });
    scheduler.await;
}

pub async fn start_cron_job(db_pool: &PgPool) -> anyhow::Result<()> {
    let start_datetime = now_utc_datetime();
    let news_cron_job = NewsCronJob {
        started_at: start_datetime.unix_timestamp(),
        started_at_str: datetime_to_str(start_datetime),
        completed_at: None,
        completed_at_str: None,
        error: None,
    };

    let news_cron_job_db = insert_news_cron_job(db_pool, news_cron_job).await?;
    match hourly_cron_job(&db_pool).await {
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
            update_news_cron_job_error(&db_pool, news_cron_job_db.id, err.to_string()).await?;
            error!("hourly_cron_job failed: {:?}", err);
        }
    }
    Ok(())
}
