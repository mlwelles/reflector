use super::TimeRange;
use std::time::{Duration, SystemTime};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TimeList {
    list: Vec<SystemTime>,
}

impl TimeList {
    pub fn len(&self) -> usize {
        self.list.len()
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

impl From<(TimeRange, Duration, Duration)> for TimeList {
    fn from(input: (TimeRange, Duration, Duration)) -> Self {
        // start is the leading edge of the range, minus the offset (for now)
        let start = input.0.from - input.1.to_owned();

        // make a duration between start and midnight
        // divide that by period to get x

        // our initial time is (x * period) + offset
        // stack on one per period, accumulating the period
        // until our accum is larger than our end time
        let l = TimeList::from(start);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_range() {
        let to = SystemTime::now();
        let frm = to.checked_sub(Duration::from_secs(300)).unwrap();
        let range = TimeRange::new(frm, to).unwrap();
        let period = Duration::from_secs(60);
        let offset = Duration::new(0, 0);
        let l = TimeList::from((range, period, offset));
        assert_eq!(5, l.len(), "length");
    }
}
