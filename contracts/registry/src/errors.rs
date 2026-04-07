use core::fmt;
use xlm_ns_common::CommonError;

#[derive(Debug)]
pub enum RegistryError {
    AlreadyRegistered,
    NotFound,
    NotYetClaimable,
    NotActive,
    Unauthorized,
    MetadataTooLong,
    Validation(CommonError),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRegistered => f.write_str("name is already registered"),
            Self::NotFound => f.write_str("name was not found"),
            Self::NotYetClaimable => f.write_str("expired name is still in its grace period"),
            Self::NotActive => f.write_str("name is not currently active"),
            Self::Unauthorized => f.write_str("caller is not authorized for this name"),
            Self::MetadataTooLong => f.write_str("metadata uri exceeds the allowed length"),
            Self::Validation(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for RegistryError {}

impl From<CommonError> for RegistryError {
    fn from(value: CommonError) -> Self {
        Self::Validation(value)
    }
}
