use {
    crate::{
        database::util::{get_mysql_container, get_postgres_container},
        test_indicator,
    },
    axum_health::prelude::*,
    sea_orm::{Database, DatabaseConnection},
    std::collections::BTreeMap,
};

#[tokio::test]
async fn mysql() {
    let (url, _container) = get_mysql_container().await;
    let db: DatabaseConnection = Database::connect(&url).await.unwrap();
    let indicator = DatabaseHealthIndicator(db);
    let expected = HealthDetails {
        status: HealthStatus::Up,
        components: BTreeMap::from_iter([("database".to_owned(), HealthDetail::up())]),
    };
    let result = test_indicator(indicator).await;
    assert_eq!(result, expected);
}

#[tokio::test]
async fn postgres() {
    let (url, _container) = get_postgres_container().await;
    let db: DatabaseConnection = Database::connect(&url).await.unwrap();
    let indicator = DatabaseHealthIndicator(db);
    let expected = HealthDetails {
        status: HealthStatus::Up,
        components: BTreeMap::from_iter([("database".to_owned(), HealthDetail::up())]),
    };
    let result = test_indicator(indicator).await;
    assert_eq!(result, expected);
}

#[tokio::test]
async fn sqlite() {
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await.unwrap();
    let indicator = DatabaseHealthIndicator(db);
    let expected = HealthDetails {
        status: HealthStatus::Up,
        components: BTreeMap::from_iter([("database".to_owned(), HealthDetail::up())]),
    };
    let result = test_indicator(indicator).await;
    assert_eq!(result, expected);
}
