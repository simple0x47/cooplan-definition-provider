use crate::logic::storage_request::StorageRequest;
use cooplan_amqp_api::api::initialization_package::InitializationPackage;
use cooplan_state_tracker::state_tracker::StateTracker;
use cooplan_state_tracker::state_tracker_client::StateTrackerClient;
use cooplan_state_tracker::state_tracking_config::StateTrackingConfig;
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

    let api_file = match std::env::args().nth(1) {
        Some(api_file) => api_file,
        None => return Err(Error::new(ErrorKind::InvalidInput, "no api file provided")),
    };

    let config_file = match std::env::args().nth(2) {
        Some(config_file) => config_file,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "no config file provided",
            ))
        }
    };

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
        async_channel::bounded::<StorageRequest>(config.storage_requests_boundary);

    let (output_sender, output_receiver) =
        tokio::sync::mpsc::channel(config.output_channel_boundary);

    let state_tracker_client = initialize_state_tracking(
        config.state_tracking_config,
        config.state_tracking_channel_boundary,
    );

    let api_package = InitializationPackage::new(
        storage_request_sender.clone(),
        Box::new(api::input::registration::register),
        output_receiver,
        Box::new(api::output::registration::register),
        config.amqp_api_connect,
        api_file,
        config_file,
        state_tracker_client,
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
        config.latest_definition_updater,
        storage_request_sender,
    );

    tokio::spawn(latest_definition_updater.run());

    match storage::init::initialize(
        config.storage_request_dispatch_instances,
        &config.git,
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

fn initialize_state_tracking(
    state_tracking_config: StateTrackingConfig,
    state_tracking_channel_boundary: usize,
) -> StateTrackerClient {
    let (state_sender, state_receiver) =
        tokio::sync::mpsc::channel(state_tracking_channel_boundary);

    let state_update_interval = state_tracking_config.state_sender_interval_in_seconds;

    tokio::spawn(async move {
        let state_tracker = match StateTracker::try_new(
            state_tracking_config.state_output_sender_path.as_str(),
            state_tracking_config.state_output_receiver_path.as_str(),
            state_receiver,
        ) {
            Ok(state_tracker) => state_tracker,
            Err(error) => {
                panic!("failed to initialize state tracker: {}", error);
            }
        };

        state_tracker.run().await;
    });

    StateTrackerClient::new("default".to_string(), state_sender, state_update_interval)
}
