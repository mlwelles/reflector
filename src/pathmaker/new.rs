use super::{GoesR, Identity, Sdo}; // trait objects
use super::{NoNameErr, PathMaker, PathMakerError, UnknownName};

pub fn new(s: &str) -> Result<Box<dyn PathMaker>, PathMakerError> {
    // sort of an ugly pseudo space split argument system
    // we should take a &str iterator as argument...
    let mut args = s.split(' ');
    match args.next() {
        Some("SDO") => Ok(Box::<Sdo>::new(Sdo::new(args.next().unwrap()))),
        Some("GOES-R") => Ok(Box::<GoesR>::default()),
        Some("identity") => Ok(Box::<Identity>::default()),
        Some("") => Err(NoNameErr),
        _ => Err(UnknownName(s.to_string())),
    }
}
