use {
    crate::{
        database::util::{get_cassandra_container, health_details},
        test_indicator,
    },
    axum_health::prelude::*,
    scylla::client::session_builder::SessionBuilder,
    std::sync::Arc,
};

#[tokio::test]
async fn cassandra() {
    let (url, _container) = get_cassandra_container().await;

    let session = Arc::new(SessionBuilder::new().known_node(url).build().await.unwrap());

    let expected = health_details("cassandra", HealthStatus::Up);
    let result = test_indicator(session).await;

    assert_eq!(result, expected);
}
