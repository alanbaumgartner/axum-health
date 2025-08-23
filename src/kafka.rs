use crate::{HealthDetail, HealthIndicator};
use async_trait::async_trait;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::util::Timeout;
use std::time::Duration;

enum KafkaHealthSource {
    StreamConsumer(StreamConsumer),
    // Consumer(Consumer),
    // Producer(Producer),
    // ThreadedProducer(ThreadedProducer),
    // FutureProducer(FutureProducer),
}

pub struct KafkaHealthIndicator {
    pub consumer: StreamConsumer,
}

impl KafkaHealthIndicator {
    const TIMEOUT: Duration = Duration::from_secs(1);
}

#[async_trait]
impl HealthIndicator for KafkaHealthIndicator {
    fn name(&self) -> String {
        "kafka".to_string()
    }

    async fn details(&self) -> HealthDetail {
        if let Some((code, message)) = self.consumer.client().fatal_error() {
            return HealthDetail::down()
                .with_detail("error_code", code)
                .with_detail("error_message", message);
        }

        match self
            .consumer
            .client()
            .fetch_cluster_id(Timeout::from(Self::TIMEOUT))
        {
            Some(id) => HealthDetail::up().with_detail("cluster_id", id),
            None => HealthDetail::down(),
        }
    }
}
