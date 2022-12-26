use cooplan_amqp_api::{
    api::output::amqp_output_element::AmqpOutputElement, config::api::config::Config, error::Error,
};

const ELEMENT_NAME: &str = "latest_definition";

pub fn get(config: &Config) -> Result<AmqpOutputElement, Error> {
    let element_config = config.try_get_output_element_config("latest_definition")?;

    Ok(AmqpOutputElement::new(
        ELEMENT_NAME.to_string(),
        element_config,
    ))
}
