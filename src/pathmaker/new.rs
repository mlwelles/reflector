use super::{GoesR, Identity, Sdo}; // trait objects
use super::{NoNameErr, PathMaker, PathMakerError, UnknownName};

pub fn new(s: &str) -> Result<Box<dyn PathMaker>, PathMakerError> {
    match s {
        "SDO" => Ok(Box::<Sdo>::default()),
        "GOES-R" => Ok(Box::<GoesR>::default()),
        "identity" => Ok(Box::<Identity>::default()),
        "" => Err(NoNameErr),
        _ => Err(UnknownName(s.to_string())),
    }
}
