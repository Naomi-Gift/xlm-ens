pub mod errors;
pub mod storage;
pub mod test;
pub mod types;

use errors::RegistryError;
use storage::RegistryStorage;
use types::RegistryEntry;
use xlm_ns_common::validation::validate_label;

#[derive(Debug, Default)]
pub struct RegistryContract {
    storage: RegistryStorage,
}

impl RegistryContract {
    pub fn register(&mut self, entry: RegistryEntry) -> Result<(), RegistryError> {
        validate_label(&entry.record.label).map_err(RegistryError::Validation)?;
        self.storage.insert(entry)
    }

    pub fn resolve(&self, name: &str) -> Option<&RegistryEntry> {
        self.storage.get(name)
    }

    pub fn transfer(&mut self, name: &str, new_owner: impl Into<String>) -> Result<(), RegistryError> {
        self.storage.transfer(name, new_owner.into())
    }
}
