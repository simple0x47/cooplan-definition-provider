use cooplan_amqp_api::api::element::Element;
use cooplan_amqp_api::config::api::config::Config;
use cooplan_amqp_api::error::Error;

use crate::logic::storage_request::StorageRequest;

pub fn register(config: &Config) -> Result<Vec<Element<StorageRequest>>, Error> {
    let elements: Vec<Element<StorageRequest>> =
        vec![crate::api::elements::definition::get(config)?];

    Ok(elements)
}
