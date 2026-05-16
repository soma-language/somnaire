use dotenvy::dotenv;
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
    web::serve().await;
}
