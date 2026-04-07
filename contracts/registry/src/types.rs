use xlm_ns_common::NameRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryEntry {
    pub record: NameRecord,
    pub metadata_uri: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub transfer_count: u32,
}

impl RegistryEntry {
    pub fn new(record: NameRecord, metadata_uri: Option<String>, now_unix: u64) -> Self {
        Self {
            record,
            metadata_uri,
            created_at: now_unix,
            updated_at: now_unix,
            transfer_count: 0,
        }
    }

    pub fn touch(&mut self, now_unix: u64) {
        self.updated_at = now_unix;
    }

    pub fn is_active_at(&self, now_unix: u64) -> bool {
        self.record.is_active_at(now_unix)
    }

    pub fn is_claimable_at(&self, now_unix: u64) -> bool {
        self.record.is_claimable_at(now_unix)
    }
}
