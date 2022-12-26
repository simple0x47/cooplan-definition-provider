use cooplan_definition_git_downloader::downloader::Downloader;
use cooplan_definitions_lib::definition::Definition;
use tokio::sync::oneshot::Sender;

use crate::{
    error::{Error, ErrorKind},
    logic::actions::definition_storage_action::DefinitionStorageAction,
    storage::definition_file_reader::DefinitionFileReader,
};

pub async fn execute(
    action: DefinitionStorageAction,
    git_downloader: &Downloader,
    definition_file_reader: &DefinitionFileReader,
) -> Result<(), Error> {
    match action {
        DefinitionStorageAction::Get { version, replier } => {
            get(version, replier, git_downloader, definition_file_reader)
        }
        DefinitionStorageAction::GetLatest { replier } => Ok(()),
    }
}

fn get(
    version: String,
    replier: Sender<Result<Definition, Error>>,
    git_downloader: &Downloader,
    definition_file_reader: &DefinitionFileReader,
) -> Result<(), Error> {
    match git_downloader.download() {
        Ok(_) => (),
        Err(error) => {
            return Err(Error::new(
                ErrorKind::InternalFailure,
                format!("failed to download definitions: {}", error),
            ))
        }
    }

    match git_downloader.set_version(version.clone()) {
        Ok(_) => (),
        Err(error) => {
            return Err(Error::new(
                ErrorKind::InternalFailure,
                format!(
                    "failed to update definitions to version '{}': {}",
                    version, error
                ),
            ))
        }
    }

    match definition_file_reader.read() {
        Ok(definition) => {
            if let Err(error) = replier.send(Ok(definition)) {
                return Err(Error::new(
                    ErrorKind::InternalFailure,
                    format!("failed to send ok as result"),
                ));
            }
        }
        Err(error) => {
            if let Err(error) = replier.send(Err(error)) {
                return Err(Error::new(
                    ErrorKind::InternalFailure,
                    format!("failed to send error as result"),
                ));
            }
        }
    }

    Ok(())
}
