use std::io;
use std::path::PathBuf;
use url::Url;

#[allow(dead_code)]
pub struct Gotten {
    pub mimetype: String,
    pub resource: String,
    pub source: Url,
    pub output: PathBuf,
    pub size: u64,
}

#[derive(Debug)]
pub enum GottenValidation {
    AllIsWell,
    OutputDoesNotExist,
    Tempdir(Box<io::Error>),
}
use GottenValidation::*;

impl Gotten {
    pub fn new(mimetype: &str, resource: &str, source: Url, output: PathBuf, size: u64) -> Gotten {
        let mimetype = mimetype.to_string();
        let resource = resource.to_string();
        Gotten {
            mimetype,
            resource,
            source,
            output,
            size,
        }
    }

    pub fn validate(&self) -> Result<(), GottenValidation> {
        if !self.output.is_file() {
            Err(OutputDoesNotExist)
        } else {
            Ok(())
        }
    }

    pub fn valid(&self) -> bool {
        self.validate().is_ok()
    }
}
