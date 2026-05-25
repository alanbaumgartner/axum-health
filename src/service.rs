use {
    crate::{indicator::*, indicators::PingHealthIndicator},
    axum::{middleware::AddExtension, Extension},
    futures::StreamExt,
    std::{collections::BTreeMap, sync::Arc},
    tower_layer::Layer,
};

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
            .copied()
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
    pub fn with_ping(self) -> Self {
        self.with_indicator(PingHealthIndicator)
    }

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

#[cfg(test)]
mod test {
    use {
        crate::{
            health_check,
            service::{Health, HealthDetail, HealthDetails, HealthIndicator, HealthStatus},
        },
        async_trait::async_trait,
        axum::{http::StatusCode, routing::get, Router},
        axum_test::TestServer,
        std::collections::BTreeMap,
    };

    pub struct MockHealthIndicator {
        name: String,
        response: HealthDetail,
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
            .route("/health", get(health_check))
            .layer(Health::builder().build());

        let server = TestServer::new(router);
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
        let router = Router::new().route("/health", get(health_check)).layer(
            Health::builder()
                .with_indicator(MockHealthIndicator {
                    name: "custom".to_owned(),
                    response: HealthDetail::up(),
                })
                .build(),
        );

        let server = TestServer::new(router);
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
        let router = Router::new().route("/health", get(health_check)).layer(
            Health::builder()
                .with_indicator(MockHealthIndicator {
                    name: "upper".to_owned(),
                    response: HealthDetail::up(),
                })
                .with_indicator(MockHealthIndicator {
                    name: "downer".to_owned(),
                    response: HealthDetail::down(),
                })
                .build(),
        );

        let server = TestServer::new(router);
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
