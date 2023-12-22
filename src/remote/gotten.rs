use std::cmp::Ordering;
use std::io;
use std::path::PathBuf;
use url::Url;

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
    OutputMetadata(io::Error),
    Tempdir(io::Error),
    OutputLargerThanExpected(u64, u64),
    OutputSmallerThanExpected(u64, u64),
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
            return Err(OutputDoesNotExist);
        }
        let m = match self.output.metadata() {
            Ok(m) => m,
            Err(e) => return Err(OutputMetadata(e)),
        };
        let gs = m.len();
        let es = self.size;
        match gs.cmp(&es) {
            Ordering::Greater => Err(OutputLargerThanExpected(gs, es)),
            Ordering::Less => Err(OutputSmallerThanExpected(gs, es)),
            _ => Ok(()),
        }
    }

    pub fn valid(&self) -> bool {
        self.validate().is_ok()
    }
}
