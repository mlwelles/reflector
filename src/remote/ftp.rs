use super::*;
use std::path::PathBuf;
use std::time::Duration;
use std::{io, net::SocketAddr};
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

#[derive(Debug)]
pub enum FtpConnectError {
    SocketError(io::Error),
    LoginError(FtpError),
}
use FtpConnectError::*;

pub struct Ftp {
    pub base: Url,
    pub stream: Option<FtpStream>,
    pub creds: FtpCredentials,
    remote: SocketAddr,
}

impl Ftp {
    pub fn new(base: Url, creds: Option<FtpCredentials>) -> Result<Ftp, FtpConnectError> {
        let remote = match base.socket_addrs(|| None) {
            Ok(a) => a[0],
            Err(e) => return Err(SocketError(e)),
        };
        let creds = match creds {
            Some(c) => c,
            None => FtpCredentials::default(),
        };
        let stream = None;
        Ok(Ftp {
            base,
            stream,
            creds,
            remote,
        })
    }
}

impl RemoteClient for Ftp {
    fn ping(&self) -> Result<Duration, PingError> {
        Err(PingError::Unimplemented)
    }

    fn connect(&self) -> Result<(), ConnectError> {
        let mut stream = FtpStream::connect(&self.remote).unwrap();
        match stream.login(&self.creds.user, &self.creds.password) {
            Err(e) => return Err(ConnectError::FtpLoginErr(e)),
            _ => Ok(()),
        }
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

    fn mock() -> Ftp {
        let u = Url::parse("ftp://ftp.debian.org/").unwrap();
        Ftp::new(u, None).unwrap()
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
        let path = PathBuf::from("/dev/null");
        let got = m.get(rsrc, &path).unwrap();
        assert_eq!(rsrc, got.resource);
        let fail = m.get("asdfasfdasfd", &path);
        assert!(fail.is_err())
    }
}
