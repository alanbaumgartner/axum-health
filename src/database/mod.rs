use {
    crate::{HealthDetail, HealthIndicator},
    async_trait::async_trait,
};

#[cfg(feature = "diesel")]
pub mod diesel;
#[cfg(feature = "sea-orm")]
pub mod sea_orm;
#[cfg(feature = "sqlx")]
pub mod sqlx;

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
        "database".to_string()
    }

    async fn details(&self) -> HealthDetail {
        if self.0.ping().await {
            HealthDetail::up()
        } else {
            HealthDetail::down()
        }
    }
}
