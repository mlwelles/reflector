pub mod error;
pub use error::PathMakerError::{self, *};
pub mod new;
pub use new::new;
pub mod sdo;
pub use sdo::SDO;
pub mod goes_r;
pub use goes_r::GoesR;
pub mod identity;
pub use identity::Identity;

use crate::time_range::TimeRange;
use chrono::{DateTime, Utc};
use std::ffi::{OsStr, OsString};
use std::path;
use std::time::{Duration, SystemTime};

pub trait PathMaker {
    fn time_to_filename(&self, time: &DateTime<Utc>) -> OsString;
    fn filename_to_time(&self, filename: &OsStr) -> Result<DateTime<Utc>, PathMakerError>;

    fn filename_to_systime(&self, filename: &OsStr) -> Result<SystemTime, PathMakerError> {
        match self.filename_to_time(filename) {
            Ok(time) => {
                let since: u64 = time.timestamp().try_into().unwrap();
                let dur = Duration::new(since, 0);
                let st = SystemTime::now();
                match st.checked_sub(dur) {
                    Some(st) => Ok(st),
                    None => Err(ImpossibleTimestamp(since)),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn time_to_path(&self, time: &DateTime<Utc>) -> path::PathBuf {
        path::PathBuf::from(self.time_to_filename(time))
    }

    fn from_range(
        &self,
        _range: &TimeRange,
        _period: &Duration,
    ) -> Result<Vec<path::PathBuf>, PathMakerError> {
        Err(Unimplemented)
    }
}
