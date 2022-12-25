use cooplan_definitions_lib::definition::Definition;
use tokio::sync::oneshot::Sender;

use crate::error::Error;

pub enum DefinitionStorageAction {
    Get {
        version: String,
        replier: Sender<Result<Definition, Error>>,
    },
}
