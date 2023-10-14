// HTTP and HTTPS remote client

use super::*;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use ureq;
use url::Url;

pub struct Http {
    base: Url,
}

impl Http {
    pub fn new(base: Url) -> Http {
        let mut builder = ureq::builder()
            .timeout_connect(Duration::from_secs(30))
            .timeout(Duration::from_secs(300));
        Http { base }
    }
}

impl RemoteClient for Http {
    fn connect(&self) -> Result<(), ConnectError> {
        Ok(())
    }
    fn ping(&self) -> Result<Duration, PingError> {
        Ok(Duration::new(0, 0))
    }
    fn get(&self, path: &str) -> Result<Gotten, GetError> {
        Ok(Gotten::new("mumble-mime-type"))
    }
    fn remote_addr(&self) -> SocketAddr {
        let host = self.base.host_str().unwrap();
        let port = self.base.port_or_known_default().unwrap();
        (host, port).to_socket_addrs().unwrap().next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock() -> Mock {
        Mock {}
    }
}
