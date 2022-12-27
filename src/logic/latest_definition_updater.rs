use std::{fmt::Display, time::Duration};

use async_recursion::async_recursion;
use cooplan_definitions_lib::definition::Definition;
use serde_json::Value;
use tokio::sync::mpsc::Sender;

use crate::config::latest_definition_updater_config::LatestDefinitionUpdaterConfig;

use super::{
    actions::definition_storage_action::DefinitionStorageAction, storage_request::StorageRequest,
};

const LATEST_DEFINITION_OUTPUT_KEY: &str = "latest_definition";

pub struct LatestDefinitionUpdater {
    latest_definition_output: Sender<(String, Value)>,
    config: LatestDefinitionUpdaterConfig,
    storage_request_sender: async_channel::Sender<StorageRequest>,
    errors_since_last_success: u32,
}

impl LatestDefinitionUpdater {
    pub fn new(
        latest_definition_output: Sender<(String, Value)>,
        config: LatestDefinitionUpdaterConfig,
        storage_request_sender: async_channel::Sender<StorageRequest>,
    ) -> LatestDefinitionUpdater {
        LatestDefinitionUpdater {
            latest_definition_output,
            config,
            storage_request_sender,
            errors_since_last_success: 0u32,
        }
    }

    pub async fn run(mut self) {
        let update_interval_seconds: u64 = self.config.update_interval_seconds;

        self.send_update_request().await;

        loop {
            tokio::time::sleep(Duration::from_secs(update_interval_seconds)).await;

            self.send_update_request().await;
        }
    }

    #[async_recursion]
    async fn send_update_request(&mut self) {
        let (replier, receiver) =
            tokio::sync::oneshot::channel::<Result<Definition, crate::error::Error>>();

        let definition = match self
            .storage_request_sender
            .send(StorageRequest::DefinitionRequest(
                DefinitionStorageAction::GetLatest { replier },
            ))
            .await
        {
            Ok(_) => match receiver.await {
                Ok(result) => match result {
                    Ok(definition) => definition,
                    Err(error) => {
                        self.handle_error(error).await;
                        return;
                    }
                },
                Err(error) => {
                    self.handle_error(error).await;
                    return;
                }
            },
            Err(error) => {
                self.handle_error(error).await;
                return;
            }
        };

        let serialized_definition = match serde_json::to_value(definition) {
            Ok(serialized_definition) => serialized_definition,
            Err(error) => {
                self.handle_error(error).await;
                return;
            }
        };

        match self
            .latest_definition_output
            .send((
                LATEST_DEFINITION_OUTPUT_KEY.to_string(),
                serialized_definition,
            ))
            .await
        {
            Ok(_) => {
                self.errors_since_last_success = 0;
            }
            Err(error) => {
                self.handle_error(error).await;
                return;
            }
        }
    }

    async fn handle_error<T: Display>(&mut self, error: T) {
        self.errors_since_last_success += 1;
        log::error!("error: {}", error);

        if self.errors_since_last_success >= self.config.retry_count {
            log::error!("too many errors, shutting down");
            std::process::exit(1);
        } else {
            tokio::time::sleep(Duration::from_secs(self.config.retry_interval_seconds)).await;

            self.send_update_request().await;
        }
    }
}
