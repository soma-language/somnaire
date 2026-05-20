#![feature(try_trait_v2)]

use dotenvy::dotenv;
use sea_orm::Database;
use tracing_subscriber::EnvFilter;

pub mod auth;
pub mod schema;
pub mod web;

#[tokio::main]
pub async fn main() {
    let _ = dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    web::serve(db).await;
}
