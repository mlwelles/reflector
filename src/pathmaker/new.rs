use super::{Identity, SDO}; // trait objects
use super::{NoNameErr, PathMaker, PathMakerError, UnknownName};

pub fn new(s: &str) -> Result<Box<dyn PathMaker>, PathMakerError> {
    match s {
        "SDO" => Ok(Box::new(SDO::default())),
        "identity" => Ok(Box::new(Identity::default())),
        "" => Err(NoNameErr),
        _ => Err(UnknownName(s.to_string())),
    }
}
