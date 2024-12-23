use anyhow::Result;

use futures::stream::StreamExt;
use rdkafka::consumer::Consumer;
use rdkafka::message::Message;
use rust_kafka_simple::{config::config::KafkaConnCfg, utils::kafka::kafka_consumer_conn};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = KafkaConnCfg {
        url: "localhost:9092".to_string(),
        topic: "test".to_string(),
    };

    let consumer = kafka_consumer_conn(&cfg.url)?;

    consumer.subscribe(&[&cfg.topic])?;

    println!("Consumer started. Waiting for messages...");

    let mut message_stream = consumer.stream();
    while let Some(message) = message_stream.next().await {
        match message {
            Ok(msg) => {
                // Deserialize the payload
                if let Some(payload) = msg.payload_view::<str>() {
                    match payload {
                        Ok(data) => {
                            println!(
                                "Received message: '{}', on partition: {}, offset: {}",
                                data,
                                msg.partition(),
                                msg.offset()
                            );
                        }
                        Err(e) => eprintln!("Error deserializing message payload: {:?}", e),
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {:?}", e),
        }
    }

    Ok(())
}
