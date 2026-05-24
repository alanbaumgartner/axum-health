use {
    crate::{indicator::HealthDetail, prelude::HealthIndicator},
    async_trait::async_trait,
    elasticsearch::Elasticsearch,
};

#[async_trait]
impl HealthIndicator for Elasticsearch {
    fn name(&self) -> String {
        "elasticsearch".to_owned()
    }

    async fn details(&self) -> HealthDetail {
        match self.ping().send().await {
            Ok(_) => HealthDetail::up(),
            Err(_) => HealthDetail::down(),
        }
    }
}
