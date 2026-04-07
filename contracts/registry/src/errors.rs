use thiserror::Error;
use xlm_ns_common::CommonError;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("name is already registered")]
    AlreadyRegistered,
    #[error("name was not found")]
    NotFound,
    #[error(transparent)]
    Validation(#[from] CommonError),
}
