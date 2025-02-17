#[cfg(feature = "postgres")]
mod test {
    use axum::routing::get;
    use axum::Router;
    use axum_health::diesel::DieselHealthIndicator;
    use axum_health::health;
    use axum_health::sea_orm::SeaOrmHealthIndicator;
    use axum_health::service::{
        Health, HealthDetail, HealthDetails, HealthIndicator, HealthStatus,
    };
    use axum_health::sqlx::SqlxHealthIndicator;
    use axum_test::TestServer;
    use diesel::r2d2::{ConnectionManager, Pool};
    use sea_orm::DatabaseConnection;
    use std::collections::HashMap;
    use testcontainers::ContainerAsync;
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;

    #[cfg(feature = "diesel-postgres")]
    #[tokio::test]
    async fn test_diesel_up() {
        let container = Postgres::default().start().await.unwrap();
        container.start().await.unwrap();

        let url = get_url(&container).await;

        let manager = ConnectionManager::<diesel::PgConnection>::new(&url);
        let pool = Pool::builder().build(manager).unwrap();
        let indicator = DieselHealthIndicator::new(pool);

        run_test_up(indicator).await;
    }

    #[cfg(feature = "diesel-postgres")]
    #[tokio::test]
    async fn test_diesel_down() {
        let container = Postgres::default().start().await.unwrap();
        container.start().await.unwrap();

        let url = get_url(&container).await;

        let manager = ConnectionManager::<diesel::PgConnection>::new(&url);
        let pool = Pool::builder().build(manager).unwrap();
        let indicator = DieselHealthIndicator::new(pool);

        run_test_down(container, indicator).await;
    }

    #[cfg(feature = "sqlx-postgres")]
    #[tokio::test]
    async fn test_sqlx_up() {
        let container = Postgres::default().start().await.unwrap();
        container.start().await.unwrap();

        let url = get_url(&container).await;

        let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();
        let indicator = SqlxHealthIndicator::new(pool);

        run_test_up(indicator).await;
    }

    #[cfg(feature = "sqlx-postgres")]
    #[tokio::test]
    async fn test_sqlx_down() {
        let container = Postgres::default().start().await.unwrap();
        container.start().await.unwrap();

        let url = get_url(&container).await;

        let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();
        let indicator = SqlxHealthIndicator::new(pool);

        run_test_down(container, indicator).await;
    }

    #[cfg(feature = "sea-orm-postgres")]
    #[tokio::test]
    async fn test_sea_orm_up() {
        let container = Postgres::default().start().await.unwrap();
        container.start().await.unwrap();

        let url = get_url(&container).await;

        let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();
        let database = DatabaseConnection::from(pool);
        let indicator = SeaOrmHealthIndicator::new(database);

        run_test_up(indicator).await;
    }

    #[cfg(feature = "sea-orm-postgres")]
    #[tokio::test]
    async fn test_sea_orm_down() {
        let container = Postgres::default().start().await.unwrap();
        container.start().await.unwrap();

        let url = get_url(&container).await;

        let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();
        let database = DatabaseConnection::from(pool);
        let indicator = SeaOrmHealthIndicator::new(database);

        run_test_down(container, indicator).await;
    }

    async fn get_url(container: &ContainerAsync<Postgres>) -> String {
        format!(
            "postgresql://postgres:postgres@{}:{}/postgres",
            container.get_host().await.unwrap(),
            container.get_host_port_ipv4(5432).await.unwrap()
        )
    }

    async fn run_test_up(indicator: impl HealthIndicator + Send + Sync + 'static) {
        let name = indicator.name();
        let router = Router::new()
            .route("/health", get(health))
            .layer(Health::builder().with_indicator(indicator).build());

        let server = TestServer::new(router).unwrap();

        let response = server.get("/health").await;
        let body = response.json::<HealthDetails>();

        let expected = HealthDetails {
            status: HealthStatus::Up,
            components: HashMap::from_iter([(name, HealthDetail::up())]),
        };

        assert_eq!(body, expected);
    }

    async fn run_test_down(
        container: ContainerAsync<Postgres>,
        indicator: impl HealthIndicator + Send + Sync + 'static,
    ) {
        let name = indicator.name();
        let router = Router::new()
            .route("/health", get(health))
            .layer(Health::builder().with_indicator(indicator).build());

        let server = TestServer::new(router).unwrap();

        container.stop().await.unwrap();

        let response = server.get("/health").await;
        let body = response.json::<HealthDetails>();

        let expected = HealthDetails {
            status: HealthStatus::Down,
            components: HashMap::from_iter([(name, HealthDetail::down())]),
        };

        assert_eq!(body, expected);
    }
}
