use {
    crate::{
        database::util::{get_mysql_container, get_postgres_container, health_details},
        test_indicator,
    },
    axum_health::prelude::*,
    sqlx::{MySql, Pool, Postgres, Sqlite},
};

#[tokio::test]
async fn mysql() {
    let (url, _container) = get_mysql_container().await;
    let pool = Pool::<MySql>::connect(&url).await.unwrap();
    let expected = health_details("mysql", HealthStatus::Up);
    let result = test_indicator(pool).await;
    assert_eq!(result, expected);
}

#[tokio::test]
async fn postgres() {
    let (url, _container) = get_postgres_container().await;
    let pool = Pool::<Postgres>::connect(&url).await.unwrap();
    let expected = health_details("postgres", HealthStatus::Up);
    let result = test_indicator(pool).await;
    assert_eq!(result, expected);
}

#[tokio::test]
async fn sqlite() {
    let pool = Pool::<Sqlite>::connect(":memory:").await.unwrap();
    let expected = health_details("sqlite", HealthStatus::Up);
    let result = test_indicator(pool).await;
    assert_eq!(result, expected);
}
