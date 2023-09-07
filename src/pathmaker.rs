#[derive(Debug, PartialEq, Eq)]
pub enum PathMaker {
    GOES,
    SDO,
}
use PathMaker::*;

#[derive(Debug, PartialEq, Eq)]
pub enum PathMakerError {
    UnknownName,
}
use PathMakerError::*;

impl PathMaker {
    pub fn new(s: &str) -> Result<PathMaker, PathMakerError> {
        match s {
            "GOES" => Ok(GOES),
            "SDO" => Ok(SDO),
            _ => Err(UnknownName),
        }
    }

    // FIXME: surely there is a macro for this
    pub fn dup(&self) -> PathMaker {
        match self {
            &GOES => GOES,
            &SDO => SDO,
        }
    }
}
