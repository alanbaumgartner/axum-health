use {
    crate::service::{HealthDetail, HealthIndicator},
    async_trait::async_trait,
};

#[cfg(feature = "cassandra")]
mod cassandra;
#[cfg(feature = "diesel")]
mod diesel;
#[cfg(feature = "elasticsearch")]
mod elasticsearch;
#[cfg(feature = "mongo")]
mod mongo;
#[cfg(feature = "neo4j")]
mod neo4j;
#[cfg(feature = "redis")]
mod redis;
#[cfg(feature = "sea-orm")]
mod sea_orm;
#[cfg(feature = "sqlx")]
mod sqlx;

#[cfg(feature = "cassandra")]
pub use cassandra::*;
#[allow(unused_imports)]
#[cfg(feature = "diesel")]
pub use diesel::*;
#[cfg(feature = "elasticsearch")]
pub use elasticsearch::*;
#[cfg(feature = "mongo")]
pub use mongo::*;
#[cfg(feature = "neo4j")]
pub use neo4j::*;
#[cfg(feature = "redis")]
pub use redis::*;
#[allow(unused_imports)]
#[cfg(feature = "sea-orm")]
pub use sea_orm::*;
#[allow(unused_imports)]
#[cfg(feature = "sqlx")]
pub use sqlx::*;

/// [DatabaseHealthIndicator] can be used with anything that implements this trait.
/// [diesel], [sea-orm], and [sqlx] all implement some form of a `ping` operation on their connection
/// or connection pools, but this can be implemented for other database drivers using a manual query,
/// generally a `SELECT 1` query or variant.
#[async_trait]
pub trait Pingable {
    async fn ping(&self) -> bool;
}

pub struct DatabaseHealthIndicator<Pool>(pub Pool)
where
    Pool: Pingable;

#[async_trait]
impl<Pool> HealthIndicator for DatabaseHealthIndicator<Pool>
where
    Pool: Pingable + Send + Sync + 'static,
{
    fn name(&self) -> String {
        "database".to_owned()
    }

    async fn details(&self) -> HealthDetail {
        if self.0.ping().await {
            HealthDetail::up()
        } else {
            HealthDetail::down()
        }
    }
}
