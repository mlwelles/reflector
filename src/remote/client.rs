use super::Gotten;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use suppaftp::FtpError;
use ureq;
use url;

#[derive(Debug)]
pub enum ConnectError {
    Unimplemented,
    RequestErr(Box<ureq::Error>),
    UnknownErr,
    FtpConnectErr(FtpError),
    FtpLoginErr(FtpError),
    SocketError(io::Error),
}

#[derive(Debug)]
pub enum PingError {
    Unimplemented,
    NotConnected,
    RequestErr(Box<ureq::Error>),
}

#[derive(Debug)]
pub enum GetError {
    Unimplemented,
    UnparsableURL(url::ParseError),
    NotConnected,
    RequestErr(ureq::Error),
    OutputExistsAsDir(PathBuf),
    OutputFileExists(PathBuf),
    OutputCreateFile(io::Error),
}

pub trait RemoteClient {
    fn ping(&self) -> Result<Duration, PingError>;
    fn get(&self, resource: &str, output: &PathBuf) -> Result<Gotten, GetError>;
    fn remote_addr(&self) -> SocketAddr;
}
