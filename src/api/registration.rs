use cooplan_amqp_api::api::element::Element;
use cooplan_amqp_api::config::api::config::Config;
use crate::logic::logic_request::LogicRequest;
use cooplan_amqp_api::error::Error;
use crate::api::elements::example;

pub fn register(config: &Config) -> Result<Vec<Element<LogicRequest>>, Error> {
    todo!();

    let elements: Vec<Element<LogicRequest>> = vec![example::get(config)?];

    Ok(elements)
}