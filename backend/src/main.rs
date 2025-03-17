use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
use db::create_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = create_pool().await;
    // Quick test to check connection
    sqlx::query!("SELECT 1 AS test")
        .fetch_one(&pool)
        .await
        .expect("Failed test query");

    println!("Connected to DB successfully! 🚀");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(|| async { "Ivansearch backend is live 🚀" }));

    let host = env::var("HOST").unwrap_or("0.0.0.0".into());
    let port = env::var("PORT").unwrap_or("8000".into());
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
