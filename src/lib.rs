use {
    crate::prelude::Health,
    axum::{response::IntoResponse, Extension},
};

pub mod service;
pub mod database;
pub mod indicator;
pub mod indicators;

pub mod prelude {
    #[allow(unused_imports)]
    pub use crate::database::*;
    pub use {
        super::health_check,
        crate::{indicator::*, indicators::*, service::*},
    };
}

pub async fn health_check(Extension(health): Extension<Health>) -> impl IntoResponse {
    health.details().await
}
