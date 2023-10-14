use super::Gotten;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(PartialEq, Eq, Debug)]
pub enum ConnectError {
    Erm,
}

#[derive(PartialEq, Eq, Debug)]
pub enum PingError {
    Erm,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GetError {
    Erm,
}

pub trait RemoteClient {
    fn connect(&self) -> Result<(), ConnectError>;
    fn ping(&self) -> Result<Duration, PingError>;
    fn get(&self, path: &str) -> Result<Gotten, GetError>;
    fn remote_addr(&self) -> SocketAddr;
}
