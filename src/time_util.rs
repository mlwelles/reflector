use chrono::{NaiveDateTime, Timelike};
use std::time::{Duration, SystemTime};

pub fn naive_from_systime(st: SystemTime) -> NaiveDateTime {
    let epoch = st.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let es = i64::try_from(epoch.as_secs()).unwrap();
    NaiveDateTime::from_timestamp_opt(es, 0).unwrap()
}

pub fn systime_from_naive(ndt: NaiveDateTime) -> SystemTime {
    let d = Duration::new(ndt.timestamp() as u64, 0);
    SystemTime::UNIX_EPOCH + d
}

pub fn display_systime(st: SystemTime) -> String {
    naive_from_systime(st)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

pub fn naive_trunc_midnight(inb: &NaiveDateTime) -> NaiveDateTime {
    let t = inb.time();
    let d = Duration::new(t.num_seconds_from_midnight() as u64, 0);
    let inb = inb.clone();
    inb - d
}

pub fn naive_since_midnight(inb: &NaiveDateTime) -> Duration {
    Duration::new(inb.time().num_seconds_from_midnight() as u64, 0)
}
