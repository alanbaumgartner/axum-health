pub mod service;

#[cfg(feature = "_diesel")]
pub mod diesel;
#[cfg(feature = "_sea-orm")]
pub mod sea_orm;
#[cfg(feature = "_sqlx")]
pub mod sqlx;

pub mod validation;

#[cfg(feature = "_diesel")]
pub use diesel::*;
#[cfg(feature = "_sea-orm")]
pub use sea_orm::*;
#[cfg(feature = "_sqlx")]
pub use sqlx::*;

pub use crate::service::*;

pub async fn health(
    axum::Extension(health): axum::Extension<Health>,
) -> impl axum::response::IntoResponse {
    health.details().await
}
