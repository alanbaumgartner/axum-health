use {
    crate::{indicator::HealthDetail, prelude::HealthIndicator},
    async_trait::async_trait,
    redis::{Client, Commands},
};

#[async_trait]
impl HealthIndicator for Client {
    fn name(&self) -> String {
        String::from("redis")
    }

    async fn details(&self) -> HealthDetail {
        match Commands::ping::<String>(&mut self.clone()) {
            Ok(_ok) => HealthDetail::up(),
            Err(_) => HealthDetail::down(),
        }
    }
}
