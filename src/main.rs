use crate::logic::storage_request::StorageRequest;
use cooplan_amqp_api::api::initialization_package::InitializationPackage;
use logic::latest_definition_updater::LatestDefinitionUpdater;
use std::io::{Error, ErrorKind};
use std::time::Duration;

mod api;
pub mod config;
mod error;
mod logic;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match simple_logger::init() {
        Ok(_) => (),
        Err(error) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("failed to initialize logger: {}", error),
            ));
        }
    }

    let config = match config::config::try_read_config().await {
        Ok(config) => config,
        Err(error) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("failed to read config: {}", error),
            ));
        }
    };

    let (storage_request_sender, storage_request_receiver) =
        async_channel::bounded::<StorageRequest>(config.storage_requests_bound());

    // Set bounds to channel
    let (output_sender, output_receiver) = tokio::sync::mpsc::channel(1024);

    let api_package = InitializationPackage::new(
        storage_request_sender.clone(),
        Box::new(api::input::registration::register),
        output_receiver,
        Box::new(api::output::registration::register),
    );

    match cooplan_amqp_api::api::init::initialize(api_package).await {
        Ok(()) => (),
        Err(error) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("failed to initialize api: {}", error),
            ));
        }
    }

    let latest_definition_updater = LatestDefinitionUpdater::new(
        output_sender,
        config.latest_definition_updater().clone(),
        storage_request_sender,
    );

    tokio::spawn(latest_definition_updater.run());

    match storage::init::initialize(
        config.storage_request_dispatch_instances(),
        config.git(),
        storage_request_receiver,
    )
    .await
    {
        Ok(()) => (),
        Err(error) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("failed to initialize storage: {}", error),
            ));
        }
    }

    std::thread::sleep(Duration::MAX);

    Ok(())
}
