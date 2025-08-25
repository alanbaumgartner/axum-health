// use {
//     axum::{http::StatusCode, routing::get, Router},
//     axum_health::{service::HealthIndicator, Health, HealthDetail, HealthDetails, HealthStatus},
//     axum_test::TestServer,
//     std::collections::BTreeMap,
//     testcontainers::{runners::AsyncRunner, ContainerAsync},
//     testcontainers_modules::postgres::Postgres,
// };
//
// #[cfg(feature = "diesel-r2d2")]
// mod diesel_r2d2 {
//     use {
//         axum_health::{database::DatabaseHealthIndicator, HealthIndicator},
//         diesel::r2d2::ConnectionManager,
//         std::time::Duration,
//     };
//
//     async fn postgres(url: &str) -> impl HealthIndicator {
//         let manager = ConnectionManager::<diesel::PgConnection>::new(url.to_owned());
//         let pool = diesel::r2d2::Pool::builder()
//             .max_size(1)
//             .connection_timeout(Duration::from_secs(5))
//             .build(manager)
//             .unwrap();
//         DatabaseHealthIndicator(pool)
//     }
// }
//
// #[cfg(feature = "diesel-bb8")]
// mod diesel_bb8 {
//     use {
//         axum_health::{database::DatabaseHealthIndicator, HealthIndicator},
//         diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection},
//         std::time::Duration,
//     };
//
//     async fn async_diesel_bb8(url: &str) -> impl HealthIndicator {
//         let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
//         let pool = diesel_async::pooled_connection::bb8::Pool::builder()
//             .max_size(1)
//             .connection_timeout(Duration::from_secs(5))
//             .build(manager)
//             .await
//             .unwrap();
//         DatabaseHealthIndicator(pool)
//     }
// }
//
// #[cfg(feature = "diesel-deadpool")]
// mod diesel_deadpool {
//     use {
//         axum_health::{database::DatabaseHealthIndicator, HealthIndicator},
//         diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection},
//     };
//
//     async fn async_diesel_deadpool(url: &str) -> impl HealthIndicator {
//         let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
//         let pool = diesel_async::pooled_connection::deadpool::Pool::builder(manager)
//             .max_size(1)
//             .build()
//             .unwrap();
//         DatabaseHealthIndicator(pool)
//     }
// }
//
// #[cfg(feature = "diesel-mobc")]
// mod diesel_mobc {
//     use {
//         axum_health::{database::DatabaseHealthIndicator, HealthIndicator},
//         diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection},
//     };
//
//     async fn async_diesel_mobc(url: &str) -> impl HealthIndicator {
//         let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.to_owned());
//         let pool = diesel_async::pooled_connection::mobc::Pool::builder()
//             .max_open(1)
//             .build(manager);
//         DatabaseHealthIndicator(pool)
//     }
// }
//
// #[cfg(feature = "sqlx")]
// mod sqlx {
//     use {
//         axum_health::{database::DatabaseHealthIndicator, HealthIndicator},
//         std::time::Duration,
//     };
//
//     async fn sqlx(url: &str) -> impl HealthIndicator {
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .max_connections(1)
//             .acquire_timeout(Duration::from_secs(5))
//             .connect(&url)
//             .await
//             .unwrap();
//         DatabaseHealthIndicator(pool)
//     }
// }
//
// #[cfg(feature = "sea-orm")]
// mod sea_orm {
//     use {
//         axum_health::{database::DatabaseHealthIndicator, HealthIndicator},
//         std::time::Duration,
//     };
//     use sea_orm::DatabaseConnection;
//
//     async fn sea_orm(url: &str) -> impl HealthIndicator {
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .max_connections(1)
//             .acquire_timeout(Duration::from_secs(5))
//             .connect(&url)
//             .await
//             .unwrap();
//         let pool = DatabaseConnection::from(pool);
//         DatabaseHealthIndicator(pool)
//     }
// }
//
// #[tokio::test]
// async fn test_all() {
//     let container = Postgres::default().start().await.unwrap();
//     container.start().await.unwrap();
//
//     let url = get_url(&container).await;
//     let url = url.as_str();
//
//     let health = Health::builder()
//         .with_indicator(diesel(url).await)
//         .with_indicator(async_diesel_bb8(url).await)
//         .with_indicator(async_diesel_deadpool(url).await)
//         .with_indicator(async_diesel_mobc(url).await)
//         .with_indicator(sqlx(url).await)
//         .with_indicator(sea_orm(url).await)
//         .build();
//
//     let router = Router::new()
//         .route("/health", get(axum_health::health))
//         .layer(health);
//
//     let server = TestServer::new(router).unwrap();
//
//     let response = server.get("/health").await;
//     assert_eq!(response.status_code(), StatusCode::OK);
//     let body = response.json::<HealthDetails>();
//
//     let expected = HealthDetails {
//         status: HealthStatus::Up,
//         components: BTreeMap::from_iter([
//             ("diesel-postgres".to_owned(), HealthDetail::up()),
//             ("diesel-bb8".to_owned(), HealthDetail::up()),
//             ("diesel-deadpool".to_owned(), HealthDetail::up()),
//             ("diesel-mobc".to_owned(), HealthDetail::up()),
//             ("sqlx".to_owned(), HealthDetail::up()),
//             ("sea-orm".to_owned(), HealthDetail::up()),
//         ]),
//     };
//     assert_eq!(body, expected);
//
//     container.stop().await.unwrap();
//
//     let response = server.get("/health").await;
//     assert_eq!(response.status_code(), StatusCode::SERVICE_UNAVAILABLE);
//     let body = response.json::<HealthDetails>();
//
//     let expected = HealthDetails {
//         status: HealthStatus::Down,
//         components: BTreeMap::from_iter([
//             ("diesel-postgres".to_owned(), HealthDetail::down()),
//             ("diesel-bb8".to_owned(), HealthDetail::down()),
//             ("diesel-deadpool".to_owned(), HealthDetail::down()),
//             ("diesel-mobc".to_owned(), HealthDetail::down()),
//             ("sqlx".to_owned(), HealthDetail::down()),
//             ("sea-orm".to_owned(), HealthDetail::down()),
//         ]),
//     };
//     assert_eq!(body, expected);
// }
//
// async fn get_url(container: &ContainerAsync<Postgres>) -> String {
//     format!(
//         "postgresql://postgres:postgres@{}:{}/postgres",
//         container.get_host().await.unwrap(),
//         container.get_host_port_ipv4(5432).await.unwrap()
//     )
// }
