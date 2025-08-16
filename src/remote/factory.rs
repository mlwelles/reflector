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

impl TryFrom<Url> for Box<dyn RemoteClient> {
    type Error = RCFactoryError;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        from_url(&url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_url() {
        let host = "gnu.org:80";
        let u = Url::parse(&format!("http://{}/", host)).unwrap();
        let mut rc = from_url(&u).unwrap();
        rc.ping().unwrap();
    }

    #[test]
    fn invalid() {
        let u = Url::parse("gopher://gopher.quux.org:70/1/").unwrap();
        let r = from_url(&u);
        assert!(r.is_err());
        assert!(match r {
            Err(NoHandlerForScheme(s)) => s == "gopher".to_string(),
            _ => false,
        });
    }

    // FIXME: not sure how to write this test
    // #[test]
    // fn try_from() {
    //     let u = Url::parse("http://www.google.com/").unwrap();
    //     let r  = Box<RemoteClient>::try_from(u);
    // }
}
