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
    FtpLoginErr(FtpError),
}

#[derive(Debug)]
pub enum PingError {
    Unimplemented,
    RequestErr(Box<ureq::Error>),
}

#[derive(Debug)]
pub enum GetError {
    Unimplemented,
    UnparsableURL(url::ParseError),
    RequestErr(ureq::Error),
    OutputExistsAsDir(PathBuf),
    OutputFileExists(PathBuf),
    OutputCreateFile(io::Error),
}

pub trait RemoteClient {
    fn connect(&self) -> Result<(), ConnectError>;
    fn ping(&self) -> Result<Duration, PingError>;
    fn get(&self, resource: &str, output: &PathBuf) -> Result<Gotten, GetError>;
    fn remote_addr(&self) -> SocketAddr;
}
