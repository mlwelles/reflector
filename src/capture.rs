use std::{path, time};
use url::Url;

#[derive(Debug, PartialEq, Eq)]
pub struct Capture {
    time: time::SystemTime,
    path: path::PathBuf,
    url: Url,
}

#[derive(Debug)]
pub struct CaptureList {
    list: Vec<Capture>,
}

impl CaptureList {
    pub fn empty() -> CaptureList {
        CaptureList { list: vec![] }
    }
}

pub enum CaptureError {
    NoCaptures,
}
