# axum-health

<div>
<a href="https://github.com/alanbaumgartner/axum-health/actions/workflows/rust.yml"><img src="https://github.com/alanbaumgartner/axum-health/actions/workflows/rust.yml/badge.svg" /></a>
<a href="https://crates.io/crates/axum-health"><img src="https://img.shields.io/crates/v/axum-health.svg" /></a>
<a href="https://docs.rs/axum-health"><img src="https://docs.rs/axum-health/badge.svg" /></a>
</div>

[Spring Boot](https://spring.io/projects/spring-boot)-like [health indicators](https://docs.spring.io/spring-boot/api/rest/actuator/health.html).

## Usage

```rust
#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("test.db").await.unwrap();

    // Clone the pool!
    let indicator = axum_health::sqlx::SqlxHealthIndicator::new(pool.clone());
    
    // Create a Health layer and add the indicator
    // These can be chained
    let health_layer = Health::builder()
        .with_indicator(indicator)
        // .with_indicator(other_indicator)
        .build();

    let router = Router::new()
        .route("/health", get(axum_health::health))
        // Don't forget to add it
        .layer(health_layer)
        .with_state(pool);
    
    ...
}
```

The health endpoint will respond

```json
{
  "status": "UP",
  "components": {
    "sqlx-sqlite": {
      "status": "UP"
    }
  }
}
```
