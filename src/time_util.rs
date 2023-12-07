//! Handy functions for working with the time formats used in the
//! reflector crate.

use chrono::{NaiveDateTime, Timelike};
use std::time::{Duration, SystemTime};

/// convert a SystemTime into seconds since midnight, or else u64::MAX
/// ```
/// use reflector::time_util::*;
/// use std::time::SystemTime;
///
/// assert_eq!(0, systime_as_secs(&SystemTime::UNIX_EPOCH));
/// ```
pub fn systime_as_secs(s: &SystemTime) -> u64 {
    match s.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => u64::MAX,
    }
}

/// round a systemtime to seconds, tossing out nanoseconds
/// ```
/// use reflector::time_util::*;
/// use std::time::SystemTime;
///
/// let now = SystemTime::now();
/// let d = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
/// let rounded = systime_round_to_s(&now);
/// if d.subsec_millis() > 0  {
///   assert_ne!(now, rounded);
/// } else {
///   assert_eq!(now, rounded);
/// }
/// ```
pub fn systime_round_to_s(s: &SystemTime) -> SystemTime {
    let e = SystemTime::UNIX_EPOCH;
    let d = s.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let f = Duration::from_secs(d.as_secs());
    match e.checked_add(f) {
        Some(r) => r,
        None => panic!("unable to calculate time"),
    }
}

/// ```
/// use reflector::time_util::*;
/// use std::time::SystemTime;
///
/// let now = SystemTime::now();
/// let rounded = systime_round_to_s(&now);
/// assert_ne!(now, rounded);
/// assert_eq!(SystemTime::UNIX_EPOCH, systime_round_to_min(&SystemTime::UNIX_EPOCH));
/// ```
pub fn systime_round_to_min(s: &SystemTime) -> SystemTime {
    let e = SystemTime::UNIX_EPOCH;
    let d = s.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let secs = (d.as_secs() / 60) * 60;
    let f = Duration::from_secs(secs);
    match e.checked_add(f) {
        Some(r) => r,
        None => panic!("unable to calculate time"),
    }
}

/// convert a SystemTime into a NaiveDateTime, in a panicky way
/// ```
/// use reflector::time_util::*;
/// use std::time::SystemTime;
///
/// let nt = naive_from_systime(SystemTime::now());
/// ```
pub fn naive_from_systime(st: SystemTime) -> NaiveDateTime {
    let es = i64::try_from(systime_as_secs(&st)).unwrap();
    NaiveDateTime::from_timestamp_opt(es, 0).unwrap()
}

/// convert a NaiveDateTime into a SystemTime
/// ```
/// use reflector::time_util::*;
/// use chrono::{NaiveDateTime, NaiveDate};
/// use std::time::SystemTime;
///
/// let n = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
/// assert_eq!(SystemTime::UNIX_EPOCH, systime_from_naive(n))
/// ```
pub fn systime_from_naive(ndt: NaiveDateTime) -> SystemTime {
    let d = Duration::new(ndt.timestamp() as u64, 0);
    SystemTime::UNIX_EPOCH + d
}

/// show the a given SystemTime ignoring resolutions below seconds
/// ```
/// use reflector::time_util::*;
/// use std::time::SystemTime;
///
/// let now = SystemTime::UNIX_EPOCH;
/// assert_eq!("1970-01-01 00:00:00", display_systime(now))
/// ```
pub fn display_systime(st: SystemTime) -> String {
    naive_from_systime(st)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

/// how long since midnight happened?
/// ```
/// use reflector::time_util::*;
/// use chrono::{NaiveDateTime, Datelike};
///
/// assert_eq!(0, naive_since_midnight(&NaiveDateTime::UNIX_EPOCH).as_secs());
/// ```
pub fn naive_since_midnight(inb: &NaiveDateTime) -> Duration {
    Duration::new(inb.time().num_seconds_from_midnight() as u64, 0)
}

/// return a new time representing the midnight (0:00) of the current
/// day
/// ```
/// use reflector::time_util::*;
/// use chrono::{NaiveDateTime, NaiveDate, Datelike};
/// use std::time::SystemTime;
///
/// let n = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(1, 2, 3).unwrap();
/// assert_eq!(NaiveDateTime::UNIX_EPOCH, naive_trunc_midnight(&n));
/// ```
pub fn naive_trunc_midnight(inb: &NaiveDateTime) -> NaiveDateTime {
    inb.clone() - naive_since_midnight(&inb)
}
