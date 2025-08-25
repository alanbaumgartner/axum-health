#[cfg(feature = "diesel-r2d2")]
mod r2d2 {
    use {
        crate::{database::util::get_postgres_container, test_indicator},
        axum_health::{
            database::DatabaseHealthIndicator, HealthDetail, HealthDetails, HealthStatus,
        },
        diesel::r2d2::ConnectionManager,
        std::{collections::BTreeMap, time::Duration},
    };

    #[tokio::test]
    async fn postgres() {
        let (url, connection) = get_postgres_container().await;

        let manager = ConnectionManager::<diesel::PgConnection>::new(url.to_owned());
        let pool = diesel::r2d2::Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(5))
            .build(manager)
            .unwrap();

        let indicator = DatabaseHealthIndicator(pool);

        let expected = HealthDetails {
            status: HealthStatus::Up,
            components: BTreeMap::from_iter([("database".to_owned(), HealthDetail::up())]),
        };

        let result = test_indicator(indicator).await;

        assert_eq!(result, expected);
    }
}

#[cfg(feature = "diesel-bb8")]
mod bb8 {
    use {
        crate::{database::util::get_postgres_container, test_indicator},
        axum_health::{
            database::DatabaseHealthIndicator, HealthDetail, HealthDetails, HealthStatus,
        },
        diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection},
        std::{collections::BTreeMap, time::Duration},
    };

    #[tokio::test]
    async fn postgres() {
        let (url, _connection) = get_postgres_container().await;
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::bb8::Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(5))
            .build(manager)
            .await
            .unwrap();

        let indicator = DatabaseHealthIndicator(pool);

        let expected = HealthDetails {
            status: HealthStatus::Up,
            components: BTreeMap::from_iter([("database".to_owned(), HealthDetail::up())]),
        };

        let result = test_indicator(indicator).await;

        assert_eq!(result, expected);
    }
}

#[cfg(feature = "diesel-deadpool")]
mod deadpool {
    use {
        crate::{database::util::get_postgres_container, test_indicator},
        axum_health::{
            database::DatabaseHealthIndicator, HealthDetail, HealthDetails, HealthStatus,
        },
        diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection},
        std::collections::BTreeMap,
    };

    #[tokio::test]
    async fn postgres() {
        let (url, _connection) = get_postgres_container().await;
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::deadpool::Pool::builder(manager)
            .max_size(1)
            .build()
            .unwrap();

        let indicator = DatabaseHealthIndicator(pool);

        let expected = HealthDetails {
            status: HealthStatus::Up,
            components: BTreeMap::from_iter([("database".to_owned(), HealthDetail::up())]),
        };

        let result = test_indicator(indicator).await;

        assert_eq!(result, expected);
    }
}

#[cfg(feature = "diesel-mobc")]
mod mobc {
    use {
        crate::{database::util::get_postgres_container, test_indicator},
        axum_health::{
            database::DatabaseHealthIndicator, HealthDetail, HealthDetails, HealthStatus,
        },
        diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection},
        std::collections::BTreeMap,
    };

    #[tokio::test]
    async fn postgres() {
        let (url, _connection) = get_postgres_container().await;
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::mobc::Pool::builder()
            .max_open(1)
            .build(manager);

        let indicator = DatabaseHealthIndicator(pool);

        let expected = HealthDetails {
            status: HealthStatus::Up,
            components: BTreeMap::from_iter([("database".to_owned(), HealthDetail::up())]),
        };

        let result = test_indicator(indicator).await;

        assert_eq!(result, expected);
    }
}
