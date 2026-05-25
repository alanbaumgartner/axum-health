use {
    axum::{Router, routing::get},
    axum_health::prelude::*,
    sea_orm::Database,
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let database_connection = Database::connect(":memory:").await.unwrap();

    let router = Router::new()
        .route("/health", get(health_check))
        .layer(
            Health::builder()
                .with_ping()
                .with_indicator(database_connection.clone())
                .build(),
        )
        .with_state(database_connection);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
