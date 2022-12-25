use super::actions::definition_storage_action::DefinitionStorageAction;

pub enum StorageRequest {
    DefinitionRequest(DefinitionStorageAction),
}
