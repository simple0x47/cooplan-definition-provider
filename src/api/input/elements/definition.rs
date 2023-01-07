use std::sync::Arc;

use async_channel::Sender;
use cooplan_amqp_api::api::input::input_element::InputElement;
use cooplan_amqp_api::api::input::request::Request;
use cooplan_amqp_api::error::{Error, ErrorKind};
use cooplan_amqp_api_shared::api::input::request_result::RequestResult;
use cooplan_amqp_api_shared::api::input::request_result_error::{
    RequestResultError, RequestResultErrorKind,
};
use cooplan_definitions_lib::definition::Definition;
use cooplan_lapin_wrapper::config::api::Api;
use serde_json::{Map, Value};

use crate::logic::actions::definition_storage_action::DefinitionStorageAction;
use crate::logic::storage_request::StorageRequest;

const ELEMENT_NAME: &str = "definition";

const VERSION_KEY: &str = "version";

const ACTIONS: &[&str] = &["get"];

pub fn get(config: &Api) -> Result<InputElement<StorageRequest>, Error> {
    let element_config = match config
        .input()
        .iter()
        .find(|element| element.id() == ELEMENT_NAME)
    {
        Some(element) => element,
        None => {
            return Err(Error::new(
                ErrorKind::AutoConfigFailure,
                format!("failed to find element with id '{}'", ELEMENT_NAME),
            ))
        }
    };

    Ok(InputElement::new(
        ELEMENT_NAME.to_string(),
        Arc::new(move |request, storage_request_sender| {
            Box::pin(request_handler(request, storage_request_sender))
        }),
        ACTIONS,
        element_config.clone(),
    ))
}

async fn request_handler(
    request: Request,
    storage_request_sender: Sender<StorageRequest>,
) -> RequestResult {
    let action = match request.try_get_header() {
        Ok(header) => header.action().to_string(),
        Err(error) => return RequestResult::Err(error.into()),
    };

    let data = request.data();

    match action.as_str() {
        "get" => get_action(data, storage_request_sender).await,
        _ => {
            return RequestResult::Err(RequestResultError::new(
                RequestResultErrorKind::MalformedRequest,
                format!("invalid action detected: {}", action),
            ));
        }
    }
}

async fn get_action(
    data: Map<String, Value>,
    storage_request_sender: Sender<StorageRequest>,
) -> RequestResult {
    let version = match data.get(VERSION_KEY) {
        Some(version) => match version.as_str() {
            Some(version) => version.to_string(),
            None => {
                return RequestResult::Err(RequestResultError::new(
                    RequestResultErrorKind::MalformedRequest,
                    format!("failed to read '{}' as a string", VERSION_KEY),
                ))
            }
        },
        None => {
            return RequestResult::Err(RequestResultError::new(
                RequestResultErrorKind::MalformedRequest,
                format!("missing '{}' from request", VERSION_KEY),
            ))
        }
    };

    let (replier, receiver) =
        tokio::sync::oneshot::channel::<Result<Definition, crate::error::Error>>();
    let storage_request =
        StorageRequest::DefinitionRequest(DefinitionStorageAction::Get { version, replier });

    match storage_request_sender.send(storage_request).await {
        Ok(_) => (),
        Err(error) => {
            return RequestResult::Err(RequestResultError::new(
                RequestResultErrorKind::InternalFailure,
                format!("failed to send storage request: {}", error),
            ))
        }
    }

    let serialized_definition = match receiver.await {
        Ok(result) => match result {
            Ok(definition) => match serde_json::to_value(definition) {
                Ok(serialized_definition) => serialized_definition,
                Err(error) => {
                    return RequestResult::Err(RequestResultError::new(
                        RequestResultErrorKind::InternalFailure,
                        format!("failed to serialize definition: {}", error),
                    ));
                }
            },
            Err(error) => {
                return RequestResult::Err(RequestResultError::new(
                    RequestResultErrorKind::InternalFailure,
                    format!("failed to get latest definition: {}", error),
                ))
            }
        },
        Err(error) => {
            return RequestResult::Err(RequestResultError::new(
                RequestResultErrorKind::InternalFailure,
                format!("failed to receive reply from storage: {}", error),
            ))
        }
    };

    RequestResult::Ok(serialized_definition)
}
