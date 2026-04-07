use std::collections::HashMap;

use crate::errors::RegistryError;
use crate::types::RegistryEntry;

#[derive(Debug, Default)]
pub struct RegistryStorage {
    entries: HashMap<String, RegistryEntry>,
}

impl RegistryStorage {
    pub fn insert(&mut self, entry: RegistryEntry) -> Result<(), RegistryError> {
        let key = entry.record.fqdn();
        if self.entries.contains_key(&key) {
            return Err(RegistryError::AlreadyRegistered);
        }

        self.entries.insert(key, entry);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&RegistryEntry> {
        self.entries.get(name)
    }

    pub fn transfer(&mut self, name: &str, new_owner: String) -> Result<(), RegistryError> {
        let entry = self
            .entries
            .get_mut(name)
            .ok_or(RegistryError::NotFound)?;
        entry.record.owner = new_owner;
        Ok(())
    }
}
