// HTTP and HTTPS remote client

use super::*;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use ureq;
use url::Url;

pub struct Http {
    base: Url,
    agent: ureq::Agent,
}

impl Http {
    pub fn new(base: Url) -> Http {
        let builder = ureq::builder()
            .timeout_connect(Duration::from_secs(30))
            .timeout(Duration::from_secs(300));
        let agent = builder.build();
        Http { base, agent }
    }
}

impl RemoteClient for Http {
    fn ping(&self) -> Result<Duration, PingError> {
        match self.agent.request_url("HEAD", &self.base).call() {
            Ok(_) => Ok(Duration::new(0, 0)),
            Err(e) => Err(PingError::RequestErr(e)),
        }
    }

    fn connect(&self) -> Result<(), ConnectError> {
        match self.ping() {
            Ok(_) => Ok(()),
            Err(PingError::RequestErr(e)) => Err(ConnectError::RequestErr(e)),
            _ => Err(ConnectError::UnknownErr),
        }
    }

    fn get(&self, resource: &str) -> Result<Gotten, GetError> {
        let u = match self.base.join(resource) {
            Ok(u) => u,
            Err(e) => return Err(GetError::UnparsableURL(e)),
        };
        match self.agent.request_url("GET", &u).call() {
            Ok(resp) => Ok(Gotten::new(resp.content_type(), resource, u)),
            Err(e) => Err(GetError::RequestErr(e)),
        }
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

    fn mock() -> Http {
        let u = Url::parse("http://deb.debian.org/debian/").unwrap();
        Http::new(u)
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

    #[test]
    fn get() {
        let m = mock();
        let rsrc = "README.html";
        let got = m.get(rsrc).unwrap();
        assert_eq!(rsrc, got.resource);
        let fail = m.get("asdfasfdasfd");
        assert!(fail.is_err())
    }
}
