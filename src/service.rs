use async_trait::async_trait;
use axum::http::StatusCode;
use axum::middleware::AddExtension;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tower_layer::Layer;

#[derive(Clone)]
pub struct Health(Arc<BTreeMap<String, Arc<dyn HealthIndicator + Send + Sync + 'static>>>);

impl Health {
    pub fn builder() -> HealthBuilder {
        HealthBuilder::default()
    }

    pub async fn details(&self) -> HealthDetails {
        let statuses = futures::stream::iter(self.0.values())
            .then(|indicator| async move { (indicator.name(), indicator.details().await) })
            .collect::<BTreeMap<_, _>>()
            .await;

        // If we have no health indicators, we are up, otherwise we take our worst one and respond with that.
        let worst_status = statuses
            .values()
            .map(|detail| &detail.status)
            .max()
            .cloned()
            .unwrap_or(HealthStatus::Up);

        HealthDetails {
            status: worst_status,
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
pub struct HealthBuilder(BTreeMap<String, Arc<dyn HealthIndicator + Send + Sync + 'static>>);

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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub components: BTreeMap<String, HealthDetail>,
}

impl IntoResponse for HealthDetails {
    fn into_response(self) -> Response {
        let status_code = match &self.status {
            HealthStatus::Down | HealthStatus::OutOfService => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::OK,
        };
        (status_code, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HealthDetail {
    pub status: HealthStatus,
    pub details: BTreeMap<String, String>,
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
    use axum::http::StatusCode;
    use axum::routing::get;
    use axum::Router;
    use axum_test::TestServer;
    use std::collections::BTreeMap;

    pub struct MockHealthIndicator {
        name: String,
        response: HealthDetail,
    }

    impl MockHealthIndicator {
        pub fn new(name: String, response: HealthDetail) -> Self {
            MockHealthIndicator { name, response }
        }
    }

    #[async_trait]
    impl HealthIndicator for MockHealthIndicator {
        fn name(&self) -> String {
            self.name.to_owned()
        }

        async fn details(&self) -> HealthDetail {
            self.response.clone()
        }
    }

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
        let router = Router::new().route("/health", get(health)).layer(
            Health::builder()
                .with_indicator(MockHealthIndicator::new(
                    "custom".to_string(),
                    HealthDetail::up(),
                ))
                .build(),
        );

        let server = TestServer::new(router).unwrap();
        let response = server.get("/health").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let body = response.json::<HealthDetails>();

        let expected = HealthDetails {
            status: HealthStatus::Up,
            components: BTreeMap::from_iter([("custom".to_owned(), HealthDetail::up())]),
        };

        assert_eq!(body, expected);
    }

    #[tokio::test]
    async fn test_status_down() {
        let router = Router::new().route("/health", get(health)).layer(
            Health::builder()
                .with_indicator(MockHealthIndicator::new(
                    "upper".to_string(),
                    HealthDetail::up(),
                ))
                .with_indicator(MockHealthIndicator::new(
                    "downer".to_string(),
                    HealthDetail::down(),
                ))
                .build(),
        );

        let server = TestServer::new(router).unwrap();
        let response = server.get("/health").await;

        assert_eq!(response.status_code(), StatusCode::SERVICE_UNAVAILABLE);

        let body = response.json::<HealthDetails>();

        let expected = HealthDetails {
            status: HealthStatus::Down,
            components: BTreeMap::from_iter([
                ("upper".to_owned(), HealthDetail::up()),
                ("downer".to_owned(), HealthDetail::down()),
            ]),
        };

        assert_eq!(body, expected);
    }
}
