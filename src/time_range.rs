use super::display_systime;
use super::TimeList;
use std::fmt;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TimeRange {
    pub from: SystemTime,
    pub to: SystemTime,
}

#[derive(Debug)]
pub enum TimeRangeError {
    FromAfterTo,
}
use TimeRangeError::*;

impl TimeRange {
    pub fn new(from: SystemTime, to: SystemTime) -> Result<Self, TimeRangeError> {
        if from > to {
            Err(FromAfterTo)
        } else {
            Ok(Self { from, to })
        }
    }

    pub fn empty(&self) -> bool {
        self.from == self.to
    }

    pub fn make_timelist(self, period: &Duration, offset: &Duration) -> TimeList {
        TimeList::from((self, period.clone(), offset.clone()))
    }
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}--{}",
            display_systime(self.from),
            display_systime(self.to)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        let five_seconds = Duration::new(5, 0);
        let now = SystemTime::now();
        assert!(
            TimeRange::new(now - five_seconds * 4, now - five_seconds * 3).unwrap()
                < TimeRange::new(now - five_seconds, now).unwrap()
        );
    }

    #[test]
    fn check_empty() {
        let to = SystemTime::now();
        let range = TimeRange::new(to, to).unwrap();
        assert!(range.empty());
    }
}
