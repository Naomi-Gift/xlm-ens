use crate::{NftContract, NftError};

impl NftContract {
    pub fn approve(&mut self, token_id: &str, caller: &str, approved: &str) -> Result<(), NftError> {
        let record = self.tokens.get_mut(token_id).ok_or(NftError::NotFound)?;
        if record.owner != caller {
            return Err(NftError::Unauthorized);
        }

        record.approved = Some(approved.to_string());
        Ok(())
    }

    pub fn transfer(
        &mut self,
        token_id: &str,
        caller: &str,
        new_owner: impl Into<String>,
    ) -> Result<(), NftError> {
        let record = self.tokens.get_mut(token_id).ok_or(NftError::NotFound)?;
        let authorized = record.owner == caller || record.approved.as_deref() == Some(caller);
        if !authorized {
            return Err(NftError::Unauthorized);
        }

        record.owner = new_owner.into();
        record.approved = None;
        Ok(())
    }
}
