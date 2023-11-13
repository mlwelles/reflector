//! Protocol engines, used to retrieve files from remote sites.

pub mod client;
pub use client::{ConnectError, GetError, ListError, PingError, RemoteClient};
pub mod factory;
pub use factory::{from_url, RCFactoryError};
pub mod gotten;
pub use gotten::{Gotten, GottenValidation};
pub mod http;
pub use http::Http;
pub mod ftp;
pub use ftp::Ftp;

// TODO: test only?
pub mod mock;
