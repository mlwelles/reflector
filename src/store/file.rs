// a class representing a file store on local disk
use crate::store::StoreError::*;
use crate::{CaptureList, PathMaker, StoreError, TimeRange};
use std::{fs, path};

#[derive(Debug)]
pub struct FileStore {
    pub path: path::PathBuf,
    pub pathmaker: PathMaker,
}

impl FileStore {
    pub fn new(path: &str, pathmaker: &PathMaker) -> Result<Self, StoreError> {
        let localmd = fs::metadata(path);
        if localmd.is_err() {
            return Err(InvalidLocalMetadata(localmd.unwrap_err()));
        }
        let local = path::PathBuf::from(path);
        let permissions = localmd.unwrap().permissions();
        if permissions.readonly() {
            return Err(NotWritable(local));
        }
        if !local.is_dir() {
            return Err(NotDirectory(local));
        }
        let pathmaker = pathmaker.dup();
        Ok(FileStore {
            path: local,
            pathmaker,
        })
    }

    // there's two ways to return a set of files from a given time range:
    // (1) PathMaker(range) and check that those files exist in the local store
    // (2) walk the dir and return files within the two time periods
    // which makes more sense?  how best to synthesize this

    pub fn from_range(&self, range: &TimeRange) -> Result<CaptureList, StoreError> {
        Err(NotImplemented)
    }
}
