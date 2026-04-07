pub mod forward;
pub mod reverse;
pub mod test;

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ResolverContract {
    forward_records: HashMap<String, String>,
    reverse_records: HashMap<String, String>,
}

impl ResolverContract {
    pub fn set_forward(&mut self, name: impl Into<String>, address: impl Into<String>) {
        self.forward_records.insert(name.into(), address.into());
    }

    pub fn resolve(&self, name: &str) -> Option<&str> {
        self.forward_records.get(name).map(String::as_str)
    }

    pub fn set_reverse(&mut self, address: impl Into<String>, name: impl Into<String>) {
        self.reverse_records.insert(address.into(), name.into());
    }

    pub fn reverse(&self, address: &str) -> Option<&str> {
        self.reverse_records.get(address).map(String::as_str)
    }
}
