#[cfg(feature = "local")]
mod local {
    use axum::http::StatusCode;
    use axum::routing::get;
    use axum::Router;
    use axum_health::database::DatabaseHealthIndicator;
    use axum_health::service::HealthIndicator;
    use axum_health::{Health, HealthDetail, HealthDetails, HealthStatus};
    use axum_test::TestServer;
    use diesel::r2d2::ConnectionManager;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::AsyncPgConnection;
    use sea_orm::DatabaseConnection;
    use std::collections::HashMap;
    use std::time::Duration;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::ContainerAsync;
    use testcontainers_modules::postgres::Postgres;

    async fn diesel(url: &str) -> impl HealthIndicator {
        let manager = ConnectionManager::<diesel::PgConnection>::new(url.to_owned());
        let pool = diesel::r2d2::Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(5))
            .build(manager)
            .unwrap();
        DatabaseHealthIndicator::new("diesel-postgres".to_owned(), pool)
    }

    async fn async_diesel_bb8(url: &str) -> impl HealthIndicator {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::bb8::Pool::builder()
            .max_size(1)
            .connection_timeout(Duration::from_secs(5))
            .build(manager)
            .await
            .unwrap();
        DatabaseHealthIndicator::new("diesel-bb8".to_owned(), pool)
    }

    async fn async_diesel_deadpool(url: &str) -> impl HealthIndicator {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::deadpool::Pool::builder(manager)
            .max_size(1)
            .build()
            .unwrap();
        DatabaseHealthIndicator::new("diesel-deadpool".to_owned(), pool)
    }

    async fn async_diesel_mobc(url: &str) -> impl HealthIndicator {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
        let pool = diesel_async::pooled_connection::mobc::Pool::builder().build(manager);
        DatabaseHealthIndicator::new("diesel-mobc".to_owned(), pool)
    }

    async fn sqlx(url: &str) -> impl HealthIndicator {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect(&url)
            .await
            .unwrap();
        DatabaseHealthIndicator::new("sqlx".to_owned(), pool)
    }

    async fn sea_orm(url: &str) -> impl HealthIndicator {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect(&url)
            .await
            .unwrap();
        let pool = DatabaseConnection::from(pool);
        DatabaseHealthIndicator::new("sea-orm".to_owned(), pool)
    }

    #[tokio::test]
    async fn test_all() {
        let container = Postgres::default().start().await.unwrap();
        container.start().await.unwrap();

        let url = get_url(&container).await;
        let url = url.as_str();

        let health = Health::builder()
            .with_indicator(diesel(url).await)
            .with_indicator(async_diesel_bb8(url).await)
            .with_indicator(async_diesel_deadpool(url).await)
            .with_indicator(async_diesel_mobc(url).await)
            .with_indicator(sqlx(url).await)
            .with_indicator(sea_orm(url).await)
            .build();

        let router = Router::new()
            .route("/health", get(axum_health::health))
            .layer(health);

        let server = TestServer::new(router).unwrap();

        let response = server.get("/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let body = response.json::<HealthDetails>();

        let expected = HealthDetails {
            status: HealthStatus::Up,
            components: HashMap::from_iter([
                ("diesel-postgres".to_owned(), HealthDetail::up()),
                ("diesel-bb8".to_owned(), HealthDetail::up()),
                ("diesel-deadpool".to_owned(), HealthDetail::up()),
                ("diesel-mobc".to_owned(), HealthDetail::up()),
                ("sqlx".to_owned(), HealthDetail::up()),
                ("sea-orm".to_owned(), HealthDetail::up()),
            ]),
        };
        assert_eq!(body, expected);

        container.stop().await.unwrap();

        let response = server.get("/health").await;
        assert_eq!(response.status_code(), StatusCode::SERVICE_UNAVAILABLE);
        let body = response.json::<HealthDetails>();

        let expected = HealthDetails {
            status: HealthStatus::Down,
            components: HashMap::from_iter([
                ("diesel-postgres".to_owned(), HealthDetail::down()),
                ("diesel-bb8".to_owned(), HealthDetail::down()),
                ("diesel-deadpool".to_owned(), HealthDetail::down()),
                ("diesel-mobc".to_owned(), HealthDetail::down()),
                ("sqlx".to_owned(), HealthDetail::down()),
                ("sea-orm".to_owned(), HealthDetail::down()),
            ]),
        };
        assert_eq!(body, expected);
    }

    async fn get_url(container: &ContainerAsync<Postgres>) -> String {
        format!(
            "postgresql://postgres:postgres@{}:{}/postgres",
            container.get_host().await.unwrap(),
            container.get_host_port_ipv4(5432).await.unwrap()
        )
    }
}
