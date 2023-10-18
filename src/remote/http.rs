// HTTP and HTTPS remote client

use super::*;
use std::fs::File;
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
    fn ping(&self) -> Result<Duration, PingError> {
        match self.agent.request_url("HEAD", &self.base).call() {
            Ok(_) => Ok(Duration::new(0, 0)),
            Err(e) => Err(PingError::RequestErr(Box::new(e))),
        }
    }

    fn connect(&mut self) -> Result<(), ConnectError> {
        match self.ping() {
            Ok(_) => Ok(()),
            Err(PingError::RequestErr(e)) => Err(ConnectError::RequestErr(e)),
            _ => Err(ConnectError::UnknownErr),
        }
    }

    fn get(&self, resource: &str, output: &PathBuf) -> Result<Gotten, GetError> {
        let u = match self.base.join(resource) {
            Ok(u) => u,
            Err(e) => return Err(GetError::UnparsableURL(e)),
        };
        let resp = match self.agent.request_url("GET", &u).call() {
            Ok(resp) => resp,
            Err(e) => return Err(GetError::RequestErr(e)),
        };
        let mimetype = String::from(resp.content_type());
        if output.is_dir() {
            return Err(GetError::OutputExistsAsDir(output.to_path_buf()));
        }
        if output.is_file() {
            return Err(GetError::OutputFileExists(output.to_path_buf()));
        }
        let file = match File::create(&output) {
            Err(why) => return Err(GetError::OutputCreateFile(why)),
            Ok(file) => file,
        };
        const BUFSIZE: usize = 8192;
        let mut buf: [u8; BUFSIZE] = [0; BUFSIZE];
        let mut bw = BufWriter::new(file);
        let mut r = resp.into_reader();
        while match r.read(&mut buf) {
            // we're ignoring size read here
            Ok(size) => match bw.write_all(&buf) {
                Ok(_) => {
                    if size < BUFSIZE {
                        eprintln!("short read");
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
                eprintln!("error from read: {:?}", e);
                false
            }
        } {
            ()
        }

        let g = Gotten::new(&mimetype, resource, u, output.to_path_buf());
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
        let mut m = mock();
        m.connect().unwrap();
    }

    #[test]
    fn get() {
        let m = mock();
        let rsrc = "README.html";
        let path = PathBuf::from("/dev/null");
        let got = m.get(rsrc, &path).unwrap();
        assert_eq!(rsrc, got.resource);
        assert_eq!(path, got.output);
    }

    #[test]
    fn not_found() {
        let path = PathBuf::from("/dev/null");
        let fail = mock().get("asdfasfdasfd", &path);
        assert!(fail.is_err())
    }
}
