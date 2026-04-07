use serde::{Deserialize, Serialize};
use xlm_ns_common::NameRecord;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub record: NameRecord,
    pub metadata_uri: Option<String>,
}
