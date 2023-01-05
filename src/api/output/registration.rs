use cooplan_amqp_api::{api::output::amqp_output_element::AmqpOutputElement, error::Error};
use cooplan_lapin_wrapper::config::api::Api;
use cooplan_state_tracker::state_tracker_client::StateTrackerClient;

use super::elements::latest_definition;

pub fn register(
    config: &Api,
    state_tracker: StateTrackerClient,
) -> Result<Vec<AmqpOutputElement>, Error> {
    let elements: Vec<AmqpOutputElement> = vec![latest_definition::get(config, state_tracker)?];

    Ok(elements)
}
