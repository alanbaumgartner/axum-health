use {
    crate::{
        database::util::{get_mongo_container, health_details},
        test_indicator,
    },
    axum_health::prelude::*,
    mongodb::Client,
};

#[tokio::test]
async fn mongo() {
    let (url, _container) = get_mongo_container().await;
    let client = Client::with_uri_str(url).await.unwrap();

    let result = test_indicator(client).await;
    let expected = health_details("mongo", HealthStatus::Up);

    assert_eq!(result.status, expected.status);

    let detail = result.components.get("mongo").unwrap();

    assert_eq!(detail.status, HealthStatus::Up);
    assert!(detail.details.contains_key("max_wire_version"));
    assert!(detail.details.contains_key("databases"));
}
