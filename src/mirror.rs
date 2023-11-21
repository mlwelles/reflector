//! Shadow upstream data to local storage.

use crate::pathmaker;
use crate::remote::{from_url as remote_from_url, PingError, RCFactoryError, RemoteClient};
use crate::store::{FileList, FileStore};
use crate::time_range;
use crate::{Capture, CaptureList, PathMaker, PathMakerError, SourceConfig, StoreError, TimeRange};
use std::fmt;
use std::time::{self, Duration, SystemTime};
use url::Url;

#[derive(Debug)]
pub enum MirrorError {
    InvalidURL(url::ParseError),
    InvalidStore(StoreError),
    InvalidPathMaker(PathMakerError),
    InvalidRemote(RCFactoryError),
}
use MirrorError::*;

/// the current status of the mirror, specifically related to how
/// completely upstream data is shadowed
#[derive(Debug)]
pub enum MirrorStatus {
    Unimplemented,
    Full(time::SystemTime),
    Partial(time::SystemTime),
    Empty,
}

impl fmt::Display for MirrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            MirrorStatus::Unimplemented => "status not implemented".to_string(),
            MirrorStatus::Full(t) => format!("mirror latest {:?}, fully reflected", t),
            MirrorStatus::Partial(t) => format!("mirror latest {:?}, only partially reflected", t),
            MirrorStatus::Empty => "mirror is empty, unpulled".to_string(),
        };
        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub enum StatusError {
    Unimplemented,
    CannotPing(PingError),
    RangeError(time_range::TimeRangeError),
}

/// a remote site, kept in sync with a local file store
pub struct Mirror {
    pub name: String,
    pub period: time::Duration,
    pub seed_past_midnight: time::Duration,
    pub loop_period: time::Duration,
    pub local: FileStore,
    pub remote: Url,
    remote_client: Box<dyn RemoteClient>,
    pub flatten: bool,
    pub pathmaker: Box<dyn PathMaker>,
}

impl Mirror {
    pub fn new(cfg: SourceConfig) -> Result<Mirror, MirrorError> {
        let period = time::Duration::from_secs(cfg.period);
        let pathmaker = pathmaker::new(&cfg.pathmaker);
        if let Err(e) = pathmaker {
            return Err(InvalidPathMaker(e));
        }
        let pathmaker = pathmaker.unwrap();

        let remote = Url::parse(&cfg.remote);
        if let Err(e) = remote {
            return Err(InvalidURL(e));
        }
        let remote = remote.unwrap();
        let remote_client = remote_from_url(&remote);
        if let Err(e) = remote_client {
            return Err(InvalidRemote(e));
        }
        let remote_client = remote_client.unwrap();

        let p2 = pathmaker::new(&cfg.pathmaker).unwrap();
        let local = FileStore::new(&cfg.local, p2);
        if let Err(e) = local {
            return Err(InvalidStore(e));
        }
        let local = local.unwrap();

        let mut flatten = false;
        if cfg.flatten == Some(true) {
            flatten = true;
        }
        let seed_past_midnight = Duration::new(cfg.seed_past_midnight.unwrap_or(0), 0);
        // FIXME: fixed at 24 hours
        let loop_period = Duration::new(24 * 60 * 60, 0);

        let m = Mirror {
            name: cfg.name,
            period,
            seed_past_midnight,
            local,
            remote,
            remote_client,
            pathmaker,
            flatten,
            loop_period,
        };
        Ok(m)
    }

    pub fn period_timerange(&self) -> Result<TimeRange, time_range::TimeRangeError> {
        let now = SystemTime::now();
        let then = now - self.period;
        TimeRange::new(then, now)
    }

    pub fn status(&mut self) -> Result<MirrorStatus, StatusError> {
        if let Err(e) = self.remote_client.ping() {
            return Err(StatusError::CannotPing(e));
        }

        let range = match TimeRange::from_now_to(&self.loop_period) {
            Ok(r) => r,
            Err(e) => return Err(StatusError::RangeError(e)),
        };

        let tt = range.make_timelist(&self.period, &self.seed_past_midnight);
        let ff = self.pathmaker.timelist_to_filelist(&tt);

        // check the store for files within our range,
        // and set the status accordingly
        let cc = self.local.captures_in_list(ff);

        Ok(MirrorStatus::Unimplemented)
    }

    pub fn ping(&mut self) -> Result<time::Duration, PingError> {
        self.remote_client.ping()
    }

    pub fn range_to_filelist(&self, range: &TimeRange) -> FileList {
        let mut files = FileList::empty();
        let range: TimeRange = range.clone();
        let times = range.make_timelist(&self.period, &self.seed_past_midnight);
        for t in times {
            let f = self.pathmaker.systime_to_filename(&t);
            files.push(&f);
        }
        files
    }

    pub fn captures_in_range(&self, range: &TimeRange) -> CaptureList {
        let items = self.range_to_filelist(range);
        self.local.captures_in_list(items)
    }

    pub fn latest_capture(&self) -> Option<Capture> {
        None
    }
}

impl fmt::Display for Mirror {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "<mirror {})", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::DirBuilder;
    use std::path::Path;

    fn mock_src_config() -> SourceConfig {
        let fc = "/tmp/mock_mirror_store";

        // ensure our store dir exists
        if !Path::new(fc).is_dir() {
            DirBuilder::new().create(fc).unwrap();
        }

        SourceConfig {
            name: "mock mirror source".to_string(),
            remote: "http://sopa.coo/mock".to_string(),
            local: fc.to_string(),
            pathmaker: "identity".to_string(),
            flatten: None,
            period: 60,
            seed_past_midnight: None,
        }
    }

    fn mock_mirror() -> Mirror {
        Mirror::new(mock_src_config()).unwrap()
    }

    #[test]
    fn check_mock() {
        let m = mock_mirror();
        assert_eq!(None, m.latest_capture());
    }

    #[test]
    fn latest_capture() {
        let m = mock_mirror();
        let c = m.latest_capture();
        assert_eq!(None, c);
    }

    #[test]
    fn status() {
        let mut m = mock_mirror();
        let s = m.status().unwrap();
        assert_eq!("status: mumble", format!("status: {s}"));
    }
}
