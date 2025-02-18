use crate::database::Pingable;
use async_trait::async_trait;
use sqlx::pool::Pool;
use sqlx::{Connection, Database};

#[async_trait]
impl<DB> Pingable for Pool<DB>
where
    DB: Database,
{
    async fn ping(&self) -> bool {
        match self.acquire().await {
            Ok(mut conn) => conn.ping().await.is_ok(),
            Err(_) => false,
        }
    }
}
