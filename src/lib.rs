#![allow(unused_imports)]

use {
    crate::prelude::Health,
    axum::{response::IntoResponse, Extension},
};

pub mod service;

#[cfg(feature = "database")]
pub mod database;

pub mod indicator;
pub mod indicators;

pub mod prelude {
    #[cfg(feature = "database")]
    pub use crate::database::*;
    pub use {
        super::health_check,
        crate::{indicator::*, indicators::*, service::*},
    };
}

pub async fn health_check(Extension(health): Extension<Health>) -> impl IntoResponse {
    health.details().await
}
