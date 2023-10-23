use super::*;
// use std::convert::From;
use url::Url;

#[derive(Debug)]
pub enum RCFactoryError {
    Unimplemented,
    NoHandlerForScheme(String),
    FtpError(ConnectError),
}
use RCFactoryError::*;

pub fn from_url(url: &Url) -> Result<Box<dyn RemoteClient>, RCFactoryError> {
    match url.scheme() {
        "http" | "https" => Ok(Box::new(Http::new(url))),
        "ftp" => match Ftp::new(url, None) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => Err(FtpError(e)),
        },
        x => Err(NoHandlerForScheme(x.to_string())),
    }
}
