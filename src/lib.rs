pub mod service;

pub mod database;

pub use crate::service::*;

pub async fn health(
    axum::Extension(health): axum::Extension<Health>,
) -> impl axum::response::IntoResponse {
    health.details().await
}
