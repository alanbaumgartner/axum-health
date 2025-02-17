use crate::service::{HealthDetail, HealthIndicator};
use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection};

pub struct SeaOrmHealthIndicator {
    pub(crate) name: String,
    pub(crate) connection: DatabaseConnection,
}

impl SeaOrmHealthIndicator {
    pub fn new(connection: DatabaseConnection) -> Self {
        let backend = connection.get_database_backend();

        let name = match backend {
            DatabaseBackend::MySql => "sea-orm-mysql",
            DatabaseBackend::Postgres => "sea-orm-postgres",
            DatabaseBackend::Sqlite => "sea-orm-sqlite",
        }
        .to_owned();

        Self { name, connection }
    }

    pub fn new_named(name: String, connection: DatabaseConnection) -> Self {
        Self { name, connection }
    }
}

#[async_trait]
impl HealthIndicator for SeaOrmHealthIndicator {
    fn name(&self) -> String {
        self.name.clone()
    }

    async fn details(&self) -> HealthDetail {
        let backend = self.connection.get_database_backend();

        let query = match backend {
            DatabaseBackend::MySql => crate::validation::mysql::VALIDATION_QUERY,
            DatabaseBackend::Postgres => crate::validation::postgres::VALIDATION_QUERY,
            DatabaseBackend::Sqlite => crate::validation::sqlite::VALIDATION_QUERY,
        };

        let result = self.connection.execute_unprepared(query).await.is_ok();
        if result {
            HealthDetail::up()
        } else {
            HealthDetail::down()
        }
    }
}
