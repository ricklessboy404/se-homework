use sqlx::mysql::MySqlPool;
use std::env;

pub async fn create_pool() -> MySqlPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create database connection pool")
}
    