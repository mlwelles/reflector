use crate::config::SourceConfig;
use std::{fs, path, time};
use url::Url;

#[derive(Debug)]
pub struct Mirror {
    pub name: String,
    pub period: time::Duration,
    pub local: path::PathBuf,
    pub remote: Url,
    flatten: bool,
    // TODO pathmaker
}

#[derive(Debug, PartialEq, Eq)]
pub enum FactoryError {
    NotDirectory(path::PathBuf),
    NotWritable(path::PathBuf),
    InvalidURL,
    InvalidLocalMetadata,
}
use FactoryError::*;

impl Mirror {
    pub fn new(cfg: SourceConfig) -> Result<Mirror, FactoryError> {
        let remote = Url::parse(&cfg.remote);
        if remote.is_err() {
            return Err(InvalidURL);
        }
        let remote = remote.unwrap();

        let period = time::Duration::from_secs(cfg.period);

        let localmd = fs::metadata(&cfg.local);
        if localmd.is_err() {
            eprintln!("{}", localmd.unwrap_err());
            return Err(InvalidLocalMetadata);
        }
        let local = path::PathBuf::from(&cfg.local);
        let permissions = localmd.unwrap().permissions();
        if permissions.readonly() {
            return Err(NotWritable(local));
        }
        if !local.is_dir() {
            return Err(NotDirectory(local));
        }

        let mut flatten = false;
        if cfg.flatten == Some(true) {
            flatten = true;
        }

        let m = Mirror {
            name: cfg.name,
            period,
            local,
            remote,
            flatten,
        };
        Ok(m)
    }
}
