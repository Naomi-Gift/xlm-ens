use crate::ResolverContract;

impl ResolverContract {
    pub fn has_forward_record(&self, name: &str) -> bool {
        self.resolve(name).is_some()
    }
}
