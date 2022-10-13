use crate::init_db_pool;
use sqlx::PgPool;

pub async fn init_db() -> PgPool {
    let db_pool = init_db_pool().await.unwrap();
    db_pool
}