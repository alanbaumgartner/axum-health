#[cfg(feature = "diesel-bb8")]
pub use bb8::*;
#[cfg(feature = "diesel-deadpool")]
pub use deadpool::*;
#[cfg(feature = "diesel-mobc")]
pub use mobc::*;
#[cfg(feature = "diesel-r2d2")]
pub use r2d2::*;

#[cfg(feature = "diesel-r2d2")]
mod r2d2 {
    use {
        crate::{indicator::HealthDetail, prelude::HealthIndicator},
        diesel::r2d2::{ConnectionManager, Pool, R2D2Connection},
    };

    macro_rules! diesel_health_indicator {
        ($ty:ty, $name:literal) => {
            #[async_trait::async_trait]
            impl HealthIndicator for Pool<ConnectionManager<$ty>> {
                fn name(&self) -> String {
                    $name.to_owned()
                }

                async fn details(&self) -> HealthDetail {
                    match self.get() {
                        Ok(mut conn) => match conn.ping() {
                            Ok(_) => HealthDetail::up(),
                            Err(_) => HealthDetail::down(),
                        },
                        Err(_) => HealthDetail::down(),
                    }
                }
            }
        };
    }

    diesel_health_indicator!(diesel::MysqlConnection, "mysql");
    diesel_health_indicator!(diesel::PgConnection, "postgres");
    diesel_health_indicator!(diesel::SqliteConnection, "sqlite");
}

#[cfg(feature = "diesel-async")]
macro_rules! diesel_async_health_indicator {
    ($ty:ty, $name:literal) => {
        #[async_trait::async_trait]
        impl crate::prelude::HealthIndicator for $ty {
            fn name(&self) -> String {
                $name.to_owned()
            }

            async fn details(&self) -> crate::prelude::HealthDetail {
                match self.get().await {
                    Ok(mut conn) => {
                        match conn
                            .ping(&diesel_async::pooled_connection::RecyclingMethod::Verified)
                            .await
                        {
                            Ok(_) => crate::prelude::HealthDetail::up(),
                            Err(_) => crate::prelude::HealthDetail::down(),
                        }
                    }
                    Err(_) => crate::prelude::HealthDetail::down(),
                }
            }
        }
    };
}

#[cfg(feature = "diesel-bb8")]
mod bb8 {
    use diesel_async::pooled_connection::PoolableConnection;
    diesel_async_health_indicator!(
        diesel_async::pooled_connection::bb8::Pool<diesel_async::AsyncMysqlConnection>,
        "mysql"
    );
    diesel_async_health_indicator!(
        diesel_async::pooled_connection::bb8::Pool<diesel_async::AsyncPgConnection>,
        "postgres"
    );
}

#[cfg(feature = "diesel-deadpool")]
mod deadpool {
    use diesel_async::pooled_connection::PoolableConnection;
    diesel_async_health_indicator!(
        diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncMysqlConnection>,
        "mysql"
    );
    diesel_async_health_indicator!(
        diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>,
        "postgres"
    );
}

#[cfg(feature = "diesel-mobc")]
mod mobc {
    use diesel_async::pooled_connection::PoolableConnection;
    diesel_async_health_indicator!(
        diesel_async::pooled_connection::mobc::Pool<diesel_async::AsyncMysqlConnection>,
        "mysql"
    );
    diesel_async_health_indicator!(
        diesel_async::pooled_connection::mobc::Pool<diesel_async::AsyncPgConnection>,
        "postgres"
    );
}
