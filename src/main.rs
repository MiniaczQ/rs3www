mod adapter;
mod config;
mod content_type;
mod directory;
mod error;
mod s3;
mod tracing;

use axum::Router;

use crate::{s3::S3Service, tracing::init_tracing};

#[tokio::main]
async fn main() {
    init_tracing();
    let config = self::config::config();
    let app = Router::new().fallback_service(S3Service::new(&config));
    let listener = tokio::net::TcpListener::bind((config.service.ip, config.service.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
