use std::path::PathBuf;
use url::Url;

#[allow(dead_code)]
pub struct Gotten {
    pub mimetype: String,
    pub resource: String,
    pub source: Url,
    pub output: PathBuf,
    // remote: SocketAddr,
}

#[derive(Debug, PartialEq)]
pub enum Validation {
    AllIsWell,
    OutputDoesNotExist,
}

impl Gotten {
    pub fn new(mimetype: &str, resource: &str, source: Url, output: PathBuf) -> Gotten {
        let mimetype = mimetype.to_string();
        let resource = resource.to_string();
        Gotten {
            mimetype,
            resource,
            source,
            output,
        }
    }

    pub fn validation(&self) -> Validation {
        if !self.output.is_file() {
            Validation::OutputDoesNotExist
        } else {
            Validation::AllIsWell
        }
    }

    pub fn valid(&self) -> bool {
        self.validation() == Validation::AllIsWell
    }
}
