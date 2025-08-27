use {
    crate::prelude::Health,
    axum::{response::IntoResponse, Extension},
};

pub mod service;

#[cfg(feature = "database")]
pub mod database;

pub mod indicators;

pub mod prelude {
    #[cfg(feature = "database")]
    pub use crate::database::*;
    pub use {
        super::health,
        crate::{indicators::*, service::*},
    };
}

pub async fn health(Extension(health): Extension<Health>) -> impl IntoResponse {
    health.details().await
}
