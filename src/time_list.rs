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

impl From<(TimeRange, Duration, Duration)> for TimeList {
    fn from(input: (TimeRange, Duration, Duration)) -> Self {
        let range = input.0;
        let period = input.1;
        let _offset = input.2; // FIXME: not implmneted yet

        // start is the leading edge of the range, minus the offset (for now)
        let mut tt = range.from - period.to_owned();

        // make a duration between start and midnight
        // divide that by period to get x

        // our initial time is (x * period) + offset
        // stack on one per period, accumulating the period
        // until our accum is larger than our end time
        let mut l = TimeList::from(tt.clone());
        while let Some(next) = tt.checked_add(period) {
            if next > range.to {
                return l;
            }
            l.push(next);
            tt = next;
        }
        l
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
        let frm = to.checked_sub(Duration::from_secs(300)).unwrap();
        let range = TimeRange::new(frm, to).unwrap();
        let period = Duration::from_secs(60);
        let offset = Duration::new(0, 0);
        let l = TimeList::from((range, period, offset));
        assert_eq!(5, l.len(), "length");
    }
}
