use crate::{HealthDetail, HealthIndicator};
use async_trait::async_trait;

#[cfg(feature = "diesel")]
pub mod diesel;
#[cfg(feature = "sea-orm")]
pub mod sea_orm;
#[cfg(feature = "sqlx")]
pub mod sqlx;

#[async_trait]
pub trait Pingable {
    async fn ping(&self) -> bool;
}

pub struct DatabaseHealthIndicator<Pool>
where
    Pool: Pingable,
{
    name: String,
    pool: Pool,
}

impl<Pool> DatabaseHealthIndicator<Pool>
where
    Pool: Pingable,
{
    pub fn new(name: String, pool: Pool) -> Self {
        DatabaseHealthIndicator { name, pool }
    }
}

#[async_trait]
impl<Pool> HealthIndicator for DatabaseHealthIndicator<Pool>
where
    Pool: Pingable + Send + Sync + 'static,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    async fn details(&self) -> HealthDetail {
        if self.pool.ping().await {
            HealthDetail::up()
        } else {
            HealthDetail::down()
        }
    }
}
