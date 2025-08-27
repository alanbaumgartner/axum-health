use {
    crate::database::Pingable,
    async_trait::async_trait,
    redis::{Client, Commands},
};

#[async_trait]
impl Pingable for Client {
    async fn ping(&self) -> bool {
        Commands::ping::<String>(&mut self.clone()).is_ok()
    }
}
