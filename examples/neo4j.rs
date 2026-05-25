use {
    axum::{routing::get, Router},
    axum_health::prelude::*,
    neo4rs::Graph,
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let uri = "127.0.0.1:7687";
    let user = "neo4j";
    let pass = "neo4j";
    let graph = Graph::new(uri, user, pass).unwrap();

    let router = Router::new()
        .route("/health", get(health_check))
        .layer(
            Health::builder()
                .with_ping()
                .with_indicator(graph.clone())
                .build(),
        )
        .with_state(graph);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // GET http://localhost:3000/health
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}
