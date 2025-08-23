// HTTP and HTTPS remote client

use super::*;
use log::debug;
use std::io::{BufWriter, Write};
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use ureq;
use url::{ParseError, Url};

pub struct Http {
    pub base: Url,
    pub agent: ureq::Agent,
}

impl Http {
    pub fn new(inbound: &Url) -> Http {
        let base = match inbound.as_str().chars().last() {
            Some('/') => inbound.clone(),
            _ => {
                let s = inbound.to_string() + "/";
                Url::parse(&s).unwrap()
            }
        };
        let builder = ureq::builder()
            .timeout_connect(Duration::from_secs(30))
            .timeout(Duration::from_secs(300));
        let agent = builder.build();
        Http { base, agent }
    }
}

impl FromStr for Http {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = Url::parse(s)?;
        Ok(Http::new(&u))
    }
}

impl RemoteClient for Http {
    fn remote_addr(&self) -> SocketAddr {
        let host = self.base.host_str().unwrap();
        let port = self.base.port_or_known_default().unwrap();
        (host, port).to_socket_addrs().unwrap().next().unwrap()
    }

    fn url(&self, resource: &str) -> Result<Url, GetError> {
        match self.base.join(resource) {
            Ok(u) => Ok(u),
            Err(e) => Err(GetError::UnparsableURL(e)),
        }
    }

    fn ping(&mut self) -> Result<Duration, PingError> {
        match self.agent.request_url("HEAD", &self.base).call() {
            Ok(_) => Ok(Duration::new(0, 0)),
            Err(e) => Err(PingError::RequestErr(Box::new(e))),
        }
    }

    fn exists(&self, resource: &str) -> Result<bool, GetError> {
        let u = self.url(resource)?;
        match self.agent.request_url("HEAD", &u).call() {
            Ok(_) => Ok(true),
            Err(ureq::Error::Status(404, _)) => Ok(false),
            Err(e) => Err(GetError::RequestErr(Box::new(e))),
        }
    }

    fn get(&mut self, resource: &str, output: PathBuf) -> Result<Gotten, GetError> {
        let u = self.url(resource)?;
        let resp = match self.agent.request_url("GET", &u).call() {
            Ok(resp) => resp,
            Err(e) => return Err(GetError::RequestErr(Box::new(e))),
        };
        let mimetype = String::from(resp.content_type());
        debug!("get with output to {}", output.to_str().unwrap());

        let file = match self.create_output(&output) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("error on create: {:?}", e);
                return Err(e);
            }
        };

        const BUFSIZE: usize = 8192;
        let mut buf: [u8; BUFSIZE] = [0; BUFSIZE];
        let mut bw = BufWriter::new(file);
        let mut r = resp.into_reader();
        let mut tot: u64 = 0;
        // keep looping while true
        while match r.read(&mut buf) {
            Ok(0) => false,
            Ok(size) => match bw.write_all(&buf[0..size]) {
                Ok(_) => {
                    tot += size as u64;
                    true
                }
                Err(e) => {
                    eprintln!("error from write: {:?}", e);
                    false
                }
            },
            Err(e) => {
                eprintln!("error from read after {} bytes: {:?}", tot, e);
                false
            }
        } {
            // no op
        }

        let g = Gotten::new(&mimetype, resource, u, output, tot);
        Ok(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    const MOCK_URL: &str = "http://deb.debian.org/debian";
    const MOCK_RESOURCE: &str = "README.html";

    fn mock() -> Http {
        Http::from_str(MOCK_URL).expect("unable to setup mock from base URL")
    }

    #[test]
    fn url() {
        let exp = format!("{}/{}", MOCK_URL, MOCK_RESOURCE);
        assert_eq!(
            Url::parse(&exp).unwrap(),
            mock().url(MOCK_RESOURCE).unwrap()
        )
    }

    #[test]
    fn url_trailing_slash() {
        let m1 =
            Http::from_str("http://deb.debian.org/debian").expect("client without trailing slash");
        let m2 =
            Http::from_str("http://deb.debian.org/debian/").expect("client with trailing slash");
        let t = "testing";
        assert_eq!(m1.url(t).unwrap(), m2.url(t).unwrap());
    }

    #[test]
    fn ping() {
        let mut m = mock();
        m.ping().unwrap();
    }

    #[test]
    fn exists() {
        let m = mock();
        let e = m.exists(MOCK_RESOURCE).unwrap();
        assert!(e, "resource exists");
        let e = m.exists("asdfasdfasdfafdasfdasdf").unwrap();
        assert!(!e, "resource doesn't exist");
    }

    #[test]
    fn get() {
        let mut m = mock();
        let path = PathBuf::from("/dev/null");
        let got = m.get(MOCK_RESOURCE, path.clone()).unwrap();
        assert_eq!(MOCK_RESOURCE, got.resource);
        assert_eq!(path, got.output);
    }

    #[test]
    fn validation() {
        let mut m = mock();
        let mut t = env::temp_dir();
        t.push("reflector-http-validation-test");
        fs::create_dir_all(&t).expect("failed to create temp directory");
        let path = t.join("test.bin");
        let got = m.get(MOCK_RESOURCE, path.clone()).unwrap();
        got.validate().unwrap();
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn not_found() {
        let path = PathBuf::from("/dev/null");
        let fail = mock().get("asdfasfdasfd", path);
        assert!(fail.is_err())
    }
}
