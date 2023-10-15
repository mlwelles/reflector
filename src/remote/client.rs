use super::Gotten;
use std::net::SocketAddr;
use std::time::Duration;
use ureq;
use url;

#[derive(Debug)]
pub enum ConnectError {
    RequestErr(ureq::Error),
    UnknownErr,
}

#[derive(Debug)]
pub enum PingError {
    RequestErr(ureq::Error),
}

#[derive(Debug)]
pub enum GetError {
    Unimplemented,
    UnparsableURL(url::ParseError),
    RequestErr(ureq::Error),
}

pub trait RemoteClient {
    fn connect(&self) -> Result<(), ConnectError>;
    fn ping(&self) -> Result<Duration, PingError>;
    fn get(&self, path: &str) -> Result<Gotten, GetError>;
    fn remote_addr(&self) -> SocketAddr;
}
