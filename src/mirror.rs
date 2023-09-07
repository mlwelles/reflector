use crate::capture::{Capture, CaptureList};
use crate::config::SourceConfig;
use crate::store::FileStore;
use crate::{PathMaker, PathMakerError, StoreError, TimeRange};
use std::time;
use url::Url;

#[derive(Debug)]
pub struct Mirror {
    pub name: String,
    pub period: time::Duration,
    // TODO: generalize this as a trait
    pub local: FileStore,
    pub remote: Url,
    pub flatten: bool,
    pub pathmaker: PathMaker,
}

#[derive(Debug)]
pub enum FactoryError {
    InvalidURL(url::ParseError),
    InvalidStore(StoreError),
    InvalidPathMaker(PathMakerError),
}
use FactoryError::*;

impl Mirror {
    pub fn new(cfg: SourceConfig) -> Result<Mirror, FactoryError> {
        let period = time::Duration::from_secs(cfg.period);
        let pathmaker = PathMaker::new(&cfg.pathmaker);
        if let Err(e) = pathmaker {
            return Err(InvalidPathMaker(e));
        }
        let pathmaker = pathmaker.unwrap();

        let remote = Url::parse(&cfg.remote);
        if let Err(e) = remote {
            return Err(InvalidURL(e));
        }
        let remote = remote.unwrap();

        let local = FileStore::new(&cfg.local, &pathmaker);
        if let Err(e) = local {
            return Err(InvalidStore(e));
        }
        let local = local.unwrap();

        let mut flatten = false;
        if cfg.flatten == Some(true) {
            flatten = true;
        }

        let m = Mirror {
            name: cfg.name,
            period,
            local,
            remote,
            pathmaker,
            flatten,
        };
        Ok(m)
    }

    // pub fn ping(&self) -> Result<(), T> {}

    pub fn captures_in_range(&self, range: &TimeRange) -> CaptureList {
        match self.local.from_range(range) {
            Ok(x) => x,
            Err(_) => CaptureList::empty(),
        }
    }

    pub fn latest_capture(&self, range: TimeRange) -> Option<Capture> {
        None
    }
}
