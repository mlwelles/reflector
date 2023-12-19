use std::{io, path};

#[derive(Debug)]
pub enum StoreError {
    InvalidLocalMetadata(io::Error, path::PathBuf),
    NotDirectory(path::PathBuf),
    NotWritable(path::PathBuf),
    NotImplemented,
}
