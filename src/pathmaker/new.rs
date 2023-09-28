use super::SDO;
use super::{NoNameErr, PathMaker, PathMakerError, UnknownName};

pub fn new(s: String) -> Result<&dyn PathMaker, PathMakerError> {
    match s {
        "SDO" => {
            // FIXME: use default trait
            let sdo = SDO {
                suffix: "".to_string(),
            };
            Ok(&sdo)
        }
        "" => Err(NoNameErr),
        _ => Err(UnknownName(s.to_string())),
    }
}
