use crate::events::utils;
use crossbeam_channel::Sender;

use log::{info, warn};
use rdkafka::client::ClientContext;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::Message;
use rdkafka::topic_partition_list::TopicPartitionList;
use std::boxed::Box;

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }
    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }
    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
async fn consume_and_print(
    topics: &[&str],
    sender: Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let consumer = utils::get_config_consumer()?;

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                sender.clone().send(payload.to_string()).unwrap();

                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
#[tokio::main]
pub async fn start_consuming(sender: Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
    let topic = "rust-messages";
    let topics = [topic];
    consume_and_print(&topics, sender).await?;
    Ok(())
}
