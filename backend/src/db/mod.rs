use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn create_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}
