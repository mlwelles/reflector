//! Handy functions for working with the time formats used in the
//! reflector crate.

use chrono::{DateTime, Timelike, Utc};
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

/// convert a SystemTime into a DateTime
pub fn datetime_from_systime(st: SystemTime) -> DateTime<Utc> {
    let es = i64::try_from(systime_as_secs(&st)).unwrap();
    DateTime::from_timestamp(es, 0).unwrap()
}

/// convert a DateTime into a SystemTime
pub fn systime_from_datetime(dt: DateTime<Utc>) -> SystemTime {
    let d = Duration::new(dt.timestamp() as u64, 0);
    SystemTime::UNIX_EPOCH + d
}

/// show the a given SystemTime ignoring resolutions below seconds
/// ```
/// use reflector::time_util::*;
/// use std::time::SystemTime;
///
/// let epch = SystemTime::UNIX_EPOCH;
/// assert_eq!("1970-01-01 00:00:00", display_systime(&epch))
/// ```
pub fn display_systime(st: &SystemTime) -> String {
    datetime_from_systime(st.clone())
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

// so far from general
fn pluralize(u: u64) -> String {
    match u {
        1 => "".to_string(),
        _ => "s".to_string(),
    }
}

/// show a duration, assuming we don't really care about nanos
/// ```
/// use reflector::time_util::*;
/// use std::time::Duration;
///
/// let d = Duration::new(62, 20);
/// assert_eq!("1 minute, 2 seconds", display_duration(&d));
/// ```
pub fn display_duration(d: &Duration) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = MINUTE * 60;
    const DAY: u64 = HOUR * 24;
    let mut s = vec!["".to_string()];
    let mut v = d.as_secs();
    if v > DAY {
        let days = v / DAY;
        s.push(format!("{days} day{}", pluralize(days)));
        v = v % DAY;
    }
    if v > HOUR {
        let hours = v / HOUR;
        s.push(format!("{hours} hour{}", pluralize(hours)));
        v = v % HOUR;
    }
    if v > MINUTE {
        let min = v / MINUTE;
        s.push(format!("{min} minute{}", pluralize(min)));
        v = v % MINUTE;
    }
    s.push(format!("{v} seconds"));

    s[1..].join(", ")
}

/// how long since midnight happened?
/// ```
/// use reflector::time_util::*;
/// use chrono::{Datelike, DateTime, Utc};
///
/// assert_eq!(0, datetime_since_midnight(&DateTime::UNIX_EPOCH).as_secs());
/// ```
pub fn datetime_since_midnight(inb: &DateTime<Utc>) -> Duration {
    Duration::new(inb.time().num_seconds_from_midnight() as u64, 0)
}

/// return a new time representing the midnight (0:00) of the current
/// day
/// ```
/// use reflector::time_util::*;
/// use std::time::{Duration,SystemTime};
///
/// let epoch = SystemTime::UNIX_EPOCH;
/// let n = datetime_from_systime(epoch + Duration::from_secs(60 * 63 + 5));
/// assert_eq!(epoch, datetime_trunc_midnight(&n).into());
/// ```
pub fn datetime_trunc_midnight(inb: &DateTime<Utc>) -> DateTime<Utc> {
    *inb - datetime_since_midnight(inb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    // use chrono::prelude::*;

    #[test]
    fn now() {
        let s = SystemTime::now();
        // let u = Utc::now();
        // assert_eq!(systime_as_secs(&s), systime_as_secs(&systime_from_naive(u)));

        let output = Command::new("date")
            .arg("+%Y-%m-%d %H:%M:%S")
            .env("TZ", "UTC")
            .output()
            .unwrap();
        let mut expect = String::from_utf8(output.stdout).unwrap();
        expect.pop(); // remove trailing newline
        assert_eq!(
            expect,
            display_systime(&s),
            "expected vs actual current time"
        );
    }
}
