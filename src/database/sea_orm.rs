use {crate::database::Pingable, async_trait::async_trait, sea_orm::DatabaseConnection};

#[async_trait]
impl Pingable for DatabaseConnection {
    async fn ping(&self) -> bool {
        self.ping().await.is_ok()
    }
}
