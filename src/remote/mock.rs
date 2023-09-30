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

#[cfg(test)]
mod tests {
    use super::*;

    fn mock() -> Mock {
        Mock {}
    }

    #[test]
    fn ping() {
        let m = mock();
        assert_ok!(m.ping())
    }
}
