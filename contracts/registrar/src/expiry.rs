use xlm_ns_common::GRACE_PERIOD_SECONDS;

const YEAR_SECONDS: u64 = 31_536_000;

pub fn expiry_from_now(now_unix: u64, years: u64) -> u64 {
    now_unix.saturating_add(YEAR_SECONDS.saturating_mul(years))
}

pub fn within_grace_period(expiry_unix: u64, now_unix: u64) -> bool {
    now_unix > expiry_unix && now_unix <= expiry_unix.saturating_add(GRACE_PERIOD_SECONDS)
}
