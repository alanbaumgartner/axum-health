pub mod service;

#[cfg(feature = "diesel")]
pub mod diesel;
#[cfg(feature = "sea-orm")]
pub mod sea_orm;
#[cfg(feature = "sqlx")]
pub mod sqlx;

pub mod validation;

pub use crate::service::*;

pub async fn health(
    axum::Extension(health): axum::Extension<Health>,
) -> impl axum::response::IntoResponse {
    health.details().await
}
