use {
    axum::{Router, routing::get},
    axum_health::prelude::*,
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    let router = Router::new()
        .route("/health", get(health_check))
        .layer(
            Health::builder()
                .with_ping()
                .with_indicator(client.clone())
                .build(),
        )
        .with_state(client);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
