use {
    crate::prelude::{HealthDetail, HealthIndicator},
    async_trait::async_trait,
    sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend},
};

#[async_trait]
impl HealthIndicator for DatabaseConnection {
    fn name(&self) -> String {
        let backend = match self.get_database_backend() {
            DbBackend::MySql => "mysql",
            DbBackend::Postgres => "postgres",
            DbBackend::Sqlite => "sqlite",
        };

        String::from(backend)
    }

    async fn details(&self) -> HealthDetail {
        match self.ping().await {
            Ok(_) => HealthDetail::up(),
            Err(_) => HealthDetail::down(),
        }
    }
}
