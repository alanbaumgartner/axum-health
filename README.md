# axum-health

<div>
<a href="https://github.com/alanbaumgartner/axum-health/actions/workflows/rust.yml"><img src="https://github.com/alanbaumgartner/axum-health/actions/workflows/rust.yml/badge.svg" /></a>
<a href="https://crates.io/crates/axum-health"><img src="https://img.shields.io/crates/v/axum-health.svg" /></a>
<a href="https://docs.rs/axum-health"><img src="https://docs.rs/axum-health/badge.svg" /></a>
</div>

[Spring Boot](https://spring.io/projects/spring-boot)
-like [health indicators](https://docs.spring.io/spring-boot/api/rest/actuator/health.html).

## Usage

```rust
#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("test.db").await.unwrap();

    // Clone the pool!
    let indicator = DatabaseHealthIndicator::new("sqlite".to_owned(), pool.clone());

    let router = Router::new()
        .route("/health", get(axum_health::health))
        // Create a Health layer and add the indicator
        .layer(Health::builder().with_indicator(indicator).build())
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap()
}

```

The health endpoint will respond

```json
{
  "status": "UP",
  "components": {
    "sqlite": {
      "status": "UP"
    }
  }
}
```

Checkout the [examples](/examples)
