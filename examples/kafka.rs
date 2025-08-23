fn main() {}

// use {
//     axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router},
//     axum_health::{kafka::KafkaHealthIndicator, Health},
//     diesel::r2d2::{ConnectionManager, Pool},
//     rdkafka::{
//         config::FromClientConfig,
//         consumer::{DefaultConsumerContext, StreamConsumer},
//         ClientConfig,
//     },
//     std::sync::Arc,
//     tokio::net::TcpListener,
// };
//
// #[tokio::main]
// async fn main() {
//     let consumer =
//         StreamConsumer::<DefaultConsumerContext>::from_config(&ClientConfig::new()).unwrap();
//
//     let consumer = Arc::new(consumer);
//     // Clone the pool!
//     let indicator = KafkaHealthIndicator::new(consumer.clone());
//
//     let router = Router::new()
//         .route("/health", get(axum_health::health))
//         .route("/things", get(things))
//         // Create a Health layer and add the indicator
//         .layer(Health::builder().with_indicator(indicator).build())
//         .with_state(pool);
//
//     let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
//
//     axum::serve(listener, router.into_make_service())
//         .await
//         .unwrap()
// }
//
// async fn things(
//     State(_pool): State<Pool<ConnectionManager<diesel::SqliteConnection>>>,
// ) -> impl IntoResponse {
//     // Do whatever
//     StatusCode::OK
// }
