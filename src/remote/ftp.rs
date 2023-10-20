use super::*;
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
    eprintln!("connecting to remote...");
    let mut stream = match FtpStream::connect_timeout(remote, Duration::new(10, 0)) {
        Ok(s) => s,
        Err(e) => return Err(ConnectError::FtpConnectErr(e)),
    };
    eprintln!("logging in...");
    if let Err(e) = stream.login(&creds.user, &creds.password) {
        return Err(ConnectError::FtpLoginErr(e));
    }
    let dir = base.path();
    if dir.len() > 1 {
        eprintln!("changing dir to {}...", dir);
        if let Err(e) = stream.cwd(dir) {
            return Err(ConnectError::FtpCwdErr(e));
        }
    }
    eprintln!("setup FTP connection to {}", base.as_str());
    Ok(stream)
}

impl Ftp {
    pub fn new(base: Url, creds: Option<FtpCredentials>) -> Result<Ftp, ConnectError> {
        let remote = match base.socket_addrs(|| None) {
            Ok(a) => a[0],
            Err(e) => return Err(ConnectError::SocketError(e)),
        };
        let creds = creds.unwrap_or(FtpCredentials::default());
        match connect(remote.clone(), &base, &creds) {
            Ok(stream) => Ok(Ftp {
                base,
                stream,
                creds,
                remote,
            }),
            Err(e) => Err(e),
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

    fn get(&mut self, resource: &str, output: &PathBuf) -> Result<Gotten, GetError> {
        let mimetype = "application/octet-stream";
        let source = match self.base.join(resource) {
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
                Ok(size) => match bw.write_all(&buf) {
                    Ok(_) => {
                        tot += size as u64;
                        if size < BUFSIZE {
                            eprintln!("short read after {tot} bytes");
                            false
                        } else {
                            true
                        }
                    }
                    Err(e) => {
                        eprintln!("error from write at {} bytes: {:?}", tot, e);
                        false
                    }
                },
                Err(e) => {
                    eprintln!("error from read after {} bytes: {:?}", tot, e);
                    false
                }
            } {
                eprintln!("read and wrote {tot} bytes");
            }
            Ok(())
        });
        if s.is_err() {
            let e = s.unwrap_err();
            eprintln!("error {:?}", e);
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

    const FTPSERVER: &str = "209.51.188.20";

    fn mock() -> Ftp {
        let u = format!("ftp://{}/", FTPSERVER);
        let u = Url::parse(&u).unwrap();
        Ftp::new(u, None).unwrap()
    }

    #[test]
    fn test_connect() {
        let m = mock();
        connect(m.remote, &m.base, &m.creds).unwrap();
    }

    #[test]
    fn cwd() {
        let dir = "/gnu";
        let base = format!("ftp://{}{}", FTPSERVER, dir);
        let base = Url::parse(&base).unwrap();
        let mut ftp = Ftp::new(base, None).unwrap();
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
        let server = format!("{}:21", FTPSERVER);
        let sa: SocketAddr = server.parse().unwrap();
        assert_eq!(sa, m.remote_addr())
    }

    #[test]
    fn get() {
        let mut m = mock();
        let rsrc = "README";
        let path = PathBuf::from("/dev/null");
        let got = m.get(rsrc, &path).unwrap();
        assert_eq!(rsrc, got.resource);
    }

    #[test]
    fn not_found() {
        let mut m = mock();
        let path = PathBuf::from("/dev/null");
        let fail = m.get("asdfasfdasfd", &path);
        assert!(fail.is_err())
    }
}
