use {
    axum::{Router, routing::get},
    axum_health::prelude::*,
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/health", get(health_check)).layer(
        Health::builder()
            .with_ping()
            .with_indicator(PingHealthIndicator.named("other ping".to_string()))
            .build(),
    );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // GET http://localhost:3000/health
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
