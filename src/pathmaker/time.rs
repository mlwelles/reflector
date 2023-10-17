use super::*;
use chrono::offset::LocalResult;
use chrono::{DateTime, TimeZone, Utc};

pub fn make_utc(
    year: i32,
    mon: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
) -> Result<DateTime<Utc>, PathMakerError> {
    match Utc.with_ymd_and_hms(year, mon, day, hour, min, sec) {
        LocalResult::Single(t) => Ok(t),
        LocalResult::Ambiguous(..) => Err(AmbiguousTimeError(year, mon, day, hour, min, sec)),
        LocalResult::None => Err(NoTimeError(year, mon, day, hour, min, sec)),
    }
}
