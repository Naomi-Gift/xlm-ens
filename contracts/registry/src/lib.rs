pub mod errors;
pub mod storage;
pub mod test;
pub mod types;

use errors::RegistryError;
use storage::RegistryStorage;
use types::RegistryEntry;
use xlm_ns_common::validation::{parse_fqdn, validate_label, validate_owner};
use xlm_ns_common::MAX_METADATA_URI_LENGTH;

#[derive(Debug, Default)]
pub struct RegistryContract {
    storage: RegistryStorage,
}

impl RegistryContract {
    pub fn register(&mut self, entry: RegistryEntry, now_unix: u64) -> Result<(), RegistryError> {
        validate_label(&entry.record.label).map_err(RegistryError::Validation)?;
        validate_owner(&entry.record.owner).map_err(RegistryError::Validation)?;
        self.validate_metadata(entry.metadata_uri.as_deref())?;

        let name = entry.record.fqdn();
        if let Some(existing) = self.storage.get(&name) {
            if existing.is_active_at(now_unix) {
                return Err(RegistryError::AlreadyRegistered);
            }

            if !existing.is_claimable_at(now_unix) {
                return Err(RegistryError::NotYetClaimable);
            }

            self.storage.remove(&name);
        }

        self.storage.insert(entry)
    }

    pub fn resolve(&self, name: &str, now_unix: u64) -> Result<&RegistryEntry, RegistryError> {
        parse_fqdn(name).map_err(RegistryError::Validation)?;
        let entry = self.storage.get(name).ok_or(RegistryError::NotFound)?;

        if !entry.is_active_at(now_unix) {
            return Err(RegistryError::NotActive);
        }

        Ok(entry)
    }

    pub fn transfer(
        &mut self,
        name: &str,
        caller: &str,
        new_owner: impl Into<String>,
        now_unix: u64,
    ) -> Result<(), RegistryError> {
        let new_owner = new_owner.into();
        validate_owner(&new_owner).map_err(RegistryError::Validation)?;

        let entry = self.storage.get_mut(name)?;
        if !entry.is_active_at(now_unix) {
            return Err(RegistryError::NotActive);
        }
        if entry.record.owner != caller {
            return Err(RegistryError::Unauthorized);
        }

        let old_owner = entry.record.owner.clone();
        entry.record.set_owner(new_owner.clone());
        entry.transfer_count = entry.transfer_count.saturating_add(1);
        entry.touch(now_unix);
        self.storage.reindex_owner(name, &old_owner, &new_owner);
        Ok(())
    }

    pub fn set_resolver(
        &mut self,
        name: &str,
        caller: &str,
        resolver: Option<String>,
        now_unix: u64,
    ) -> Result<(), RegistryError> {
        let entry = self.storage.get_mut(name)?;
        Self::ensure_owner(entry, caller, now_unix)?;
        entry.record.set_resolver(resolver);
        entry.touch(now_unix);
        Ok(())
    }

    pub fn set_target_address(
        &mut self,
        name: &str,
        caller: &str,
        target_address: Option<String>,
        now_unix: u64,
    ) -> Result<(), RegistryError> {
        let entry = self.storage.get_mut(name)?;
        Self::ensure_owner(entry, caller, now_unix)?;
        entry.record.set_target_address(target_address);
        entry.touch(now_unix);
        Ok(())
    }

    pub fn set_metadata(
        &mut self,
        name: &str,
        caller: &str,
        metadata_uri: Option<String>,
        now_unix: u64,
    ) -> Result<(), RegistryError> {
        self.validate_metadata(metadata_uri.as_deref())?;
        let entry = self.storage.get_mut(name)?;
        Self::ensure_owner(entry, caller, now_unix)?;
        entry.metadata_uri = metadata_uri;
        entry.touch(now_unix);
        Ok(())
    }

    pub fn renew(
        &mut self,
        name: &str,
        caller: &str,
        expires_at: u64,
        grace_period_ends_at: u64,
        now_unix: u64,
    ) -> Result<(), RegistryError> {
        let entry = self.storage.get_mut(name)?;
        Self::ensure_owner(entry, caller, now_unix)?;
        entry
            .record
            .extend_expiry(expires_at, grace_period_ends_at);
        entry.touch(now_unix);
        Ok(())
    }

    pub fn names_for_owner(&self, owner: &str) -> Vec<String> {
        self.storage
            .names_for_owner(owner)
            .into_iter()
            .map(|entry| entry.record.fqdn())
            .collect()
    }

    fn validate_metadata(&self, metadata_uri: Option<&str>) -> Result<(), RegistryError> {
        if metadata_uri
            .map(|value| value.len() > MAX_METADATA_URI_LENGTH)
            .unwrap_or(false)
        {
            return Err(RegistryError::MetadataTooLong);
        }

        Ok(())
    }

    fn ensure_owner(entry: &RegistryEntry, caller: &str, now_unix: u64) -> Result<(), RegistryError> {
        if !entry.is_active_at(now_unix) {
            return Err(RegistryError::NotActive);
        }

        if entry.record.owner != caller {
            return Err(RegistryError::Unauthorized);
        }

        Ok(())
    }
}
