use crate::errors::SdkError;
use crate::types::{RegistrationQuote, RegistrationRequest, RenewalRequest, ResolutionResult};

#[derive(Debug, Clone)]
pub struct XlmNsClient {
    pub rpc_url: String,
}

impl XlmNsClient {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            rpc_url: rpc_url.into(),
        }
    }

    pub fn resolve(&self, name: &str) -> Result<ResolutionResult, SdkError> {
        Ok(ResolutionResult {
            name: name.to_string(),
            address: None,
        })
    }

    pub fn quote_registration(
        &self,
        label: &str,
        duration_years: u32,
    ) -> Result<RegistrationQuote, SdkError> {
        if label.trim().is_empty() {
            return Err(SdkError::InvalidRequest("label must not be empty".into()));
        }

        // Mock logic: 10 XLM per year
        let fee = (duration_years as u64) * 10;
        let expires_at = 1682200000 + (duration_years as u64 * 31536000); // Dummy unix timestamp

        Ok(RegistrationQuote { fee, expires_at })
    }

    pub fn register(&self, request: RegistrationRequest) -> Result<String, SdkError> {
        if request.label.trim().is_empty() {
            return Err(SdkError::InvalidRequest("label must not be empty".into()));
        }

        // Mock successful transaction hash
        Ok("tx_abc123789xyz".to_string())
    }

    pub fn renew(&self, request: RenewalRequest) -> Result<(), SdkError> {
        if request.additional_years == 0 {
            return Err(SdkError::InvalidRequest(
                "additional_years must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}
