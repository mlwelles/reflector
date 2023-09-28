pub mod error;
mod util;
pub use error::PathMakerError::{self, *};
pub mod new;
pub use new::new;
pub mod sdo;
pub use sdo::SDO;

use chrono::{DateTime, Utc};
use std::ffi::OsStr;
use std::path;
use std::time::{Duration, SystemTime};

pub trait PathMaker {
    fn time_to_filename(&self, time: &DateTime<Utc>) -> String;
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
}
