use cooplan_amqp_api::error::ErrorKind;
use cooplan_amqp_api::{api::output::amqp_output_element::AmqpOutputElement, error::Error};
use cooplan_lapin_wrapper::config::api::Api;
use cooplan_state_tracker::state_tracker_client::StateTrackerClient;

const ELEMENT_NAME: &str = "latest_definition";

pub fn get(api: &Api, state_sender: StateTrackerClient) -> Result<AmqpOutputElement, Error> {
    let api_config = match api
        .output()
        .iter()
        .find(|element| element.id() == ELEMENT_NAME)
    {
        Some(element_config) => element_config,
        None => {
            return Err(Error::new(
                ErrorKind::AutoConfigFailure,
                format!("failed to find output api with id '{}'", ELEMENT_NAME),
            ))
        }
    };

    Ok(AmqpOutputElement::new(
        ELEMENT_NAME.to_string(),
        api_config.clone(),
        state_sender,
    ))
}
