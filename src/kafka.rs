// use {
//     crate::{HealthDetail, HealthIndicator},
//     async_trait::async_trait,
//     rdkafka::{
//         client::Client,
//         consumer::{BaseConsumer, Consumer, ConsumerContext},
//         producer::{BaseProducer, Producer, ProducerContext},
//         util::Timeout,
//         ClientContext,
//     },
//     std::{sync::Arc, time::Duration},
// };
//
// const TIMEOUT: Duration = Duration::from_secs(1);
//
// #[async_trait]
// pub trait KafkaHealthSource {
//     async fn details(&self) -> HealthDetail;
// }
//
// trait HasClient<C: ClientContext> {
//     fn get_client(&self) -> &Client<C>;
// }
//
// impl<C: ConsumerContext> HasClient<C> for BaseConsumer<C> {
//     fn get_client(&self) -> &Client<C> {
//         self.client()
//     }
// }
//
// impl<C: ProducerContext> HasClient<C> for BaseProducer<C> {
//     fn get_client(&self) -> &Client<C> {
//         self.client()
//     }
// }
//
// #[async_trait]
// impl<T> KafkaHealthSource for T
// where
//     T: HasClient<C: ClientContext> + Send + Sync,
// {
//     async fn details(&self) -> HealthDetail {
//         if let Some((code, message)) = self.get_client().fatal_error() {
//             return HealthDetail::down()
//                 .with_detail("error_code", code)
//                 .with_detail("error_message", message);
//         }
//
//         match self.get_client().fetch_cluster_id(Timeout::from(TIMEOUT)) {
//             Some(id) => HealthDetail::up().with_detail("cluster_id", id),
//             None => HealthDetail::down(),
//         }
//     }
// }
//
// pub struct KafkaHealthIndicator {
//     source: Arc<dyn KafkaHealthSource + Send + Sync>,
// }
//
// impl KafkaHealthIndicator {
//     pub fn new(source: impl KafkaHealthSource + Send + Sync + 'static) -> Self {
//         Self {
//             source: Arc::new(source),
//         }
//     }
// }
//
// #[async_trait]
// impl HealthIndicator for KafkaHealthIndicator {
//     fn name(&self) -> String {
//         "kafka".to_owned()
//     }
//
//     async fn details(&self) -> HealthDetail {
//         self.source.details().await
//     }
// }
