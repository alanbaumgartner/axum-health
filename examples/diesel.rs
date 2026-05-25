use {
    axum::{Router, routing::get},
    axum_health::prelude::*,
    diesel::r2d2::{ConnectionManager, Pool},
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let manager = ConnectionManager::<diesel::SqliteConnection>::new(":memory:");
    let pool = Pool::builder().build(manager).unwrap();

    let router = Router::new()
        .route("/health", get(health_check))
        .layer(
            Health::builder()
                .with_ping()
                .with_indicator(pool.clone())
                .build(),
        )
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
