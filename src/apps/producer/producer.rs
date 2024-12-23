use anyhow::Result;

use rdkafka::{producer::FutureRecord, util::Timeout};
use rust_kafka_simple::{
    config::config::KafkaConnCfg,
    models::example::Example,
    utils::{helper::compress_to_json_bytes, kafka::kafka_producer_conn},
};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = KafkaConnCfg {
        url: "localhost:9092".to_string(),
        topic: "test".to_string(),
    };

    let producer = kafka_producer_conn(&cfg.url)?;

    let examples = vec![
        Example {
            id: "2dc7cf08-e238-4faa-bd5f-f1cfe2e0b565".to_string(),
            name: "Coffee".to_string(),
        },
        Example {
            id: "4c56ec5b-d638-42f2-ae1d-38b6fc6d2122".to_string(),
            name: "Tea".to_string(),
        },
        Example {
            id: "36da5a84-f333-4ecf-a2fe-130c3e8d4ef1".to_string(),
            name: "Milk".to_string(),
        },
    ];

    for example in examples {
        let payload = compress_to_json_bytes(&example);

        match producer
            .send(
                FutureRecord::to(&cfg.topic)
                    .payload(&payload)
                    .key(&example.id),
                Timeout::Never,
            )
            .await
        {
            Ok(delivery) => println!("Delivered to partition: {:?}", delivery),
            Err(e) => println!("Error sending message: {:?}", e),
        };
    }

    Ok(())
}
