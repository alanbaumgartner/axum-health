use {
    axum::{http::StatusCode, routing::get, Router},
    axum_health::{
        database::DatabaseHealthIndicator,
        health,
        service::{Health, HealthDetail, HealthDetails, HealthIndicator, HealthStatus},
    },
    axum_test::TestServer,
    std::{collections::BTreeMap, fs::OpenOptions},
};

#[cfg(feature = "diesel-r2d2")]
#[tokio::test]
async fn test_diesel() {
    let url = get_sqlite_path();
    let manager = diesel::r2d2::ConnectionManager::<diesel::SqliteConnection>::new(url);
    let pool = diesel::r2d2::Pool::builder().build(manager).unwrap();
    let indicator = DatabaseHealthIndicator(pool);

    run_test(indicator).await;
}

#[cfg(feature = "sqlx")]
#[tokio::test]
async fn test_sqlx() {
    let url = get_sqlite_path();
    let pool = sqlx::sqlite::SqlitePool::connect(url.as_str())
        .await
        .unwrap();
    let indicator = DatabaseHealthIndicator(pool);

    run_test(indicator).await;
}

#[cfg(feature = "sea-orm")]
#[tokio::test]
async fn test_sea_orm() {
    let url = get_sqlite_path();
    let pool = sqlx::sqlite::SqlitePool::connect(url.as_str())
        .await
        .unwrap();
    let database = sea_orm::DatabaseConnection::from(pool);
    let indicator = DatabaseHealthIndicator(database);

    run_test(indicator).await;
}

fn get_sqlite_path() -> String {
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
    url.to_owned()
}

pub async fn run_test(indicator: impl HealthIndicator + Send + Sync + 'static) {
    let router = Router::new()
        .route("/health", get(health))
        .layer(Health::builder().with_indicator(indicator).build());

    let server = TestServer::new(router).unwrap();

    let response = server.get("/health").await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let body = response.json::<HealthDetails>();

    let expected = HealthDetails {
        status: HealthStatus::Up,
        components: BTreeMap::from_iter([("database".to_string(), HealthDetail::up())]),
    };

    assert_eq!(body, expected);
}
