use {
    crate::database::Pingable, async_trait::async_trait, scylla::client::session::Session,
    std::sync::Arc,
};

const PING: &'static str = "SELECT now() FROM system.local;";

#[async_trait]
impl Pingable for Arc<Session> {
    async fn ping(&self) -> bool {
        self.query_unpaged(PING, []).await.is_ok()
    }
}
