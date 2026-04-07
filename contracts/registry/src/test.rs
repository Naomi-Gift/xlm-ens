#[cfg(test)]
mod tests {
    use crate::types::RegistryEntry;
    use crate::RegistryContract;
    use xlm_ns_common::{NameRecord, Tld};

    #[test]
    fn registers_and_transfers_name() {
        let mut registry = RegistryContract::default();
        let entry = RegistryEntry {
            record: NameRecord {
                label: "timmy".into(),
                tld: Tld::Xlm,
                owner: "GABC".into(),
                resolver: None,
                ttl_seconds: 60,
            },
            metadata_uri: None,
        };

        registry.register(entry).unwrap();
        registry.transfer("timmy.xlm", "GDEF").unwrap();

        let stored = registry.resolve("timmy.xlm").unwrap();
        assert_eq!(stored.record.owner, "GDEF");
    }
}
