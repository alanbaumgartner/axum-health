use {
    axum::{routing::get, Router},
    axum_health::prelude::*,
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    // Clone the pool!
    let indicator = DatabaseHealthIndicator(client.clone());

    let router = Router::new()
        .route("/health", get(axum_health::health))
        .layer(
            Health::builder()
                .with_ping()
                .with_indicator(indicator)
                .build(),
        )
        .with_state(client);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
