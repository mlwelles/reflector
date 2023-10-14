// a class representing a file store on local disk, geared towards
// storing and retreiving captures and dealing in CaptureLists

use crate::store::StoreError::*;
use crate::{Capture, CaptureList, PathMaker, StoreError};
use std::{
    ffi::OsString,
    fmt, fs, io,
    path::{self, PathBuf},
};
use url::Url;

pub struct FileStore {
    pub path: path::PathBuf,
    pub pathmaker: Box<dyn PathMaker>,
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

impl FileStore {
    pub fn new(path: &str, pathmaker: Box<dyn PathMaker>) -> Result<Self, StoreError> {
        let path = path::PathBuf::from(path);
        Ok(FileStore {
            path,
            pathmaker,
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

    pub fn get_str(&self, p: &str) -> Result<Capture, GetError> {
        self.get(&PathBuf::from(p))
    }

    pub fn get(&self, p: &PathBuf) -> Result<Capture, GetError> {
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
                Err(_) => Err(IncomprehensibleFilename(f.to_os_string())),
            },
            None => Err(NoSuchFile(fetched)),
        }
    }

    pub fn put(&self, path: &PathBuf, contents: &[u8]) -> io::Result<()> {
        fs::write(self.path.join(path), contents)
    }

    pub fn captures_in_list(&self, ll: FileList) -> CaptureList {
        let mut cl = CaptureList::empty();
        for l in ll {
            match self.get_str(&l) {
                Ok(c) => cl.list.push(c),
                Err(e) => eprintln!("error on getting capture '{l}': {:?}", e),
            }
        }
        cl
    }
}

pub struct FileStoreConfig {
    pub path: String,
    pub pathmaker: Box<dyn PathMaker>,
}

impl From<FileStoreConfig> for FileStore {
    fn from(cfg: FileStoreConfig) -> FileStore {
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

impl fmt::Display for FileStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "file storage in dir {}", self.path.to_str().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pathmaker;
    use chrono::Utc;

    const MOCK_PATH: &str = "/tmp/reflector_file_store_test";

    fn mock_file_store() -> FileStore {
        let pbuf = path::PathBuf::from(MOCK_PATH);
        if !pbuf.is_dir() {
            fs::create_dir(&pbuf).unwrap()
        }
        let pathmaker = Box::new(pathmaker::Identity::new());
        FileStore {
            path: pbuf,
            pathmaker,
            url: None,
        }
    }

    #[test]
    fn check_mock() {
        let m = mock_file_store();
        assert_eq!(m.path, PathBuf::from(MOCK_PATH));
        assert_eq!((), m.validate().unwrap());
    }

    #[test]
    fn format() {
        let m = mock_file_store();
        assert_eq!(
            format!("{m}"),
            "file storage in dir /tmp/reflector_file_store_test"
        );
    }

    #[test]
    fn get_put() {
        let m = mock_file_store();
        let f = m.pathmaker.time_to_filename(&Utc::now());
        let p = PathBuf::from(&f);

        m.put(&p, f.into_string().unwrap().as_bytes()).unwrap();
        let c = m.get(&p).unwrap();
        assert!(c.valid());
    }
}
