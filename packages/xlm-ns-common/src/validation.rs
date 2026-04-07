use crate::constants::{
    MAX_NAME_LENGTH, MAX_REGISTRATION_YEARS, MIN_NAME_LENGTH, MIN_REGISTRATION_YEARS,
};
use crate::errors::CommonError;
use crate::types::Tld;

pub fn validate_label(label: &str) -> Result<(), CommonError> {
    let len = label.len();

    if len < MIN_NAME_LENGTH {
        return Err(CommonError::NameTooShort);
    }

    if len > MAX_NAME_LENGTH {
        return Err(CommonError::NameTooLong);
    }

    if !label
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        return Err(CommonError::InvalidCharacters);
    }

    if label.starts_with('-') || label.ends_with('-') {
        return Err(CommonError::InvalidLabelBoundary);
    }

    Ok(())
}

pub fn validate_owner(owner: &str) -> Result<(), CommonError> {
    if owner.trim().is_empty() {
        return Err(CommonError::EmptyOwner);
    }

    Ok(())
}

pub fn validate_registration_years(years: u64) -> Result<(), CommonError> {
    if !(MIN_REGISTRATION_YEARS..=MAX_REGISTRATION_YEARS).contains(&years) {
        return Err(CommonError::InvalidRegistrationPeriod);
    }

    Ok(())
}

pub fn parse_fqdn(name: &str) -> Result<(String, Tld), CommonError> {
    let mut parts = name.split('.');
    let label = parts.next().ok_or(CommonError::InvalidName)?;
    let tld = parts.next().ok_or(CommonError::MissingTld)?;

    if parts.next().is_some() {
        return Err(CommonError::InvalidName);
    }

    validate_label(label)?;
    let parsed_tld = Tld::parse(tld).ok_or(CommonError::UnsupportedTld)?;

    Ok((label.to_string(), parsed_tld))
}

pub fn validate_chain_name(chain: &str) -> Result<(), CommonError> {
    if chain.trim().is_empty() {
        return Err(CommonError::EmptyChainName);
    }

    Ok(())
}
