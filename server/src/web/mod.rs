use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod api;
pub mod routing;

pub async fn serve(db: DatabaseConnection) {
    let address = std::env::var("API_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind rest API address");
    let router = routing::router(db);
    tracing::info!("Serving on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Failed to run server");
}
