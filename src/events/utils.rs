use log::info;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::{ClientContext, TopicPartitionList};

use std::env;
use std::error::Error;

pub fn get_config_producer() -> Result<ClientConfig, Box<dyn Error>> {
    let brokers = env::var("BOOTSTRAP_SERVER").expect("broker must be set");
    let sasl_mechanism = env::var("SASL_MECHANISM").expect("sasl mech must be set");
    let sasl_username = env::var("SASL_USERNAME").expect("sasl username must be set");
    let sasl_password = env::var("SASL_PASSWORD").expect("sasl password must be set");
    let mut kafka_config = ClientConfig::new();

    kafka_config
        .set("bootstrap.servers", brokers)
        .set("session.timeout.ms", "4500")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", sasl_mechanism)
        .set("sasl.username", sasl_username)
        .set("sasl.password", sasl_password);

    Ok(kafka_config)
}
pub struct CustomContext;

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

pub fn get_config_consumer() -> Result<StreamConsumer<CustomContext>, Box<dyn Error>> {
    let context = CustomContext;
    type LoggingConsumer = StreamConsumer<CustomContext>;
    let brokers = env::var("BOOTSTRAP_SERVER").expect("broker must be set");
    let sasl_mechanism = env::var("SASL_MECHANISM").expect("sasl mech must be set");
    let sasl_username = env::var("SASL_USERNAME").expect("sasl username must be set");
    let sasl_password = env::var("SASL_PASSWORD").expect("sasl password must be set");
    let group_id = env::var("GROUP_ID").expect("group id must be set");

    let kafka_config: LoggingConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group_id)
        .set("session.timeout.ms", "4500")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", sasl_mechanism)
        .set("sasl.username", sasl_username)
        .set("sasl.password", sasl_password)
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    Ok(kafka_config)
}
