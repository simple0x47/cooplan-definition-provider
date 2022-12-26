use crate::error::Error;
use crate::logic::storage_request::StorageRequest;
use async_channel::Receiver;
use cooplan_definition_git_downloader::git_config::GitConfig;

pub async fn initialize(
    concurrent_dispatchers: u16,
    git_config: &GitConfig,
    request_receiver: Receiver<StorageRequest>,
) -> Result<(), Error> {
    for id in 0..concurrent_dispatchers {
        let git_request_dispatch = crate::storage::git_request_dispatch::GitRequestDispatch::new(
            id.into(),
            git_config.clone(),
            request_receiver.clone(),
        );

        tokio::spawn(git_request_dispatch.run());
    }

    Ok(())
}
