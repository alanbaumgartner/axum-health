use {crate::database::Pingable, async_trait::async_trait, mongodb::Client};

#[async_trait]
impl Pingable for Client {
    async fn ping(&self) -> bool {
        self.list_databases().await.is_ok()
    }
}
