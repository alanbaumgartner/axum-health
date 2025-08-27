use {crate::database::Pingable, async_trait::async_trait, elasticsearch::Elasticsearch};

#[async_trait]
impl Pingable for Elasticsearch {
    async fn ping(&self) -> bool {
        self.ping().send().await.is_ok()
    }
}
