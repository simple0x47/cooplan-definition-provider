use cooplan_amqp_api::api::element::Element;
use cooplan_amqp_api::config::api::config::Config;
use crate::logic::logic_request::LogicRequest;
use cooplan_amqp_api::error::{Error};


pub fn get(config: &Config) -> Result<Element<LogicRequest>, Error> {
    todo!()
}