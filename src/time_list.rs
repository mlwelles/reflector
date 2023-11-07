use super::TimeRange;
use chrono::{NaiveDateTime, Timelike};
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

impl From<NaiveDateTime> for TimeList {
    fn from(start: NaiveDateTime) -> Self {
        Self::from(systime_from_naive(start))
    }
}

fn naive_from_systime(st: SystemTime) -> NaiveDateTime {
    let epoch = st.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let es = i64::try_from(epoch.as_secs()).unwrap();
    NaiveDateTime::from_timestamp_opt(es, 0).unwrap()
}

fn systime_from_naive(ndt: NaiveDateTime) -> SystemTime {
    let d = Duration::new(ndt.timestamp() as u64, 0);
    SystemTime::UNIX_EPOCH + d
}

fn naive_trunc_midnight(inb: &NaiveDateTime) -> NaiveDateTime {
    let t = inb.time();
    let d = Duration::new(t.num_seconds_from_midnight() as u64, 0);
    let inb = inb.clone();
    inb - d
}

fn naive_since_midnight(inb: &NaiveDateTime) -> Duration {
    Duration::new(inb.time().num_seconds_from_midnight() as u64, 0)
}

impl From<(TimeRange, Duration, Duration)> for TimeList {
    fn from(input: (TimeRange, Duration, Duration)) -> Self {
        let range = input.0;
        let period = input.1;
        let offset = input.2; // FIXME: not implmneted yet

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
        let frm = to.checked_sub(Duration::from_secs(60 * 5 + 20)).unwrap();
        let range = TimeRange::new(frm, to).unwrap();
        let period = Duration::from_secs(60);
        let offset = Duration::new(0, 0);
        let l = TimeList::from((range, period, offset));
        eprintln!("got timelist {:?}", l);
        assert_eq!(5, l.len(), "length");
    }
}
