use std::time::{SystemTime, UNIX_EPOCH};

use crate::VERIFY_ID_INVALID_HOURS;

pub fn timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn is_verify_id_valid(earlier: u64, now: u64) -> bool {
    now - earlier <= 60 * 60 * *VERIFY_ID_INVALID_HOURS as u64
}

#[cfg(test)]
mod tests {
    use super::{is_verify_id_valid, timestamp_secs};

    #[test]
    fn test_is_verify_id_valid() {
        let valid = is_verify_id_valid(timestamp_secs(), timestamp_secs() + 60 * 5);
        assert!(valid);

        let valid = is_verify_id_valid(timestamp_secs(), timestamp_secs() + 60 * 60 * 25);
        assert!(!valid);
    }
}