#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;

pub mod web;

#[tokio::main]
pub async fn main() {
    let _ = dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    web::serve().await;
}
