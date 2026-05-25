use {
    async_trait::async_trait,
    axum::{
        Json,
        http::StatusCode,
        response::{IntoResponse, Response},
    },
    serde::{Deserialize, Serialize},
    serde_json::Value,
    std::collections::BTreeMap,
};

#[async_trait]
pub trait HealthIndicator {
    fn name(&self) -> String;
    async fn details(&self) -> HealthDetail;
}

pub struct NamedHealthIndicator<T: HealthIndicator> {
    name: String,
    indicator: T,
}

#[async_trait]
impl<T: HealthIndicator + Send + Sync> HealthIndicator for NamedHealthIndicator<T> {
    fn name(&self) -> String {
        self.name.clone()
    }

    async fn details(&self) -> HealthDetail {
        self.indicator.details().await
    }
}

pub trait Named {
    fn named(self, name: String) -> NamedHealthIndicator<Self>
    where
        Self: HealthIndicator + Send + Sync + Sized;
}

impl<T: HealthIndicator + Send + Sync> Named for T {
    fn named(self, name: String) -> NamedHealthIndicator<Self> {
        NamedHealthIndicator {
            name,
            indicator: self,
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum HealthStatus {
    Up,
    Down,
    OutOfService,
    Unknown,
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
            HealthStatus::Up => StatusCode::OK,
            _ => StatusCode::SERVICE_UNAVAILABLE,
        };
        (status_code, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HealthDetail {
    pub status: HealthStatus,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub details: BTreeMap<String, Value>,
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

    pub fn with_detail(mut self, name: impl ToString, detail: impl Into<Value>) -> Self {
        self.details.insert(name.to_string(), detail.into());
        self
    }

    pub fn with_details<I, V, T>(mut self, details: T) -> Self
    where
        I: ToString,
        V: Into<Value>,
        T: IntoIterator<Item = (I, V)>,
    {
        self.details.append(
            &mut details
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect(),
        );
        self
    }
}
