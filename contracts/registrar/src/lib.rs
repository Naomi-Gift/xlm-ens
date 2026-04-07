pub mod expiry;
pub mod pricing;
pub mod test;

use expiry::{expiry_from_now, within_grace_period};
use pricing::price_for_label;

#[derive(Debug, Clone)]
pub struct RegistrationQuote {
    pub fee_stroops: u64,
    pub expiry_unix: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum RegistrarError {
    #[error("fee paid is below required amount")]
    InsufficientFee,
}

pub fn quote_registration(label: &str, years: u64, now_unix: u64) -> RegistrationQuote {
    let annual_fee = price_for_label(label);

    RegistrationQuote {
        fee_stroops: annual_fee.saturating_mul(years),
        expiry_unix: expiry_from_now(now_unix, years),
    }
}

pub fn can_renew(expiry_unix: u64, now_unix: u64) -> bool {
    now_unix <= expiry_unix || within_grace_period(expiry_unix, now_unix)
}
