// a class representing a file store on local disk
use crate::store::StoreError::*;
use crate::{Capture, CaptureList, PathMaker, StoreError};
use std::{
    ffi::OsString,
    fmt, fs,
    path::{self, PathBuf},
};
use url::Url;

pub struct FileStore<'a> {
    pub path: path::PathBuf,
    pub pathmaker: Box<&'a dyn PathMaker>,
    // remote URL if any
    pub url: Option<Url>,
}

pub struct FileList {
    list: Vec<String>,
}

impl FileList {
    pub fn empty() -> FileList {
        let list = vec![];
        FileList { list }
    }
}

impl Iterator for FileList {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

#[derive(Debug)]
pub enum GetError {
    NoSuchFile(PathBuf),
    IncomprehensibleFilename(OsString),
}
use GetError::*;

impl FileStore<'_> {
    pub fn new(path: &str, pathmaker: &dyn PathMaker) -> Result<Self, StoreError> {
        Ok(FileStore {
            path: path::PathBuf::from(path),
            pathmaker: Box::new(pathmaker),
            url: None,
        })
    }

    pub fn validate(&self) -> Result<(), StoreError> {
        let localmd = fs::metadata(&self.path);
        if let Err(e) = localmd {
            return Err(InvalidLocalMetadata(e));
        }
        let local = path::PathBuf::from(&self.path);
        let permissions = localmd.unwrap().permissions();
        if permissions.readonly() {
            return Err(NotWritable(local));
        }
        if !local.is_dir() {
            return Err(NotDirectory(local));
        }
        Ok(())
    }

    fn get(&self, p: &str) -> Result<Capture, GetError> {
        let fetched = self.path.join(p);
        if !fetched.is_file() {
            return Err(NoSuchFile(fetched));
        }
        match fetched.file_name() {
            Some(f) => match self.pathmaker.filename_to_systime(f) {
                Ok(time) => Ok(Capture {
                    time,
                    path: fetched,
                    url: None,
                }),
                Err(e) => Err(IncomprehensibleFilename(f.to_os_string())),
            },
            None => Err(NoSuchFile(fetched)),
        }
    }

    pub fn captures_in_list(&self, ll: FileList) -> CaptureList {
        let cl = CaptureList::empty();
        for l in ll {
            match self.get(&l) {
                Ok(c) => cl.list.push(c),
                Err(e) => eprintln!("error on getting capture '{l}': {:?}", e),
            }
        }
        cl
    }
}

pub struct FileStoreConfig<'a> {
    pub path: String,
    pub pathmaker: Box<&'a dyn PathMaker>,
}

impl From<FileStoreConfig<'_>> for FileStore<'_> {
    fn from(cfg: FileStoreConfig<'_>) -> FileStore<'static> {
        let path = path::PathBuf::from(cfg.path);
        let pathmaker = cfg.pathmaker;
        let url = None;
        FileStore {
            path,
            pathmaker,
            url,
        }
    }
}

impl fmt::Display for FileStore<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file storage in dir {}", self.path.to_str().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_file_store() -> FileStore {
        let path = "/tmp/reflector_file_store_test";
        let pbuf = path::PathBuf::from("/tmp/reflector_file_store_test");
        if !pbuf.is_dir() {
            panic!("please make path {}", path);
        }
        let pathmaker = PathMaker::Identity;
        FileStore {
            path: pbuf,
            pathmaker,
        }
    }

    #[test]
    fn check_mock() {
        let m = mock_file_store();
        assert_eq!(format!("{m}"), "test file store");
    }
}
