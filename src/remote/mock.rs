// barely implements RemoteClient

use super::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use url::Url;

pub struct Mock();

impl RemoteClient for Mock {
    fn connect(&self) -> Result<(), ConnectError> {
        Ok(())
    }
    fn ping(&self) -> Result<Duration, PingError> {
        Ok(Duration::new(0, 0))
    }
    fn get(&self, resource: &str) -> Result<Gotten, GetError> {
        let source = Url::parse("http://127.0.0.1/").unwrap();
        Ok(Gotten::new("x-raw/mock", resource, source))
    }
    fn remote_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6666)
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
        m.ping().unwrap();
    }

    #[test]
    fn connect() {
        let m = mock();
        m.connect().unwrap();
    }
}
