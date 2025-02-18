use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum_health::sea_orm::SeaOrmHealthIndicator;
use axum_health::Health;
use sea_orm::DatabaseConnection;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("test.db").await.unwrap();
    let database_connection = DatabaseConnection::from(pool);

    // Clone the pool!
    let indicator = SeaOrmHealthIndicator::new(database_connection.clone());

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

async fn things(State(pool): State<DatabaseConnection>) -> impl IntoResponse {
    // Do whatever
    StatusCode::OK
}
