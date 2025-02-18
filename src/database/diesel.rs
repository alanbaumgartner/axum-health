use crate::database::Pingable;
use async_trait::async_trait;
use diesel::backend::DieselReserveSpecialization;
use diesel::r2d2::{ConnectionManager, Pool};

#[async_trait]
impl<Conn> Pingable for Pool<ConnectionManager<Conn>>
where
    Conn: diesel::r2d2::R2D2Connection + Send + 'static,
    Conn::Backend: DieselReserveSpecialization,
{
    async fn ping(&self) -> bool {
        match self.get() {
            Ok(mut conn) => conn.ping().is_ok(),
            Err(_) => false,
        }
    }
}

macro_rules! async_ping_impl {
    ($pool:ty) => {
        #[async_trait::async_trait]
        impl crate::database::Pingable for $pool {
            async fn ping(&self) -> bool {
                use diesel_async::pooled_connection::PoolableConnection;

                match self.get().await {
                    Ok(mut conn) => conn
                        .ping(&diesel_async::pooled_connection::RecyclingMethod::Verified)
                        .await
                        .is_ok(),
                    Err(_) => false,
                }
            }
        }
    };
}

#[cfg(feature = "diesel-bb8")]
mod bb8 {
    use diesel_async::pooled_connection::bb8::Pool;
    #[cfg(feature = "diesel-async-postgres")]
    async_ping_impl!(Pool<diesel_async::AsyncPgConnection>);
    #[cfg(feature = "diesel-async-mysql")]
    async_ping_impl!(Pool<diesel_async::AsyncMysqlConnection>);
}

#[cfg(feature = "diesel-deadpool")]
mod deadpool {
    use diesel_async::pooled_connection::deadpool::Pool;
    #[cfg(feature = "diesel-async-postgres")]
    async_ping_impl!(Pool<diesel_async::AsyncPgConnection>);
    #[cfg(feature = "diesel-async-mysql")]
    async_ping_impl!(Pool<diesel_async::AsyncMysqlConnection>);
}

#[cfg(feature = "diesel-mobc")]
mod mobc {
    use diesel_async::pooled_connection::mobc::Pool;
    #[cfg(feature = "diesel-async-postgres")]
    async_ping_impl!(Pool<diesel_async::AsyncPgConnection>);
    #[cfg(feature = "diesel-async-mysql")]
    async_ping_impl!(Pool<diesel_async::AsyncMysqlConnection>);
}
