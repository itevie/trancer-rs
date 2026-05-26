use std::time::Duration;

pub fn mins(amount: u64) -> u64 {
    Duration::from_mins(amount).as_secs()
}

pub fn hours(amount: u64) -> u64 {
    Duration::from_hours(amount).as_secs()
}
