pub mod axelar;
pub mod evm_resolver;
pub mod test;

use std::collections::HashMap;

use axelar::build_gmp_message;
pub use evm_resolver::{target_for_chain, EvmTarget};
use xlm_ns_common::validation::{parse_fqdn, validate_chain_name};
use xlm_ns_common::CommonError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRoute {
    pub destination_chain: String,
    pub destination_resolver: String,
    pub gateway: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeError {
    Validation(CommonError),
    UnsupportedChain,
}

impl core::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Validation(error) => write!(f, "{error}"),
            Self::UnsupportedChain => f.write_str("destination chain is not supported"),
        }
    }
}

impl std::error::Error for BridgeError {}

impl From<CommonError> for BridgeError {
    fn from(value: CommonError) -> Self {
        Self::Validation(value)
    }
}

#[derive(Debug, Default)]
pub struct BridgeContract {
    routes: HashMap<String, BridgeRoute>,
}

impl BridgeContract {
    pub fn register_chain(&mut self, chain: &str) -> Result<(), BridgeError> {
        validate_chain_name(chain)?;
        let target = target_for_chain(chain).ok_or(BridgeError::UnsupportedChain)?;
        self.routes.insert(
            chain.to_string(),
            BridgeRoute {
                destination_chain: target.chain.to_string(),
                destination_resolver: target.resolver.to_string(),
                gateway: target.gateway.to_string(),
            },
        );
        Ok(())
    }

    pub fn build_message(&self, name: &str, chain: &str) -> Result<String, BridgeError> {
        parse_fqdn(name)?;
        validate_chain_name(chain)?;
        let route = self.routes.get(chain).ok_or(BridgeError::UnsupportedChain)?;

        Ok(build_gmp_message(
            name,
            &route.destination_chain,
            &route.destination_resolver,
        ))
    }

    pub fn route(&self, chain: &str) -> Option<&BridgeRoute> {
        self.routes.get(chain)
    }
}
