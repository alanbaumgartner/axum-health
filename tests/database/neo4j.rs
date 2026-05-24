use {
    crate::{
        database::util::{get_neo4j_container, health_details},
        test_indicator,
    },
    axum_health::prelude::*,
    neo4rs::Graph,
};

#[tokio::test]
async fn neo4j() {
    let (url, _container) = get_neo4j_container().await;

    let graph = Graph::new(url, "neo4j", "password").unwrap();

    let result = test_indicator(graph).await;
    let expected = health_details("neo4j", HealthStatus::Up);

    assert_eq!(result.status, expected.status);

    let detail = result.components.get("neo4j").unwrap();

    assert_eq!(detail.status, HealthStatus::Up);
    assert!(detail.details.contains_key("version"));
    assert!(detail.details.contains_key("edition"));
}
