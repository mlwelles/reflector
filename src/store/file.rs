// a class representing a file store on local disk, geared towards
// storing and retreiving captures and dealing in CaptureLists

use super::StoreGetError;
use super::StoreGetError::*;
use crate::store::StoreError::*;
use crate::{Capture, CaptureList, CaptureMissing, FileList, PathMaker, StoreError};
use log::info;
use std::time::SystemTime;
use std::{
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

impl FileStore {
    pub fn new(path: &str, pathmaker: Box<dyn PathMaker>) -> Result<Self, StoreError> {
        let path = path::PathBuf::from(path);
        let fs = FileStore {
            path,
            pathmaker,
            url: None,
        };
        match fs.validate() {
            Ok(_) => Ok(fs),
            Err(e) => Err(e),
        }
    }

    pub fn validate(&self) -> Result<(), StoreError> {
        let localmd = fs::metadata(&self.path);
        if let Err(e) = localmd {
            return Err(InvalidLocalMetadata(e, path::PathBuf::from(&self.path)));
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

    pub fn join(&self, p: &PathBuf) -> PathBuf {
        self.path.join(p)
    }

    pub fn get(&self, p: &PathBuf) -> Result<Capture, StoreGetError> {
        let fetched = self.join(p);
        // eprintln!("{}", fetched.metadata().unwrap().file_type());
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
            None => Err(NotAFile(fetched)), // FIXME: not sure about this state
        }
    }

    pub fn get_str(&self, p: &str) -> Result<Capture, StoreGetError> {
        self.get(&PathBuf::from(p))
    }

    pub fn put(&self, path: &PathBuf, contents: &[u8]) -> io::Result<()> {
        fs::write(self.join(path), contents)
    }

    pub fn captures_in_list(&self, ll: FileList) -> CaptureList {
        let mut cl = CaptureList::empty();
        for l in ll {
            let p = PathBuf::from(&l);
            match self.get(&p) {
                Ok(c) => cl.push(c),
                Err(e) => {
                    let cs = l.to_str().unwrap();
                    let time = self.pathmaker.filename_to_systime(&l).unwrap_or_else(|ee| {
                        eprintln!(
                            "got error attempting to backtrack filename to systime: {:?}",
                            ee
                        );
                        SystemTime::now()
                    });
                    let m = CaptureMissing::new(time, p, cs);
                    cl.push_missing(m);
                    match e {
                        NoSuchFile(_) => info!(target: "remote",
                            "capture {} not found in dir {}",
                            cs,
                            self.path.to_str().unwrap()
                        ),
                        _ => eprintln!("error on getting capture '{}': {:?}", cs, e),
                    };
                }
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
    fn from(cfg: FileStoreConfig) -> Self {
        FileStore {
            path: PathBuf::from(cfg.path),
            pathmaker: cfg.pathmaker,
            url: None,
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
    use std::ffi::OsString;

    const MOCK_PATH: &str = "/tmp/reflector_file_store_test";
    static MOCK_FILE: &str = "2023-11-21T14:40:00+00:00";

    fn mock_file_store() -> FileStore {
        let pbuf = path::PathBuf::from(MOCK_PATH);
        if !pbuf.is_dir() {
            fs::create_dir(&pbuf).unwrap()
        }
        let pathmaker = Box::new(pathmaker::Identity::new());

        // create a file to be in our store
        fs::write(pbuf.join(PathBuf::from(MOCK_FILE)), "just testing")
            .unwrap_or_else(|e| eprintln!("error setting up file in mock store: {e}"));

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

        let f = OsString::from(MOCK_FILE);
        let _f = m.pathmaker.filename_to_systime(&f).unwrap();
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

    #[test]
    fn captures_in_list() {
        let m = mock_file_store();
        let l = FileList::from(vec![MOCK_FILE.to_string(), "doesnotexist".to_string()]);
        let c = m.captures_in_list(l.clone());
        assert_eq!(l.len(), c.len_all(), "one capture per file in list");
        assert_eq!(1, c.list.len(), "found");
        assert_eq!(1, c.missing.len(), "missing");
    }
}
