use {
    crate::database::Pingable,
    async_trait::async_trait,
    sqlx::{pool::Pool, Connection, Database},
};

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
