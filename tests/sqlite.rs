#[cfg(feature = "_sqlite")]
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
    use std::fs::OpenOptions;

    #[cfg(feature = "diesel-sqlite")]
    #[tokio::test]
    async fn test_diesel() {
        let file = tempfile::tempdir().unwrap();
        let path = file.path().join("test.db");
        let url = path.to_str().unwrap();
        {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(url)
                .unwrap();
        }

        let manager = ConnectionManager::<diesel::SqliteConnection>::new(url);
        let pool = Pool::builder().build(manager).unwrap();
        let indicator = DieselHealthIndicator::new(pool);

        run_test("diesel-sqlite".to_owned(), indicator).await;
    }

    #[cfg(feature = "sqlx-sqlite")]
    #[tokio::test]
    async fn test_sqlx() {
        let file = tempfile::tempdir().unwrap();
        let path = file.path().join("test.db");
        let url = path.to_str().unwrap();
        {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(url)
                .unwrap();
        }

        let pool = sqlx::sqlite::SqlitePool::connect(url).await.unwrap();
        let indicator = SqlxHealthIndicator::new(pool);
        run_test("sqlx-sqlite".to_owned(), indicator).await;
    }

    #[cfg(feature = "sea-orm-sqlite")]
    #[tokio::test]
    async fn test_sea_orm() {
        let file = tempfile::tempdir().unwrap();
        let path = file.path().join("test.db");
        let url = path.to_str().unwrap();
        {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(url)
                .unwrap();
        }

        let pool = sqlx::sqlite::SqlitePool::connect(url).await.unwrap();
        let database = DatabaseConnection::from(pool);
        let indicator = SeaOrmHealthIndicator::new(database);

        run_test("sea-orm-sqlite".to_owned(), indicator).await;
    }

    pub async fn run_test(name: String, indicator: impl HealthIndicator + Send + Sync + 'static) {
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
}
