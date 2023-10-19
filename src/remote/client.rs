use super::Gotten;
use std::fs::File;
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
    FtpCwdErr(FtpError),
    SocketError(io::Error),
}

#[derive(Debug)]
pub enum PingError {
    Unimplemented,
    NotConnected,
    RequestErr(Box<ureq::Error>),
    FtpNoopError(FtpError),
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
    RetrieveError(FtpError),
}

pub trait RemoteClient {
    fn ping(&mut self) -> Result<Duration, PingError>;
    fn get(&mut self, resource: &str, output: &PathBuf) -> Result<Gotten, GetError>;
    fn remote_addr(&self) -> SocketAddr;

    fn create_output(&self, output: &PathBuf) -> Result<File, GetError> {
        if output.is_dir() {
            return Err(GetError::OutputExistsAsDir(output.to_path_buf()));
        }
        if output.is_file() {
            return Err(GetError::OutputFileExists(output.to_path_buf()));
        }
        match File::create(output) {
            Err(why) => Err(GetError::OutputCreateFile(why)),
            Ok(file) => Ok(file),
        }
    }
}
