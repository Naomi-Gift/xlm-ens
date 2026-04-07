#[cfg(test)]
mod tests {
    use crate::ResolverContract;

    #[test]
    fn stores_forward_and_reverse_records() {
        let mut resolver = ResolverContract::default();
        resolver.set_forward("timmy.xlm", "GABC");
        resolver.set_reverse("GABC", "timmy.xlm");

        assert_eq!(resolver.resolve("timmy.xlm"), Some("GABC"));
        assert_eq!(resolver.reverse("GABC"), Some("timmy.xlm"));
        assert!(resolver.has_forward_record("timmy.xlm"));
        assert!(resolver.has_reverse_record("GABC"));
    }
}
