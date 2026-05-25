#[cfg(feature = "diesel-r2d2")]
mod r2d2 {
    use {
        crate::{
            database::util::{get_mysql_container, get_postgres_container, health_details},
            test_indicator,
        },
        axum_health::prelude::*,
        diesel::r2d2::ConnectionManager,
        std::time::Duration,
    };

    #[tokio::test]
    async fn postgres() {
        let (url, _connection) = get_postgres_container().await;

        let manager = ConnectionManager::<diesel::PgConnection>::new(url.to_owned());
        let pool = diesel::r2d2::Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(5))
            .build(manager)
            .unwrap();

        let expected = health_details("postgres", HealthStatus::Up);

        let result = test_indicator(pool).await;

        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn mysql() {
        let (url, _container) = get_mysql_container().await;
        let manager = ConnectionManager::<diesel::MysqlConnection>::new(url.to_owned());
        let pool = diesel::r2d2::Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(5))
            .build(manager)
            .unwrap();

        let expected = health_details("mysql", HealthStatus::Up);

        let result = test_indicator(pool).await;

        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn sqlite() {
        let manager = ConnectionManager::<diesel::SqliteConnection>::new(":memory:");
        let pool = diesel::r2d2::Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(5))
            .build(manager)
            .unwrap();

        let expected = health_details("sqlite", HealthStatus::Up);

        let result = test_indicator(pool).await;

        assert_eq!(result, expected);
    }
}

#[cfg(feature = "diesel-bb8")]
mod bb8 {
    use {
        crate::{
            database::util::{get_mysql_container, get_postgres_container, health_details},
            test_indicator,
        },
        axum_health::prelude::*,
        diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager},
        std::time::Duration,
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

        let expected = health_details("postgres", HealthStatus::Up);

        let result = test_indicator(pool).await;

        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn mysql() {
        let (url, _container) = get_mysql_container().await;
        let manager =
            AsyncDieselConnectionManager::<diesel_async::AsyncMysqlConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::bb8::Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(5))
            .build(manager)
            .await
            .unwrap();

        let expected = health_details("mysql", HealthStatus::Up);

        let result = test_indicator(pool).await;

        assert_eq!(result, expected);
    }
}

#[cfg(feature = "diesel-deadpool")]
mod deadpool {
    use {
        crate::{
            database::util::{get_mysql_container, get_postgres_container, health_details},
            test_indicator,
        },
        axum_health::prelude::*,
        diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager},
    };

    #[tokio::test]
    async fn postgres() {
        let (url, _connection) = get_postgres_container().await;
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::deadpool::Pool::builder(manager)
            .max_size(1)
            .build()
            .unwrap();

        let expected = health_details("postgres", HealthStatus::Up);

        let result = test_indicator(pool).await;

        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn mysql() {
        let (url, _container) = get_mysql_container().await;
        let manager =
            AsyncDieselConnectionManager::<diesel_async::AsyncMysqlConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::deadpool::Pool::builder(manager)
            .max_size(1)
            .build()
            .unwrap();

        let expected = health_details("mysql", HealthStatus::Up);

        let result = test_indicator(pool).await;

        assert_eq!(result, expected);
    }
}

#[cfg(feature = "diesel-mobc")]
mod mobc {
    use {
        crate::{
            database::util::{get_mysql_container, get_postgres_container, health_details},
            test_indicator,
        },
        axum_health::prelude::*,
        diesel_async::{
            AsyncMysqlConnection, AsyncPgConnection,
            pooled_connection::AsyncDieselConnectionManager,
        },
    };

    #[tokio::test]
    async fn postgres() {
        let (url, _connection) = get_postgres_container().await;
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::mobc::Pool::builder()
            .max_open(1)
            .build(manager);

        let expected = health_details("postgres", HealthStatus::Up);

        let result = test_indicator(pool).await;

        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn mysql() {
        let (url, _connection) = get_mysql_container().await;
        let manager = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::mobc::Pool::builder()
            .max_open(1)
            .build(manager);

        let expected = health_details("mysql", HealthStatus::Up);

        let result = test_indicator(pool).await;

        assert_eq!(result, expected);
    }
}
