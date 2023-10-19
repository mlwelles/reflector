// barely implements RemoteClient

use super::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

pub struct Mock();

impl RemoteClient for Mock {
    fn ping(&mut self) -> Result<Duration, PingError> {
        Ok(Duration::new(0, 0))
    }

    fn get(&mut self, resource: &str, output: &PathBuf) -> Result<Gotten, GetError> {
        let source = Url::parse("http://127.0.0.1/").unwrap();
        Ok(Gotten::new(
            "x-raw/mock",
            resource,
            source,
            output.to_path_buf(),
        ))
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
        let mut m = mock();
        m.ping().unwrap();
    }

    #[test]
    fn get() {
        let mut m = mock();
        let path = PathBuf::from("/dev/null");
        m.get("mumble", &path).unwrap();
    }
}
