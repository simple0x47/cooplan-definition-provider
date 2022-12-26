use cooplan_amqp_api::{
    api::output::amqp_output_element::AmqpOutputElement, config::api::config::Config, error::Error,
};

use super::elements::latest_definition;

pub fn register(config: &Config) -> Result<Vec<AmqpOutputElement>, Error> {
    let elements: Vec<AmqpOutputElement> = vec![latest_definition::get(config)?];

    Ok(elements)
}
