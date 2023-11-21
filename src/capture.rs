use std::ffi::OsString;
use std::{path, time};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capture {
    pub time: time::SystemTime,
    pub path: path::PathBuf,
    pub url: Option<Url>,
}

impl Capture {
    pub fn valid(&self) -> bool {
        self.path.is_file()
    }
}

#[derive(Clone, Debug)]
pub struct CaptureList {
    pub list: Vec<Capture>,
    pub missing: Vec<OsString>,
}

impl CaptureList {
    pub fn new(list: Vec<Capture>, missing: Vec<OsString>) -> Self {
        CaptureList { list, missing }
    }

    pub fn empty() -> Self {
        CaptureList::new(vec![], vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty() && self.missing.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn len_all(&self) -> usize {
        self.list.len() + self.missing.len()
    }
}

impl From<Capture> for CaptureList {
    fn from(init: Capture) -> Self {
        CaptureList::new(vec![init], vec![])
    }
}

impl Iterator for CaptureList {
    type Item = Capture;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

pub enum CaptureError {
    NoCaptures,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let l = CaptureList::empty();
        assert!(l.is_empty(), "is empty");
        assert_eq!(0, l.len(), "len");
    }
}
