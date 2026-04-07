pub fn price_for_label(label: &str) -> u64 {
    match label.len() {
        0..=3 => 1_000_000_000,
        4..=6 => 250_000_000,
        _ => 100_000_000,
    }
}
