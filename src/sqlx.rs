use crate::service::{HealthDetail, HealthIndicator};
use crate::validation::ValidationQuery;
use async_trait::async_trait;
use sqlx::pool::Pool;
use sqlx::{Acquire, Database, Executor};

/// [HealthIndicator] for [sqlx] connection pools
pub struct SqlxHealthIndicator<DB>
where
    DB: Database + ValidationQuery,
    for<'a> &'a mut <DB as Database>::Connection: Executor<'a>,
{
    pub(crate) name: String,
    pub(crate) pool: Pool<DB>,
}

impl<DB> SqlxHealthIndicator<DB>
where
    DB: Database + ValidationQuery,
    for<'a> &'a mut <DB as Database>::Connection: Executor<'a>,
{
    /// Creates a new [SqlxHealthIndicator] with the default name `sqlx-*` depending on the sqlx backend used
    pub fn new(pool: Pool<DB>) -> Self {
        let name = DB::name().to_owned();
        Self { name, pool }
    }

    /// Creates a new [SqlxHealthIndicator] with the given name
    pub fn new_named(name: String, pool: Pool<DB>) -> Self {
        Self { name, pool }
    }
}

#[async_trait]
impl<DB> HealthIndicator for SqlxHealthIndicator<DB>
where
    DB: Database + ValidationQuery,
    for<'a> &'a mut <DB as Database>::Connection: Executor<'a>,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    async fn details(&self) -> HealthDetail {
        let query = DB::validation_query();

        let Ok(mut pool_connection) = self.pool.acquire().await else {
            return HealthDetail::down();
        };

        let Ok(conn) = pool_connection.acquire().await else {
            return HealthDetail::down();
        };

        let result = sqlx::raw_sql(query).execute(conn).await.is_ok();
        if result {
            HealthDetail::up()
        } else {
            HealthDetail::down()
        }
    }
}
