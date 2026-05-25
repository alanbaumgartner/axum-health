use {
    axum::{Router, routing::get},
    axum_health::prelude::*,
    mongodb::{
        Client,
        options::{ClientOptions, Credential},
    },
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let options = ClientOptions::builder()
        .credential(
            Credential::builder()
                .username(Some("mongo".to_owned()))
                .password(Some("mongo".to_owned()))
                .build(),
        )
        .build();

    let client = Client::with_options(options).unwrap();

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
