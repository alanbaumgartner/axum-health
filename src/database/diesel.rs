macro_rules! async_ping_impl {
    ($pool:tt) => {
        #[async_trait::async_trait]
        impl<Conn> crate::database::Pingable for $pool<Conn>
        where
            Conn: diesel_async::pooled_connection::PoolableConnection + Send + 'static,
            diesel::dsl::select<diesel::dsl::AsExprOf<i32, diesel::sql_types::Integer>>:
                diesel_async::methods::ExecuteDsl<Conn>,
            diesel::query_builder::SqlQuery: diesel::query_builder::QueryFragment<Conn::Backend>,
        {
            async fn ping(&self) -> bool {
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

#[cfg(feature = "diesel-r2d2")]
mod r2d2 {
    use diesel::r2d2::{ConnectionManager, Pool};
    #[async_trait::async_trait]
    impl<Conn> crate::database::Pingable for Pool<ConnectionManager<Conn>>
    where
        Conn: diesel::r2d2::R2D2Connection + Send + 'static,
        Conn::Backend: diesel::backend::DieselReserveSpecialization,
    {
        async fn ping(&self) -> bool {
            match self.get() {
                Ok(mut conn) => conn.ping().is_ok(),
                Err(_) => false,
            }
        }
    }
}

#[cfg(feature = "diesel-bb8")]
mod bb8 {
    use diesel_async::pooled_connection::bb8::Pool;
    async_ping_impl!(Pool);
}

#[cfg(feature = "diesel-deadpool")]
mod deadpool {
    use diesel_async::pooled_connection::deadpool::Pool;
    async_ping_impl!(Pool);
}

#[cfg(feature = "diesel-mobc")]
mod mobc {
    use diesel_async::pooled_connection::mobc::Pool;
    async_ping_impl!(Pool);
}
