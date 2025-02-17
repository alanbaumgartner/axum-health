use crate::service::{HealthDetail, HealthIndicator};
use crate::validation::ValidationQuery;
use async_trait::async_trait;
use diesel::backend::DieselReserveSpecialization;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{sql_query, RunQueryDsl};

/// [HealthIndicator] for [diesel] connection pools
pub struct DieselHealthIndicator<Conn>
where
    Conn: diesel::r2d2::R2D2Connection + Send + 'static,
    Conn::Backend: ValidationQuery + DieselReserveSpecialization,
{
    pub(crate) name: String,
    pub(crate) pool: Pool<ConnectionManager<Conn>>,
}

impl<Conn> DieselHealthIndicator<Conn>
where
    Conn: diesel::r2d2::R2D2Connection + Send + 'static,
    Conn::Backend: ValidationQuery + DieselReserveSpecialization,
{
    /// Creates a new [DieselHealthIndicator] with the default name `diesel-*` depending on the diesel backend used
    pub fn new(pool: Pool<ConnectionManager<Conn>>) -> Self {
        let name = Conn::Backend::name().to_string();
        Self { name, pool }
    }

    /// Creates a new [DieselHealthIndicator] with the given name
    pub fn new_named(name: String, pool: Pool<ConnectionManager<Conn>>) -> Self {
        Self { name, pool }
    }
}

#[async_trait]
impl<Conn> HealthIndicator for DieselHealthIndicator<Conn>
where
    Conn: diesel::r2d2::R2D2Connection + Send + 'static,
    Conn::Backend: ValidationQuery + DieselReserveSpecialization,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    async fn details(&self) -> HealthDetail {
        let query = Conn::Backend::validation_query();

        let ok = self
            .pool
            .get()
            .map(|mut conn| sql_query(query).execute(&mut conn))
            .is_ok();

        if ok {
            HealthDetail::up()
        } else {
            HealthDetail::down()
        }
    }
}
