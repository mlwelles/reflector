//! A simple representation of a series of times.
//!
//! These are generally used to glue between [TimeRange] and
//! [CaptureList].
//!
//! Some open questions:
//!   - do we need a uniqueness constraint?
//!   - do we need to guarantee order?
//! None of that currently present.

#![allow(unused_imports)]
use super::time_util::*;
use super::{CaptureList, TimeRange};
use chrono::NaiveDateTime;
use std::fmt;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TimeList {
    list: Vec<SystemTime>,
}

impl TimeList {
    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn push(&mut self, time: SystemTime) {
        self.list.push(time)
    }
}

impl Iterator for TimeList {
    type Item = SystemTime;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl From<Vec<SystemTime>> for TimeList {
    fn from(list: Vec<SystemTime>) -> Self {
        Self { list }
    }
}

impl From<SystemTime> for TimeList {
    fn from(start: SystemTime) -> Self {
        let list = vec![start];
        Self { list }
    }
}

impl From<NaiveDateTime> for TimeList {
    fn from(start: NaiveDateTime) -> Self {
        Self::from(systime_from_naive(start))
    }
}

/// Convert from a range, a period and an offset.
impl From<(TimeRange, Duration, Duration)> for TimeList {
    fn from(input: (TimeRange, Duration, Duration)) -> Self {
        let range = input.0;
        let period = input.1;
        let offset = input.2;

        // start is the leading edge of the range, minus the offset (for now)
        let from = naive_from_systime(range.from) - offset;

        // make a duration between start and midnight
        let since_midnight = naive_since_midnight(&from);

        // divide that by period to get x
        let x = since_midnight / (period.as_secs() as u32);

        // our initial time is (x * period) + offset
        let start = naive_trunc_midnight(&from) + (period * x.as_secs() as u32) + offset;

        // shift back from Chrono to std
        let start = systime_from_naive(start);

        // stack on one per period, accumulating the period
        // until our accum is larger than our end time
        let mut l = Self::from(start);
        let mut tt = start.clone();
        while tt < range.to {
            l.push(tt);
            tt += period;
        }
        l
    }
}

/// Convert from a range and a period, which uses a zero offset.
impl From<(TimeRange, Duration)> for TimeList {
    fn from(input: (TimeRange, Duration)) -> Self {
        Self::from((input.0, input.1, Duration::ZERO))
    }
}

impl fmt::Display for TimeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tt = self
            .clone()
            .map(|t| display_systime(t))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", tt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let to = SystemTime::now();
        let mut l = TimeList::from(to);
        assert_eq!(1, l.len());
        l.push(SystemTime::now());
        assert_eq!(2, l.len());
    }

    #[test]
    fn from_range() {
        let to = SystemTime::now();
        let minutes: usize = 5;
        let frm = to
            .checked_sub(Duration::from_secs((60 * minutes as u64) + 20))
            .unwrap();
        let range = TimeRange::new(frm, to).unwrap();
        let period = Duration::from_secs(60);
        let offset = Duration::ZERO;
        let l = TimeList::from((range.clone(), period, offset));
        assert_eq!(minutes, l.len(), "timelist {l} from range {range}");
    }

    #[test]
    fn simple_range() {}
}
