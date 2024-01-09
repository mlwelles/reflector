//! Local data storage.

pub mod file;
pub use file::FileStore;
pub mod error;
pub use error::{StoreError, StoreGetError};

pub mod file_list;
pub use file_list::FileList;
