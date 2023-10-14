use std::{path, time};
use url::Url;

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct CaptureList {
    pub list: Vec<Capture>,
}

impl CaptureList {
    pub fn empty() -> CaptureList {
        CaptureList { list: vec![] }
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
