//! Shadow upstream data to local storage.

use crate::pathmaker;
use crate::remote::{
    from_url as remote_from_url, GetError, Gotten, PingError, RCFactoryError, RemoteClient,
};
use crate::{
    display_systime, flatten_filename, time_range, Capture, CaptureError, CaptureList,
    CaptureMissing, FileList, FileStore, PathMaker, PathMakerError, SourceConfig, StoreError,
    StoreGetError, TimeList, TimeRange,
};
use log::{info, warn};
use std::fmt;
use std::path::PathBuf;
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
#[derive(Debug, PartialEq, Eq)]
pub enum MirrorStatus {
    Unimplemented,
    Full(time::SystemTime),
    Partial(time::SystemTime),
    Empty(time::SystemTime),
}

impl fmt::Display for MirrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MirrorStatus::Unimplemented => "status not implemented".to_string(),
                MirrorStatus::Full(t) =>
                    format!("mirror latest {}, fully reflected", display_systime(t)),
                MirrorStatus::Empty(t) =>
                    format!("mirror has no captures since {}", display_systime(t)),
                MirrorStatus::Partial(t) => format!(
                    "mirror latest {}, only partially reflected",
                    display_systime(t)
                ),
            }
        )
    }
}

#[derive(Debug)]
pub enum StatusError {
    Unimplemented,
    CannotPing(PingError),
    RangeError(time_range::TimeRangeError),
    CaptureError(CaptureError),
    Inconsistent, // shouldn't normally happen
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
        // FIXME: requires network to be up
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

        let flatten = matches!(cfg.flatten, Some(true));
        let seed_past_midnight = Duration::new(cfg.offset.unwrap_or(0), 0);
        let loop_period = Duration::new(cfg.loop_period.unwrap_or(24 * 60 * 60), 0);

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

        // check the store for files within our range,
        // and set the status accordingly
        let cc = self.loop_captures();
        info!("captures {cc} len {}", cc.len());
        match cc.full_ratio() {
            Err(e) => Err(StatusError::CaptureError(e)),
            Ok(f) => {
                if !cc.has_captures() {
                    Ok(MirrorStatus::Empty(cc.missing.front().unwrap().time))
                } else if let Some(latest) = cc.last() {
                    let lt = latest.time;
                    if f < 1.0 {
                        Ok(MirrorStatus::Partial(lt))
                    } else {
                        Ok(MirrorStatus::Full(lt))
                    }
                } else {
                    Err(StatusError::Inconsistent)
                }
            }
        }
    }

    pub fn ping(&mut self) -> Result<time::Duration, PingError> {
        self.remote_client.ping()
    }

    // FIXME: Resource should be a type
    pub fn url(&self, rsrc: &str) -> Result<Url, GetError> {
        self.remote_client.url(rsrc)
    }

    pub fn exists(&self, rsrc: &str) -> Result<bool, GetError> {
        self.remote_client.exists(rsrc)
    }

    pub fn get(&mut self, rsrc: &str, path: PathBuf) -> Result<Gotten, GetError> {
        self.remote_client.get(rsrc, path)
    }

    pub fn get_missing(&mut self, m: &CaptureMissing) -> Result<Gotten, GetError> {
        let p = self.local.join(&m.path);
        self.remote_client.get(&m.resource, p)
    }

    pub fn timelist(&self, range: &TimeRange) -> TimeList {
        range
            .clone()
            .make_timelist(&self.period, &self.seed_past_midnight)
    }

    fn filelist(&self, times: &TimeList) -> FileList {
        let mut files = FileList::empty();
        for t in times.clone() {
            let f = self.pathmaker.systime_to_filename(&t);
            if self.flatten {
                files.push(flatten_filename(&f));
            } else {
                files.push(f);
            }
        }
        files
    }

    pub fn range_to_filelist(&self, range: &TimeRange) -> FileList {
        let range: TimeRange = range.clone();
        let times = self.timelist(&range);
        self.filelist(&times)
    }

    // based on a TimeList, construct a capturelist reflecting which
    // files we've already downloaded
    pub fn captures_in_timelist(&self, times: TimeList) -> CaptureList {
        let mut c = CaptureList::empty();
        for time in times {
            let f = self.pathmaker.systime_to_filename(&time);
            let path = match self.flatten {
                true => flatten_filename(&f),
                false => f.clone(),
            };
            let path = PathBuf::from(path);
            match self.local.get(&path) {
                Ok(cap) => c.push(cap),
                Err(e) => {
                    let cap = CaptureMissing::new(time, path, f.to_str().unwrap_or(""));
                    c.push_missing(cap);
                    match e {
                        StoreGetError::NoSuchFile(_) => (),
                        _ => eprintln!("unexpected error: {:?}", e),
                    }
                }
            }
        }
        c
    }

    pub fn captures_in_range(&self, range: &TimeRange) -> CaptureList {
        let times = self.timelist(&range);
        self.captures_in_timelist(times)
    }

    pub fn loop_range(&self) -> TimeRange {
        TimeRange::from_now_to(&self.loop_period).unwrap()
    }

    pub fn loop_captures(&self) -> CaptureList {
        self.captures_in_range(&self.loop_range())
    }

    pub fn fill_captures(&mut self, c: CaptureList) -> Result<CaptureList, GetError> {
        let mut new = CaptureList::empty();
        // let mut errs = vec![GetError];
        new.list = c.list;
        let mut missing = c.missing.clone();
        let mut err: Option<GetError> = None;
        while let Some(m) = missing.pop_back() {
            info!("attempting to fill missing {}", m.resource);
            match self.get_missing(&m) {
                Ok(g) => {
                    info!("success with {}", g.source);
                    let c = Capture::from((g, m.time));
                    new.push(c)
                }
                Err(e) => {
                    warn!("error: {:?}", e);
                    new.push_missing(m);
                    if err.is_none() {
                        err = Some(e);
                    }
                }
            }
        }
        match err.is_some() {
            true => Err(GetError::IncompleteFill(
                Box::new(err.unwrap()),
                new.missing,
            )),
            false => Ok(new),
        }
    }

    pub fn fill_loop(&mut self) -> Result<CaptureList, GetError> {
        self.fill_captures(self.loop_captures())
    }

    pub fn latest_capture(&self) -> Option<Capture> {
        self.loop_captures().last()
    }
}

