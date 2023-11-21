use std::ffi::{OsStr, OsString};

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
        let list = ss.iter().map(|s| OsString::from(s)).collect();
        Self { list }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut l = FileList::empty();
        assert!(l.is_empty(), "is_empty()");
        l.push("bar");
        assert!(!l.is_empty(), "no longer empty");
    }

    #[test]
    fn from_vec() {
        let l = FileList::from(vec!["item1".to_string(), "item2".to_string()]);
        assert_eq!(2, l.len(), "len()");
    }
}
