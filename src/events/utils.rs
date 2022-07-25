use log::info;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::{ClientContext, TopicPartitionList};
use std::env::{self};
use std::error::Error;

pub fn get_config_producer() -> Result<ClientConfig, Box<dyn Error>> {
    let mut kafka_config = ClientConfig::new();
    let env_vars = EnvVars::get_env_vars();
    kafka_config
        .set("bootstrap.servers", env_vars.brokers)
        .set("session.timeout.ms", "4500")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", env_vars.sasl_mechanism)
        .set("sasl.username", env_vars.sasl_username)
        .set("sasl.password", env_vars.sasl_password);

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

struct EnvVars {
    brokers: String,
    sasl_mechanism: String,
    sasl_username: String,
    sasl_password: String,
    group_id: String,
}
impl EnvVars {
    fn get_env_vars() -> EnvVars {
        let env_vars = EnvVars {
            brokers: env::var("BOOTSTRAP_SERVER").expect("broker must be set"),
            sasl_mechanism: env::var("SASL_MECHANISM").expect("sasl mech must be set"),
            sasl_username: env::var("SASL_USERNAME").expect("sasl username must be set"),
            sasl_password: env::var("SASL_PASSWORD").expect("sasl password must be set"),
            group_id: env::var("GROUP_ID").expect("group id must be set"),
        };

        env_vars
    }
}
pub fn get_config_consumer() -> Result<StreamConsumer<CustomContext>, Box<dyn Error>> {
    let context = CustomContext;
    type LoggingConsumer = StreamConsumer<CustomContext>;
    let env_vars = EnvVars::get_env_vars();
    let kafka_config: LoggingConsumer = ClientConfig::new()
        .set("enable.auto.commit", "true")
        .set("bootstrap.servers", env_vars.brokers)
        .set("group.id", env_vars.group_id)
        .set("session.timeout.ms", "6000")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", env_vars.sasl_mechanism)
        .set("sasl.username", env_vars.sasl_username)
        .set("sasl.password", env_vars.sasl_password)
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");
    Ok(kafka_config)
}