impl fmt::Display for Mirror {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "<mirror {}>", self.name)
    }
}

impl TryFrom<SourceConfig> for Mirror {
    type Error = MirrorError;

    fn try_from(src: SourceConfig) -> Result<Self, Self::Error> {
        Mirror::new(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time_util::*;
    use std::fs::{DirBuilder, File};
    use std::path::Path;

    fn mock_src_config() -> SourceConfig {
        let fc = "/tmp/mock_mirror_store";
        let fcp = Path::new(fc);

        // ensure our store dir exists
        if !fcp.is_dir() {
            DirBuilder::new().create(fc).unwrap();
        }

        // setup a mock capture, at midnight
        let dt = datetime_from_systime(SystemTime::now());
        let ts: String = format!("{}", dt.format("%Y-%m-%dT00:00:00+00:00"));
        eprintln!("creating file in store '{}'...", fcp.join(&ts).display());
        let _file = File::create(fcp.join(&ts));

        SourceConfig {
            name: "mock mirror source".to_string(),
            abbrev: "mock".to_string(),
            remote: "http://sopa.coo/mock".to_string(),
            local: fc.to_string(),
            pathmaker: "identity".to_string(),
            flatten: None,
            period: 60 * 60, // once per hour
            offset: None,
            loop_period: Some(60 * 60 * 24),
        }
    }

    fn mock_mirror() -> Mirror {
        Mirror::new(mock_src_config()).unwrap()
    }

    #[test]
    fn latest_capture() {
        let m = mock_mirror();
        assert!(m.latest_capture().is_some());
    }

    #[test]
    fn status() {
        let mut m = mock_mirror();
        let s = m.status().unwrap();
        assert!(matches!(s, MirrorStatus::Partial(_)));
        let ss: String = format!("{s}");
        assert!(ss.contains("only partially reflected"));
    }
}
