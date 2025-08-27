use {
    crate::prelude::Health,
    axum::{response::IntoResponse, Extension},
};

pub mod service;

#[cfg(feature = "database")]
pub mod database;

pub mod indicators;

#[cfg(feature = "kafka")]
pub mod kafka;

pub mod prelude {
    #[cfg(any(feature = "sqlx", feature = "diesel", feature = "sqlx"))]
    pub use crate::database::*;
    pub use {
        super::health,
        crate::{indicators::*, service::*},
    };
}

pub async fn health(Extension(health): Extension<Health>) -> impl IntoResponse {
    health.details().await
}
