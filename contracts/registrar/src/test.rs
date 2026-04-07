#[cfg(test)]
mod tests {
    use crate::expiry::{expiry_from_now, within_grace_period};
    use crate::pricing::price_for_label;
    use crate::{can_renew, quote_registration, RegistrarContract};

    #[test]
    fn applies_tiered_pricing() {
        assert_eq!(price_for_label("abc"), 1_000_000_000);
        assert_eq!(price_for_label("timmy"), 250_000_000);
        assert_eq!(price_for_label("verylongname"), 100_000_000);
    }

    #[test]
    fn computes_expiry_and_grace_period() {
        let expiry = expiry_from_now(100, 1);
        assert!(within_grace_period(expiry, expiry + 10));
        assert!(can_renew(expiry, expiry + 10));
    }

    #[test]
    fn quotes_fee_and_expiry() {
        let quote = quote_registration("timmy", 2, 100);
        assert_eq!(quote.fee_stroops, 500_000_000);
        assert_eq!(quote.expiry_unix, 63_072_100);
        assert!(quote.grace_period_ends_at > quote.expiry_unix);
    }

    #[test]
    fn registers_and_renews_names() {
        let mut registrar = RegistrarContract::default();
        registrar.register("timmy", "alice", 1, 250_000_000, 100).unwrap();
        assert!(!registrar.is_available("timmy", 200));

        registrar
            .renew("timmy.xlm", "alice", 1, 250_000_000, 200)
            .unwrap();

        let record = registrar.registration("timmy.xlm").unwrap();
        assert_eq!(record.record.owner, "alice");
        assert!(registrar.treasury_balance() >= 500_000_000);
    }
}
