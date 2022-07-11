use rdkafka::config::ClientConfig;

use std::env;
use std::error::Error;

pub fn get_config() -> Result<ClientConfig, Box<dyn Error>> {
    let brokers = env::var("BOOTSTRAP_SERVER").expect("broker must be set");
    let sasl_mechanism = env::var("SASL_MECHANISM").expect("sasl mech must be set");
    let sasl_username = env::var("SASL_USERNAME").expect("sasl username must be set");
    let sasl_password = env::var("SASL_PASSWORD").expect("sasl password must be set");
    let mut kafka_config = ClientConfig::new();

    kafka_config
        .set("bootstrap.servers", brokers)
        .set("session.timeout.ms", "6000")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", sasl_mechanism)
        .set("sasl.username", sasl_username)
        .set("sasl.password", sasl_password);

    Ok(kafka_config)
}
