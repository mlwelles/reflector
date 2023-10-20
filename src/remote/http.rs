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
    pub fn new(base: Url) -> Http {
        let builder = ureq::builder()
            .timeout_connect(Duration::from_secs(30))
            .timeout(Duration::from_secs(300));
        let agent = builder.build();
        Http { base, agent }
    }
}

impl RemoteClient for Http {
    fn ping(&mut self) -> Result<Duration, PingError> {
        match self.agent.request_url("HEAD", &self.base).call() {
            Ok(_) => Ok(Duration::new(0, 0)),
            Err(e) => Err(PingError::RequestErr(Box::new(e))),
        }
    }

    fn get(&mut self, resource: &str, output: &PathBuf) -> Result<Gotten, GetError> {
        let u = match self.base.join(resource) {
            Ok(u) => u,
            Err(e) => return Err(GetError::UnparsableURL(e)),
        };
        let resp = match self.agent.request_url("GET", &u).call() {
            Ok(resp) => resp,
            Err(e) => return Err(GetError::RequestErr(e)),
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

        let g = Gotten::new(&mimetype, resource, u, output.to_path_buf(), tot);
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
        Http::new(u)
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
        let got = m.get(MOCK_RESOURCE, &path).unwrap();
        assert_eq!(MOCK_RESOURCE, got.resource);
        assert_eq!(path, got.output);
    }

    #[test]
    fn validation() {
        let mut m = mock();
        let t = TempDir::new().unwrap();
        let path = t.path().join("test.bin");
        let got = m.get(MOCK_RESOURCE, &path).unwrap();
        got.validate().unwrap();
    }

    #[test]
    fn not_found() {
        let path = PathBuf::from("/dev/null");
        let fail = mock().get("asdfasfdasfd", &path);
        assert!(fail.is_err())
    }
}
