use std::time::{Duration, SystemTime};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TimeRange {
    from: SystemTime,
    to: SystemTime,
}

#[derive(Debug)]
pub enum FactoryError {
    FromAfterTo,
}
use FactoryError::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TimeList {
    list: Vec<SystemTime>,
}

impl TimeRange {
    pub fn new(from: SystemTime, to: SystemTime) -> Result<Self, FactoryError> {
        if from > to {
            Err(FromAfterTo)
        } else {
            Ok(Self { from, to })
        }
    }

    pub fn make_timelist(&self, period: &Duration, offset: &Duration) -> TimeList {
        // start is the leading edge of the range, minus the offset (for now)
        let start = self.from - offset;
        // make a duration between start and midnight
        // let start_after_midnight = ...
        // divide that by period to get x

        // our initial time is (x * period) + offset
        // stack on one per period, accumulating the period
        // until our accum is larger than our end time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn cmp() {
        let five_seconds = Duration::new(5, 0);
        let now = SystemTime::now();
        assert!(
            TimeRange::new(now - five_seconds * 4, now - five_seconds * 3).unwrap()
                < TimeRange::new(now - five_seconds, now).unwrap()
        );
    }
}
