use {
    crate::{
        database::util::{get_elasticsearch_container, health_details},
        test_indicator,
    },
    axum_health::prelude::*,
    elasticsearch::{http::transport::Transport, Elasticsearch},
};

#[tokio::test]
async fn elasticsearch() {
    let (url, _container) = get_elasticsearch_container().await;
    let transport = Transport::single_node(&url).unwrap();
    let client = Elasticsearch::new(transport);

    let expected = health_details("elasticsearch", HealthStatus::Up);
    let result = test_indicator(client).await;

    assert_eq!(result, expected);
}
