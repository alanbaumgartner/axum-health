use {
    crate::{indicator::HealthDetail, prelude::HealthIndicator},
    async_trait::async_trait,
    sqlx::{Connection, pool::Pool},
};

macro_rules! sqlx_health_indicator {
    ($ty:ty, $name:literal) => {
        #[async_trait]
        impl HealthIndicator for Pool<$ty> {
            fn name(&self) -> String {
                $name.to_owned()
            }

            async fn details(&self) -> HealthDetail {
                match self.acquire().await {
                    Ok(mut conn) => match conn.ping().await {
                        Ok(_) => HealthDetail::up(),
                        Err(_) => HealthDetail::down(),
                    },
                    Err(_) => HealthDetail::down(),
                }
            }
        }
    };
}

sqlx_health_indicator!(sqlx::Postgres, "postgres");
sqlx_health_indicator!(sqlx::Sqlite, "sqlite");
sqlx_health_indicator!(sqlx::MySql, "mysql");
