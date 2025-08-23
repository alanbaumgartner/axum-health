pub mod service;

#[cfg(feature = "database")]
pub mod database;

pub mod indicators;

#[cfg(feature = "kafka")]
pub mod kafka;

pub use crate::service::*;

pub async fn health(
    axum::Extension(health): axum::Extension<Health>,
) -> impl axum::response::IntoResponse {
    health.details().await
}
