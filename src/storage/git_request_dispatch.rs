use async_channel::Receiver;
use cooplan_definition_git_downloader::{downloader::Downloader, git_config::GitConfig};

use crate::logic::storage_request::StorageRequest;

use super::definition_file_reader::DefinitionFileReader;

pub struct GitRequestDispatch {
    git_downloader: Downloader,
    definition_file_reader: DefinitionFileReader,
    request_receiver: Receiver<StorageRequest>,
}

impl GitRequestDispatch {
    pub fn new(
        id: u32,
        mut git_config: GitConfig,
        request_receiver: Receiver<StorageRequest>,
    ) -> GitRequestDispatch {
        let repository_dir = format!("./{}", id);

        git_config.repository_local_dir = repository_dir.clone();

        let git_downloader = Downloader::new(git_config);
        let definition_file_reader = DefinitionFileReader::new(repository_dir);

        GitRequestDispatch {
            git_downloader,
            definition_file_reader,
            request_receiver,
        }
    }

    pub async fn run(self) {
        loop {
            match self.request_receiver.recv().await {
                Ok(request) => {
                    log::info!("received storage request");

                    let result = match request {
                        StorageRequest::DefinitionRequest(action) => {
                            crate::storage::executors::definition::execute(
                                action,
                                &self.git_downloader,
                                &self.definition_file_reader,
                            )
                            .await
                        }
                    };

                    if let Err(error) = result {
                        log::info!("failed to execute storage request: {}", error);
                    }
                }
                Err(error) => {
                    log::error!("failed to receive request: {}", error);
                }
            }
        }
    }
}
