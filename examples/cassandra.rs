use {
    axum::{routing::get, Router},
    axum_health::prelude::*,
    scylla::client::{session::Session, session_builder::SessionBuilder},
    std::sync::Arc,
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await
        .unwrap();

    let session = Arc::new(session);

    let router = Router::new()
        .route("/health", get(health_check))
        .layer(
            Health::builder()
                .with_ping()
                .with_indicator(session.clone())
                .build(),
        )
        .with_state(session);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
