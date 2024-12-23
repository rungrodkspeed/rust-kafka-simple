use anyhow::{Context, Result};
use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;

pub fn kafka_consumer_conn(url: &str) -> Result<StreamConsumer> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", url)
        .set("group.id", "default_group")
        .set("auto.offset.reset", "earliest") // Ensures consumer starts at the beginning if offsets are missing
        .set("enable.auto.commit", "true") // Automatically commits offsets
        .set("allow.auto.create.topics", "false") // Prevents accidental topic creation
        .create()
        .context("Failed to create Kafka consumer")?;

    Ok(consumer)
}

pub fn kafka_producer_conn(url: &str) -> Result<FutureProducer> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", url)
        .set("enable.idempotence", "true") // Ensures message ordering and delivery guarantees
        .set("acks", "all") // Wait for all replicas to acknowledge
        .set("retries", "5") // Retry on transient errors
        .set("linger.ms", "5") // Batching optimization
        .set("batch.size", "16384") // Adjust batch size for performance
        .create()
        .context("Failed to create Kafka producer")?;

    Ok(producer)
}
