use actix_web::{get, App, HttpResponse, HttpServer, Result};
use chrono::Utc;
use db::init_env;
use news_feed::constants::RESET_DB;
use std::env;
use tokio_schedule::{every, Job};
use twitter::db::init_db;

use crate::news_feed::hourly_cron_job::hourly_cron_job;

pub mod news_feed;
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
    let db_pool = init_db(RESET_DB).await;
    #[cfg(debug_assertions)]
    hourly_cron_job(&db_pool).await; // only run in debug mode

    let every_second = every(2)
        .hours()
        .in_timezone(&Utc)
        .perform(|| async { hourly_cron_job(&db_pool).await });
    every_second.await;
}
