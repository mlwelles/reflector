use super::time_util::*;
use super::TimeList;
use chrono::NaiveDateTime;
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
    CannotRemovePeriod(SystemTime, Duration),
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

    pub fn from_now_to(dur: &Duration) -> Result<Self, TimeRangeError> {
        let to = SystemTime::now();
        let from = match to.checked_sub(dur.clone()) {
            Some(f) => f,
            None => return Err(TimeRangeError::CannotRemovePeriod(to, dur.clone())),
        };
        Self::new(from, to)
    }

    pub fn empty(&self) -> bool {
        self.from == self.to
    }

    pub fn equal_by_seconds(&self, compare: &Self) -> bool {
        (systime_as_secs(&self.from) == systime_as_secs(&compare.from))
            && (systime_as_secs(&self.to) == systime_as_secs(&compare.to))
    }

    pub fn empty_by_secs(&self) -> bool {
        systime_as_secs(&self.from) == systime_as_secs(&self.to)
    }

    pub fn make_timelist(self, period: &Duration, offset: &Duration) -> TimeList {
        TimeList::from((self, period.clone(), offset.clone()))
    }
}

impl From<SystemTime> for TimeRange {
    fn from(start: SystemTime) -> Self {
        Self {
            from: start,
            to: start,
        }
    }
}

impl From<(SystemTime, SystemTime)> for TimeRange {
    fn from(input: (SystemTime, SystemTime)) -> Self {
        Self {
            from: input.0,
            to: input.1,
        }
    }
}

impl From<NaiveDateTime> for TimeRange {
    fn from(start: NaiveDateTime) -> Self {
        Self::from(systime_from_naive(start))
    }
}

impl From<(NaiveDateTime, NaiveDateTime)> for TimeRange {
    fn from(input: (NaiveDateTime, NaiveDateTime)) -> Self {
        Self::from((systime_from_naive(input.0), systime_from_naive(input.1)))
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
    fn from_single() {
        let now = SystemTime::now();
        let r = TimeRange::from(now);
        assert_eq!(TimeRange::new(now, now).unwrap(), r);
        assert!(r.empty())
    }

    #[test]
    fn from_now_to() {
        let dur = Duration::from_secs(60);
        let to = SystemTime::now();
        let from = to.checked_sub(dur).unwrap();
        let got = TimeRange::from_now_to(&dur).unwrap();
        assert!(got.equal_by_seconds(&TimeRange::new(from, to).unwrap()));
    }

    #[test]
    fn from_tuple() {
        let five_seconds = Duration::new(5, 0);
        let now = SystemTime::now();
        let then = now - five_seconds;
        let r = TimeRange::from((then, now));
        assert_eq!(TimeRange::new(then, now).unwrap(), r);
        assert!(!r.empty());
    }

    #[test]
    fn check_empty() {
        let to = SystemTime::now();
        let range = TimeRange::new(to, to).unwrap();
        assert!(range.empty());
    }
}
