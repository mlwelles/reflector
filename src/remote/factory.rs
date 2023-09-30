use super::RemoteClient;
// use std::convert::From;
use url::Url;

#[derive(PartialEq, Eq, Debug)]
pub enum RCFactoryError {
    Unimplemented,
}

pub fn from_url(_url: &Url) -> Result<Box<dyn RemoteClient>, RCFactoryError> {
    Err(RCFactoryError::Unimplemented)
}

// impl From<Url> for
