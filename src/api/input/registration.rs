use cooplan_amqp_api::api::input::input_element::InputElement;
use cooplan_amqp_api::config::api::config::Config;
use cooplan_amqp_api::error::Error;

use crate::logic::storage_request::StorageRequest;

pub fn register(config: &Config) -> Result<Vec<InputElement<StorageRequest>>, Error> {
    let elements: Vec<InputElement<StorageRequest>> =
        vec![crate::api::input::elements::definition::get(config)?];

    Ok(elements)
}
