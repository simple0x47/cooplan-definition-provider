use cooplan_amqp_api::config::openid_connect_config::OpenIdConnectConfig;
use cooplan_definition_git_downloader::git_config::GitConfig;
use cooplan_lapin_wrapper::config::amqp_connect_config::AmqpConnectConfig;
use cooplan_state_tracker::state_tracking_config::StateTrackingConfig;
use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind};

use super::latest_definition_updater_config::LatestDefinitionUpdaterConfig;

const CONFIG_FILE: &str = "./config.json";

#[derive(Deserialize)]
pub struct Config {
    pub openid_connect: OpenIdConnectConfig,
    pub git: GitConfig,
    pub amqp_api_connect: AmqpConnectConfig,
    pub latest_definition_updater: LatestDefinitionUpdaterConfig,
    pub storage_request_dispatch_instances: u16,
    pub storage_requests_boundary: usize,
    pub output_channel_boundary: usize,
    pub state_tracking_config: StateTrackingConfig,
    pub state_tracking_channel_boundary: usize,
}

pub async fn try_read_config() -> Result<Config, Error> {
    let config = match tokio::fs::read_to_string(CONFIG_FILE).await {
        Ok(config) => match serde_json::from_str::<Config>(config.as_str()) {
            Ok(config) => config,
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::AutoConfigFailure,
                    format!("failed to deserialize config file's content: {}", error),
                ));
            }
        },
        Err(error) => {
            return Err(Error::new(
                ErrorKind::AutoConfigFailure,
                format!("failed to read config file: {}", error),
            ));
        }
    };

    Ok(config)
}
