use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, ServiceExt};
use axum_health::sqlx::SqlxHealthIndicator;
use axum_health::Health;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("test.db").await.unwrap();

    // Clone the pool!
    let indicator = SqlxHealthIndicator::new(pool.clone());

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

async fn things(State(pool): State<SqlitePool>) -> impl IntoResponse {
    // Do whatever
    StatusCode::OK
}
