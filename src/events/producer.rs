use crate::events::utils;

use rdkafka::producer::{FutureProducer, FutureRecord};
use std::boxed::Box;
use std::time::{SystemTime, UNIX_EPOCH};
#[tokio::main]
pub async fn produce_event(message: String) -> Result<(), Box<dyn std::error::Error>> {
    let config = utils::get_config_producer()?;
    let producer: &FutureProducer = &config.create().expect("producer error");
    let topic = String::from("rust-messages");
    let i = 0_usize;

    let _result = producer
        .send_result(
            FutureRecord::to(&topic)
                .key(&i.to_string())
                .payload(&message)
                .timestamp(now()),
        )
        .unwrap()
        .await;
    Ok(())
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap()
}
