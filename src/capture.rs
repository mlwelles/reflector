//! A simple representation of a capture and list of the same.
//!
//! [Capture]s represent a captured resource from a [Remote] source.
//!
//! They are collected into a [CaptureList], which also includes information on
//! any missing resources which cannot be made into [Capture]s.

use crate::remote::Gotten;
use crate::time_util::display_systime;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::SystemTime;
use std::{fmt, path, time};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capture {
    pub time: SystemTime,
    pub path: PathBuf,
    pub url: Option<Url>,
}

impl Capture {
    pub fn new(time: SystemTime, path: PathBuf, url: Option<Url>) -> Self {
        Self { time, path, url }
    }

    pub fn valid(&self) -> bool {
        self.path.is_file()
    }
}

impl From<(Gotten, SystemTime)> for Capture {
    fn from(input: (Gotten, SystemTime)) -> Self {
        let g = input.0;
        Self::new(input.1, g.output, Some(g.source))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureMissing {
    pub time: time::SystemTime,
    pub resource: String,
    pub path: path::PathBuf,
}

impl CaptureMissing {
    pub fn new(time: SystemTime, path: path::PathBuf, resource: &str) -> Self {
        let resource = resource.to_string();
        Self {
            time,
            resource,
            path,
        }
    }
}

impl fmt::Display for CaptureMissing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Missing '{}' at {}>",
            self.resource,
            display_systime(&self.time),
        )
    }
}

#[derive(Clone, Debug)]
pub struct CaptureList {
    pub list: VecDeque<Capture>,
    pub missing: VecDeque<CaptureMissing>,
}

#[derive(Debug)]
pub enum CaptureError {
    NoCaptures,
}

impl CaptureList {
    pub fn new(list: VecDeque<Capture>, missing: VecDeque<CaptureMissing>) -> Self {
        CaptureList { list, missing }
    }

    pub fn empty() -> Self {
        CaptureList::new(VecDeque::new(), VecDeque::new())
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty() && self.missing.is_empty()
    }

    pub fn has_captures(&self) -> bool {
        !self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn len_all(&self) -> usize {
        self.list.len() + self.missing.len()
    }

    pub fn push(&mut self, cap: Capture) {
        self.list.push_back(cap)
    }

    pub fn push_missing(&mut self, mis: CaptureMissing) {
        self.missing.push_back(mis)
    }

    pub fn latest(&self) -> Option<&Capture> {
        self.list.back()
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
        CaptureList::new(VecDeque::from([init]), VecDeque::new())
    }
}

impl Iterator for CaptureList {
    type Item = Capture;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl fmt::Display for CaptureList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let rat = match self.full_ratio() {
            Ok(r) => format!(", {}% full", r * 100.0), // FIXME: rounding
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
        assert_eq!(None, l.latest(), "latest");
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
