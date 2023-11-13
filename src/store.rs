//! Local data storage.

pub mod file;
pub use file::{FileList, FileStore};
pub mod error;
pub use error::StoreError;
