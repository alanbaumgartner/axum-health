use async_trait::async_trait;
use axum::middleware::AddExtension;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tower_layer::Layer;

#[derive(Clone)]
pub struct Health(Arc<HashMap<String, Arc<dyn HealthIndicator + Send + Sync + 'static>>>);

impl Health {
    pub fn builder() -> HealthBuilder {
        HealthBuilder::default()
    }

    pub async fn details(&self) -> HealthDetails {
        let statuses = futures::stream::iter(self.0.values())
            .then(|indicator| async move { (indicator.name(), indicator.details().await) })
            .collect::<HashMap<_, _>>()
            .await;

        // let statuses = self
        //     .0
        //     .values()
        //     .map(|indicator| (indicator.name(), indicator.details()))
        //     .collect::<HashMap<_, _>>();

        let worst = statuses
            .values()
            .map(|detail| &detail.status)
            .min()
            .cloned()
            .unwrap_or(HealthStatus::Up);

        HealthDetails {
            status: worst,
            components: statuses,
        }
    }
}

impl<S> Layer<S> for Health {
    type Service = AddExtension<S, Health>;

    fn layer(&self, inner: S) -> Self::Service {
        Extension(self.clone()).layer(inner)
    }
}

#[derive(Default)]
pub struct HealthBuilder(HashMap<String, Arc<dyn HealthIndicator + Send + Sync + 'static>>);

impl HealthBuilder {
    pub fn with_indicator<I>(mut self, indicator: I) -> Self
    where
        I: HealthIndicator + Send + Sync + 'static,
    {
        self.0.insert(indicator.name(), Arc::new(indicator));
        self
    }

    pub fn build(self) -> Health {
        Health(Arc::new(self.0))
    }
}

#[async_trait]
pub trait HealthIndicator {
    fn name(&self) -> String;
    async fn details(&self) -> HealthDetail;
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum HealthStatus {
    Up,
    Down,
    OutOfService,
    Unknown,
    Custom(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HealthDetails {
    pub status: HealthStatus,
    pub components: HashMap<String, HealthDetail>,
}

impl IntoResponse for HealthDetails {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HealthDetail {
    pub status: HealthStatus,
    pub details: HashMap<String, String>,
}

impl HealthDetail {
    pub fn new(status: HealthStatus) -> Self {
        HealthDetail {
            status,
            details: Default::default(),
        }
    }

    pub fn up() -> Self {
        HealthDetail::new(HealthStatus::Up)
    }

    pub fn down() -> Self {
        HealthDetail::new(HealthStatus::Down)
    }

    pub fn with_detail(&mut self, name: String, detail: String) -> &mut Self {
        self.details.insert(name, detail);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::health;
    use crate::service::{Health, HealthDetail, HealthDetails, HealthIndicator, HealthStatus};
    use async_trait::async_trait;
    use axum::routing::get;
    use axum::Router;
    use axum_test::TestServer;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_health() {
        let router = Router::new()
            .route("/health", get(health))
            .layer(Health::builder().build());

        let server = TestServer::new(router).unwrap();
        let response = server.get("/health").await;

        let body = response.json::<HealthDetails>();
        let expected = HealthDetails {
            status: HealthStatus::Up,
            components: Default::default(),
        };
        assert_eq!(body, expected);
    }

    #[tokio::test]
    async fn test_custom_health_indicator() {
        struct CustomHealthIndicator {
            pub value: &'static str,
        }

        #[async_trait]
        impl HealthIndicator for CustomHealthIndicator {
            fn name(&self) -> String {
                self.value.to_owned()
            }

            async fn details(&self) -> HealthDetail {
                HealthDetail::up()
            }
        }

        let router = Router::new().route("/health", get(health)).layer(
            Health::builder()
                .with_indicator(CustomHealthIndicator { value: "custom" })
                .build(),
        );

        let server = TestServer::new(router).unwrap();
        let response = server.get("/health").await;

        let body = response.json::<HealthDetails>();

        let expected = HealthDetails {
            status: HealthStatus::Up,
            components: HashMap::from_iter([("custom".to_owned(), HealthDetail::up())]),
        };

        assert_eq!(body, expected);
    }
}
