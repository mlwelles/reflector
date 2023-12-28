use std::ffi::{OsStr, OsString};
use std::fmt;
use std::os::unix::ffi::OsStrExt;
use std::str;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FileList {
    list: Vec<OsString>,
}

impl FileList {
    pub fn empty() -> FileList {
        let list = vec![];
        FileList { list }
    }

    pub fn push(&mut self, s: &OsStr) {
        self.list.push(OsString::from(s))
    }
    pub fn push_str(&mut self, s: &str) {
        self.list.push(OsString::from(s))
    }

    // tramping from our stored list class

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl Iterator for FileList {
    type Item = OsString;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl From<String> for FileList {
    fn from(initial: String) -> Self {
        let list = vec![OsString::from(initial)];
        Self { list }
    }
}

impl From<Vec<String>> for FileList {
    fn from(ss: Vec<String>) -> Self {
        let list = ss.iter().map(OsString::from).collect();
        Self { list }
    }
}

impl fmt::Display for FileList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in self.clone() {
            write!(f, "{}, ", str::from_utf8(i.as_bytes()).unwrap())?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut l = FileList::empty();
        assert!(l.is_empty(), "is_empty()");
        l.push_str("cargo test");
        assert!(!l.is_empty(), "no longer empty");
    }

    #[test]
    fn from_vec() {
        let l = FileList::from(vec!["item1".to_string(), "item2".to_string()]);
        assert_eq!(2, l.len(), "len()");

        let cl = l.clone();
        assert_eq!(l.len(), cl.len(), "cloned len()");
    }
}
