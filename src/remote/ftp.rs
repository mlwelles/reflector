use super::*;
use log::{debug, warn};
use std::io::{BufWriter, Write};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use suppaftp::FtpStream;
use url::Url;

pub struct FtpCredentials {
    pub user: String,
    password: String,
}

impl Default for FtpCredentials {
    fn default() -> FtpCredentials {
        let user = "ftp".to_string();
        let password = "a.p.dicarlo@gmail.com".to_string();
        FtpCredentials { user, password }
    }
}

pub struct Ftp {
    pub base: Url,
    pub stream: FtpStream,
    pub creds: FtpCredentials,
    remote: SocketAddr,
}

fn connect(
    remote: SocketAddr,
    base: &Url,
    creds: &FtpCredentials,
) -> Result<FtpStream, ConnectError> {
    let mut stream = match FtpStream::connect_timeout(remote, Duration::new(10, 0)) {
        Ok(s) => s,
        Err(e) => return Err(ConnectError::FtpConnectErr(e)),
    };
    if let Err(e) = stream.login(&creds.user, &creds.password) {
        return Err(ConnectError::FtpLoginErr(e));
    }
    let dir = base.path();
    if dir.len() > 1 {
        if let Err(e) = stream.cwd(dir) {
            return Err(ConnectError::FtpCwdErr(e));
        }
    }
    Ok(stream)
}

impl Ftp {
    pub fn new(base: &Url, creds: Option<FtpCredentials>) -> Result<Ftp, ConnectError> {
        let remote = match base.socket_addrs(|| None) {
            Ok(a) => a[0],
            Err(e) => return Err(ConnectError::SocketError(e)),
        };
        let creds = creds.unwrap_or_default();
        let base = base.clone();
        match connect(remote, &base, &creds) {
            Ok(stream) => Ok(Ftp {
                base,
                stream,
                creds,
                remote,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn listing(&mut self) -> Result<Vec<String>, ListError> {
        match self.stream.nlst(None) {
            Ok(s) => Ok(s),
            Err(e) => Err(ListError::FtpNlstError(e)),
        }
    }
}

impl RemoteClient for Ftp {
    fn ping(&mut self) -> Result<Duration, PingError> {
        match self.stream.noop() {
            Ok(_) => Ok(Duration::new(0, 0)), // FIXME: duration
            Err(e) => Err(PingError::FtpNoopError(e)),
        }
    }

    fn url(&self, resource: &str) -> Result<Url, GetError> {
        match self.base.join(resource) {
            Ok(u) => Ok(u),
            Err(e) => Err(GetError::UnparsableURL(e)),
        }
    }

    fn exists(&self, _resource: &str) -> Result<bool, GetError> {
        Err(GetError::Unimplemented)
    }

    fn get(&mut self, resource: &str, output: PathBuf) -> Result<Gotten, GetError> {
        let mimetype = "application/octet-stream";
        let source = match self
            .base
            .join(&format!("{}/{}", self.base.path(), resource))
        {
            Ok(s) => s,
            Err(e) => return Err(GetError::UnparsableURL(e)),
        };
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
        let mut tot: u64 = 0;
        let s = self.stream.retr(resource, |r| {
            while match r.read(&mut buf) {
                Ok(size) => match bw.write_all(&buf[0..size]) {
                    Ok(_) => {
                        tot += size as u64;
                        if size == 0 {
                            debug!("zero read after {tot} bytes");
                            false
                        } else {
                            true
                        }
                    }
                    Err(e) => {
                        warn!("error from write at {} bytes: {:?}", tot, e);
                        false
                    }
                },
                Err(e) => {
                    warn!("error from read after {} bytes: {:?}", tot, e);
                    false
                }
            } {
                debug!("read and wrote {tot} bytes for file {resource}");
            }
            Ok(())
        });
        if s.is_err() {
            let e = s.unwrap_err();
            warn!("error on file {resource}: {:?}", e);
            return Err(GetError::RetrieveError(e));
        }

        Ok(Gotten::new(
            mimetype,
            resource,
            source,
            output.to_path_buf(),
            tot,
        ))
    }

    fn remote_addr(&self) -> SocketAddr {
        self.remote
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::net::ToSocketAddrs;

    // a public server which might be used, ftp.gnu.org
    const FTPSERVER: &str = "209.51.188.20";
    const MOCK_RESOURCE: &str = "README";

    // a local server
    // const FTPSERVER: &str = "sopa.coo";
    // const MOCK_RESOURCE: &str = "README";

    fn mock_url() -> Url {
        let u = format!("ftp://{}/", FTPSERVER);
        Url::parse(&u).unwrap()
    }

    fn mock_resource_url(rsrc: &str) -> Url {
        let u = mock_url();
        let u = u.join(&format!("gnu/{rsrc}")).unwrap();
        eprintln!("CHECK A: {u}");
        u
    }

    #[test]
    fn test_mock_resource_url() {
        // intentionally double coded
        let expect = Url::parse("ftp://209.51.188.20/gnu/README").unwrap();
        assert_eq!(expect, mock_resource_url("README"));
    }

    fn mock() -> Ftp {
        Ftp::new(&mock_url(), None).unwrap()
    }

    #[test]
    fn test_connect() {
        let m = mock();
        assert_eq!(mock_url(), m.base);
        connect(m.remote, &m.base, &m.creds).unwrap();
    }

    #[test]
    fn cwd() {
        let dir = "/gnu";
        let base = format!("ftp://{}{}", FTPSERVER, dir);
        let base = Url::parse(&base).unwrap();
        let mut ftp = Ftp::new(&base, None).unwrap();
        assert_eq!(dir, ftp.stream.pwd().unwrap());
    }

    #[test]
    fn ping() {
        let mut m = mock();
        m.ping().unwrap();
    }

    #[test]
    fn remote_addr() {
        let m = mock();
        let ss = format!("{}:21", FTPSERVER);
        let sa = ss.to_socket_addrs().unwrap().next().unwrap();
        assert_eq!(sa, m.remote_addr());
    }

    #[test]
    fn get() {
        let mut m = mock();
        let path = PathBuf::from("/dev/null");
        let got = m.get(MOCK_RESOURCE, path).unwrap();
        assert_eq!(MOCK_RESOURCE, got.resource);
    }

    #[test]
    fn not_found() {
        let mut m = mock();
        let path = PathBuf::from("/dev/null");
        let fail = m.get("asdfasfdasfd", path);
        assert!(fail.is_err())
    }

    #[test]
    fn validation() {
        let mut m = mock();
        let mut t = env::temp_dir();
        t.push("reflector-ftp-validation-test");
        fs::create_dir_all(&t).expect("failed to create temp directory");
        t.push(MOCK_RESOURCE);
        if t.exists() {
            fs::remove_file(&t).unwrap();
        }
        let got = m.get(MOCK_RESOURCE, t.clone()).unwrap();
        got.validate().unwrap();
        assert_eq!(mock_resource_url(MOCK_RESOURCE), got.source);
        fs::remove_file(&t).unwrap();
    }

    #[test]
    fn list() {
        let mut m = mock();
        let l = m.listing().unwrap();
        assert!(l.len() > 2, "something in the listing");
        assert!(l[0].len() > 2, "first listing");
    }
}
