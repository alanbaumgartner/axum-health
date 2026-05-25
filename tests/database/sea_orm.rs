use {
    crate::{
        database::util::{get_mysql_container, get_postgres_container, health_details},
        test_indicator,
    },
    axum_health::prelude::*,
    sea_orm::{Database, DatabaseConnection},
};

#[tokio::test]
async fn mysql() {
    let (url, _container) = get_mysql_container().await;
    let db: DatabaseConnection = Database::connect(&url).await.unwrap();
    let expected = health_details("mysql", HealthStatus::Up);
    let result = test_indicator(db).await;
    assert_eq!(result, expected);
}

#[tokio::test]
async fn postgres() {
    let (url, _container) = get_postgres_container().await;
    let db: DatabaseConnection = Database::connect(&url).await.unwrap();
    let expected = health_details("postgres", HealthStatus::Up);
    let result = test_indicator(db).await;
    assert_eq!(result, expected);
}

#[tokio::test]
async fn sqlite() {
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await.unwrap();
    let expected = health_details("sqlite", HealthStatus::Up);
    let result = test_indicator(db).await;
    assert_eq!(result, expected);
}
