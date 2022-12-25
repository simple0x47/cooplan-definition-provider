use mongodb::Client;
use crate::error::Error;
use crate::logic::actions::example_storage_action::ExampleStorageAction;

pub async fn execute(action: ExampleStorageAction, client: &Client) -> Result<(), Error> {
    match action {
        ExampleStorageAction::Example { } => todo!(),
    }
}