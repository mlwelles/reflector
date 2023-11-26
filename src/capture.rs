use std::ffi::OsString;
use std::{fmt, path, time};
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

#[derive(Debug)]
pub enum CaptureError {
    NoCaptures,
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

    pub fn full_ratio(&self) -> Result<f64, CaptureError> {
        let all = self.len_all() as f64;
        if all > 0.0 {
            Ok(self.len() as f64 / all)
        } else {
            Err(CaptureError::NoCaptures)
        }
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

impl fmt::Display for CaptureList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let rat = match self.full_ratio() {
            Ok(r) => format!(", {}% full", r),
            Err(_) => "".to_string(),
        };
        write!(
            f,
            "list of {} out of {} captures{}",
            self.len(),
            self.len_all(),
            rat,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let l = CaptureList::empty();
        assert!(l.is_empty(), "is empty");
        assert_eq!(0, l.len(), "len");
        assert_eq!(0, l.len_all(), "len_all");
        assert!(l.full_ratio().is_err(), "full_ratio");
        assert_eq!(
            "list of 0 out of 0 captures",
            format!("{}", l),
            "displayed representation"
        );
    }
}
