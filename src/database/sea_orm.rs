use crate::database::Pingable;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

#[async_trait]
impl Pingable for DatabaseConnection {
    async fn ping(&self) -> bool {
        self.ping().await.is_ok()
    }
}
