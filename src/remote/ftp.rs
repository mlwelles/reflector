use super::*;
use std::io;
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

pub enum FtpConnectError {
    SocketError(io::Error),
    LoginError(FtpError),
}
use FtpConnectError::*;

pub struct Ftp {
    pub base: Url,
    pub stream: FtpStream,
    pub creds: FtpCredentials,
}

impl Ftp {
    pub fn new(base: Url, creds: Option<FtpCredentials>) -> Result<Ftp, FtpConnectError> {
        let sa = match base.socket_addrs(|| None) {
            Ok(a) => a[0],
            Err(e) => return Err(SocketError(e)),
        };
        let creds = match creds {
            Some(c) => c,
            None => FtpCredentials::default(),
        };
        let mut stream = FtpStream::connect(sa).unwrap();
        match stream.login(&creds.user, &creds.password) {
            Err(e) => return Err(LoginError(e)),
            _ => (),
        }
        Ok(Ftp {
            base,
            stream,
            creds,
        })
    }
}
