use {
    axum::{routing::get, Router},
    axum_health::prelude::*,
    elasticsearch::{http::transport::Transport, Elasticsearch},
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let transport = Transport::single_node("http://localhost:9200").unwrap();
    let client = Elasticsearch::new(transport);

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
