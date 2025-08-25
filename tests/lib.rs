use {
    axum::{http::StatusCode, routing::get, Router},
    axum_health::{Health, HealthDetails, HealthIndicator},
    axum_test::TestServer,
};

mod database;

async fn test_indicator(indicator: impl HealthIndicator + Send + Sync + 'static) -> HealthDetails {
    let health = Health::builder().with_indicator(indicator).build();

    let router = Router::new()
        .route("/health", get(axum_health::health))
        .layer(health);

    let server = TestServer::new(router).unwrap();

    let response = server.get("/health").await;
    assert_eq!(response.status_code(), StatusCode::OK);
    response.json::<HealthDetails>()
}
