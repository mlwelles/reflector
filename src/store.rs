//! Local data storage.

pub mod file;
pub use file::FileStore;
pub mod error;
pub use error::StoreError;

pub mod file_list;
pub use file_list::FileList;
