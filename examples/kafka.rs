use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum_health::kafka::KafkaHealthIndicator;
use axum_health::Health;
use diesel::r2d2::{ConnectionManager, Pool};
use rdkafka::consumer::StreamConsumer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Clone the pool!
    let indicator = KafkaHealthIndicator {
        consumer: StreamConsumer
    };

    let router = Router::new()
        .route("/health", get(axum_health::health))
        .route("/things", get(things))
        // Create a Health layer and add the indicator
        .layer(Health::builder().with_indicator(indicator).build())
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}

async fn things(
    State(_pool): State<Pool<ConnectionManager<diesel::SqliteConnection>>>,
) -> impl IntoResponse {
    // Do whatever
    StatusCode::OK
}
