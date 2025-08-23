use {
    axum::{http::StatusCode, routing::get, Router},
    axum_health::{
        database::DatabaseHealthIndicator,
        health,
        service::{Health, HealthDetail, HealthDetails, HealthIndicator, HealthStatus},
    },
    axum_test::TestServer,
    diesel::r2d2::{ConnectionManager, Pool},
    sea_orm::DatabaseConnection,
    std::{collections::BTreeMap, fs::OpenOptions},
};

#[cfg(feature = "diesel-r2d2")]
#[tokio::test]
async fn test_diesel() {
    let url = get_sqlite_path();
    let manager = ConnectionManager::<diesel::SqliteConnection>::new(url);
    let pool = Pool::builder().build(manager).unwrap();
    let indicator = DatabaseHealthIndicator::new("diesel-sqlite".to_owned(), pool);

    run_test("diesel-sqlite".to_owned(), indicator).await;
}

#[cfg(feature = "sqlx")]
#[tokio::test]
async fn test_sqlx() {
    let url = get_sqlite_path();
    let pool = sqlx::sqlite::SqlitePool::connect(url.as_str())
        .await
        .unwrap();
    let indicator = DatabaseHealthIndicator::new("sqlx-sqlite".to_owned(), pool);

    run_test("sqlx-sqlite".to_owned(), indicator).await;
}

#[cfg(feature = "sea-orm")]
#[tokio::test]
async fn test_sea_orm() {
    let url = get_sqlite_path();
    let pool = sqlx::sqlite::SqlitePool::connect(url.as_str())
        .await
        .unwrap();
    let database = DatabaseConnection::from(pool);
    let indicator = DatabaseHealthIndicator::new("sea-orm-sqlite".to_owned(), database);

    run_test("sea-orm-sqlite".to_owned(), indicator).await;
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

pub async fn run_test(name: String, indicator: impl HealthIndicator + Send + Sync + 'static) {
    let router = Router::new()
        .route("/health", get(health))
        .layer(Health::builder().with_indicator(indicator).build());

    let server = TestServer::new(router).unwrap();

    let response = server.get("/health").await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let body = response.json::<HealthDetails>();

    let expected = HealthDetails {
        status: HealthStatus::Up,
        components: BTreeMap::from_iter([(name, HealthDetail::up())]),
    };

    assert_eq!(body, expected);
}
