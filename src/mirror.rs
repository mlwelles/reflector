use std::time::Duration;

use crate::config::SourceConfig;
use std::path;
use url::Url;

#[derive(Debug)]
pub struct Mirror {
    pub name: String,
    pub period: Duration,
    pub local: path::PathBuf,
    pub remote: Url,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FactoryError {
    NotDirectory(path::PathBuf),
    NotWritable(path::PathBuf),
    InvalidURL,
}
use FactoryError::*;

impl Mirror {
    pub fn new(cfg: SourceConfig) -> Result<Mirror, FactoryError> {
        let remote = Url::parse(&cfg.remote);
        if remote.is_err() {
            return Err(InvalidURL);
        }
        let remote = remote.unwrap();
        let local = path::PathBuf::from(&cfg.local);
        if !local.is_dir() {
            return Err(NotDirectory(local));
        }
        let m = Mirror {
            name: cfg.name,
            period: cfg.period,
            local,
            remote,
        };
        Ok(m)
    }
}
