// HTTP and HTTPS remote client

use super::*;
use std::io::{BufWriter, Write};
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::time::Duration;
use ureq;
use url::Url;

pub struct Http {
    base: Url,
    agent: ureq::Agent,
}

impl Http {
    pub fn new(base: &Url) -> Http {
        let base = base.clone();
        let builder = ureq::builder()
            .timeout_connect(Duration::from_secs(30))
            .timeout(Duration::from_secs(300));
        let agent = builder.build();
        Http { base, agent }
    }

    fn url(&self, resource: &str) -> Result<Url, GetError> {
        match self.base.join(resource) {
            Ok(u) => Ok(u),
            Err(e) => return Err(GetError::UnparsableURL(e)),
        }
    }
}

impl RemoteClient for Http {
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
            Err(ureq::Error::Status(c, _)) if c == 404 => return Ok(false),
            Err(e) => return Err(GetError::RequestErr(Box::new(e))),
        }
    }

    fn get(&mut self, resource: &str, output: PathBuf) -> Result<Gotten, GetError> {
        let u = self.url(resource)?;
        let resp = match self.agent.request_url("GET", &u).call() {
            Ok(resp) => resp,
            Err(e) => return Err(GetError::RequestErr(Box::new(e))),
        };
        let mimetype = String::from(resp.content_type());
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
            Ok(size) => match bw.write_all(&buf[0..size]) {
                Ok(_) => {
                    tot += size as u64;
                    if size == 0 {
                        eprintln!("zero read after {tot} bytes");
                        false
                    } else {
                        true
                    }
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
            eprintln!("read {tot} bytes");
        }

        let g = Gotten::new(&mimetype, resource, u, output, tot);
        Ok(g)
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
    use tempfile::TempDir;

    const MOCK_RESOURCE: &str = "README.html";

    fn mock() -> Http {
        let u = Url::parse("http://deb.debian.org/debian/").unwrap();
        Http::new(&u)
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
        assert_eq!(true, e, "resource exists");
        let e = m.exists("asdfasdfasdfafdasfdasdf").unwrap();
        assert_eq!(false, e, "resource doesn't exist");
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
        let t = TempDir::new().unwrap();
        let path = t.path().join("test.bin");
        let got = m.get(MOCK_RESOURCE, path).unwrap();
        got.validate().unwrap();
    }

    #[test]
    fn not_found() {
        let path = PathBuf::from("/dev/null");
        let fail = mock().get("asdfasfdasfd", path);
        assert!(fail.is_err())
    }
}
