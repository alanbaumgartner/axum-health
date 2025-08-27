use {
    axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router},
    axum_health::prelude::*,
    sea_orm::DatabaseConnection,
    sqlx::SqlitePool,
    tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    let database_connection = DatabaseConnection::from(pool);

    // Clone the pool!
    let indicator = DatabaseHealthIndicator(database_connection.clone());

    let router = Router::new()
        .route("/health", get(axum_health::health))
        .route("/things", get(things))
        // Create a Health layer and add the indicator
        .layer(Health::builder().with_indicator(indicator).build())
        .with_state(database_connection);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}

async fn things(State(_pool): State<DatabaseConnection>) -> impl IntoResponse {
    // Do whatever
    StatusCode::OK
}
