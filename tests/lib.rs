use {
    axum::{Router, routing::get},
    axum_health::prelude::*,
    axum_test::TestServer,
};

mod database;

async fn test_indicator(indicator: impl HealthIndicator + Send + Sync + 'static) -> HealthDetails {
    let health = Health::builder().with_indicator(indicator).build();

    let router = Router::new()
        .route("/health", get(health_check))
        .layer(health);

    let server = TestServer::new(router);

    let response = server.get("/health").await;

    response.json::<HealthDetails>()
}
