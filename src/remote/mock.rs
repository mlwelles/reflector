use super::client::{ConnectError, GetError, PingError};
use std::time::Duration;

// barely implements RemoteClient
pub struct Mock();

impl RemoteClient for Mock {
    fn connect(&self) -> Result<(), ConnectError> {
        Ok()
    }
    fn ping(&self) -> Result<Duration, PingError> {
        Ok()
    }
}
