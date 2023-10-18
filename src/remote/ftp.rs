use super::*;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use suppaftp::{FtpError, FtpStream};
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
    pub stream: Option<FtpStream>,
    pub creds: FtpCredentials,
    remote: SocketAddr,
}

impl Ftp {
    fn connect(&mut self) -> Result<(), ConnectError> {
        eprintln!("connecting to remote...");
        let mut stream = match FtpStream::connect_timeout(self.remote, Duration::new(10, 0)) {
            Ok(s) => s,
            Err(e) => return Err(ConnectError::FtpConnectErr(e)),
        };
        eprintln!("logging in...");
        match stream.login(&self.creds.user, &self.creds.password) {
            Err(e) => Err(ConnectError::FtpLoginErr(e)),
            _ => {
                self.stream = Some(stream);
                Ok(())
            }
        }
    }

    pub fn new(base: Url, creds: Option<FtpCredentials>) -> Result<Ftp, ConnectError> {
        let remote = match base.socket_addrs(|| None) {
            Ok(a) => a[0],
            Err(e) => return Err(ConnectError::SocketError(e)),
        };
        let creds = match creds {
            Some(c) => c,
            None => FtpCredentials::default(),
        };
        let stream = None;
        let mut ftp = Ftp {
            base,
            stream,
            creds,
            remote,
        };
        match ftp.connect() {
            Ok(_) => Ok(ftp),
            Err(e) => Err(e),
        }
    }
}

impl RemoteClient for Ftp {
    fn ping(&self) -> Result<Duration, PingError> {
        Err(PingError::Unimplemented)
    }

    fn get(&self, _resource: &str, _output: &PathBuf) -> Result<Gotten, GetError> {
        Err(GetError::Unimplemented)
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

    fn connected_mock() -> Ftp {
        let mut m = mock();
        m.connect().unwrap();
        m
    }

    #[test]
    fn connect() {
        let mut m = mock();
        m.connect().unwrap();
    }

    #[test]
    fn ping() {
        let m = connected_mock();
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
        let m = connected_mock();
        let rsrc = "README";
        let path = PathBuf::from("/dev/null");
        let got = m.get(rsrc, &path).unwrap();
        assert_eq!(rsrc, got.resource);
        let fail = m.get("asdfasfdasfd", &path);
        assert!(fail.is_err())
    }
}
