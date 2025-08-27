use {
    axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router},
    axum_health::prelude::*,
    diesel::r2d2::{ConnectionManager, Pool},
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let manager = ConnectionManager::<diesel::SqliteConnection>::new(":memory:");
    let pool = Pool::builder().build(manager).unwrap();

    // Clone the pool!
    let indicator = DatabaseHealthIndicator(pool.clone());

    let router = Router::new()
        .route("/health", get(axum_health::health))
        .layer(
            Health::builder()
                .with_ping()
                .with_indicator(indicator)
                .build(),
        )
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
