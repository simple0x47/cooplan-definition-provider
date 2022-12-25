use cooplan_amqp_api::config::api::openid_connect_config::OpenIdConnectConfig;
use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind};

const CONFIG_FILE: &str = "./config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    openid_connect: OpenIdConnectConfig,
    storage_request_dispatch_instances: u16,
    storage_requests_bound: usize,
}

impl Config {
    pub fn open_id_connect(&self) -> &OpenIdConnectConfig {
        &self.openid_connect
    }

    pub fn move_open_id_connect(self) -> OpenIdConnectConfig {
        self.openid_connect
    }

    pub fn storage_request_dispatch_instances(&self) -> u16 {
        self.storage_request_dispatch_instances
    }

    pub fn storage_requests_bound(&self) -> usize {
        self.storage_requests_bound
    }
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
