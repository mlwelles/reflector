pub mod error;
pub use error::PathMakerError::{self, *};
pub mod new;
pub use new::new;
pub mod sdo;
pub use sdo::Sdo;
pub mod goes_r;
pub use goes_r::GoesR;
pub mod identity;
pub use identity::Identity;
pub mod time;
pub use time::make_utc;

use super::{FileList, TimeList};
use chrono::{DateTime, Utc};
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
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

    fn systime_to_filename(&self, time: &SystemTime) -> OsString {
        let utc: DateTime<Utc> = (*time).into();
        self.time_to_filename(&utc)
    }

    fn systime_to_path(&self, time: &SystemTime) -> PathBuf {
        PathBuf::from(self.systime_to_filename(time))
    }

    fn time_to_path(&self, time: &DateTime<Utc>) -> PathBuf {
        PathBuf::from(self.time_to_filename(time))
    }

    // FIXME: why?  whuh?
    fn time_to_string(&self, time: &DateTime<Utc>) -> String {
        time.to_rfc3339()
    }

    fn timelist_to_filelist(&self, tt: &TimeList) -> FileList {
        let mut ff = FileList::empty();
        let tt = tt.clone();
        for t in tt {
            let f = self.systime_to_filename(&t);
            ff.push_osstr(&f);
        }
        ff
    }
}
