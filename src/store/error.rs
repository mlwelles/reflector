use std::ffi::OsString;
use std::path::PathBuf;
use std::{io, path};

#[derive(Debug)]
pub enum StoreError {
    InvalidLocalMetadata(io::Error, path::PathBuf),
    NotDirectory(path::PathBuf),
    NotWritable(path::PathBuf),
    NotImplemented,
}

#[derive(Debug, PartialEq)]
pub enum StoreGetError {
    NotAFile(PathBuf),
    NoSuchFile(PathBuf),
    IncomprehensibleFilename(OsString),
}
