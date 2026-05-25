use {
    crate::{
        database::util::{get_redis_container, health_details},
        test_indicator,
    },
    axum_health::prelude::*,
    redis::Client,
};

#[tokio::test]
async fn redis() {
    let (url, _container) = get_redis_container().await;

    let client = Client::open(url).unwrap();

    let expected = health_details("redis", HealthStatus::Up);
    let result = test_indicator(client).await;

    assert_eq!(result, expected);
}
