use {
    crate::prelude::{HealthDetail, HealthIndicator},
    async_trait::async_trait,
    scylla::client::session::Session,
    std::sync::Arc,
};

const PING: &str = "SELECT now() FROM system.local;";

#[async_trait]
impl HealthIndicator for Arc<Session> {
    fn name(&self) -> String {
        "cassandra".to_owned()
    }

    async fn details(&self) -> HealthDetail {
        match self.query_unpaged(PING, []).await {
            Ok(_result) => HealthDetail::up(),
            Err(_) => HealthDetail::down(),
        }
    }
}
