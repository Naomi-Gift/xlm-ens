use crate::ResolverContract;

impl ResolverContract {
    pub fn has_reverse_record(&self, address: &str) -> bool {
        self.reverse(address).is_some()
    }
}
