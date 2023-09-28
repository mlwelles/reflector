use crate::capture::{Capture, CaptureList};
use crate::config::SourceConfig;
use crate::pathmaker;
use crate::remote::{
    from_url as remote_from_url, ConnectError, PingError, RCFactoryError, RemoteClient,
};
use crate::store::{FileList, FileStore};
use crate::{PathMaker, PathMakerError, StoreError, TimeRange};
use std::time;
use url::Url;

pub struct Mirror<'a> {
    pub name: String,
    pub period: time::Duration,
    // TODO: generalize this as a trait
    pub local: FileStore<'a>,
    pub remote: Url,
    remote_client: Box<dyn RemoteClient>,
    pub flatten: bool,
    pub pathmaker: Box<&'a dyn PathMaker>,
}

#[derive(Debug)]
pub enum FactoryError {
    InvalidURL(url::ParseError),
    InvalidStore(StoreError),
    InvalidPathMaker(PathMakerError),
    InvalidRemote(RCFactoryError),
}
use FactoryError::*;

impl Mirror<'_> {
    pub fn new(cfg: SourceConfig) -> Result<Mirror<'static>, FactoryError> {
        let period = time::Duration::from_secs(cfg.period);
        let pathmaker = pathmaker::new(cfg.pathmaker);
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

        let local = FileStore::new(&cfg.local, pathmaker);
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
            remote_client,
            pathmaker: Box::new(pathmaker),
            flatten,
        };
        Ok(m)
    }

    pub fn connect(&self) -> Result<(), ConnectError> {
        self.remote_client.connect()
    }

    pub fn ping(&self) -> Result<time::Duration, PingError> {
        self.remote_client.ping()
    }

    pub fn range_to_filelist(&self, range: &TimeRange) -> FileList {
        // TODO
        FileList::empty()
    }

    pub fn captures_in_range(&self, range: &TimeRange) -> CaptureList {
        let items = self.range_to_filelist(range);
        self.local.captures_in_list(items)
    }

    pub fn latest_capture(&self) -> Option<Capture> {
        None
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
}
