use super::*;
use chrono::{DateTime, Utc};
use std::ffi::{OsStr, OsString};

#[derive(Clone, Debug)]
pub struct Identity {}

impl Identity {
    pub fn new() -> Identity {
        Identity {}
    }
}

impl Default for Identity {
    fn default() -> Self {
        Identity::new()
    }
}

impl PathMaker for Identity {
    fn time_to_filename(&self, time: &DateTime<Utc>) -> OsString {
        let s = time.to_rfc3339();
        OsString::from(s)
    }

    fn filename_to_time(&self, filename: &OsStr) -> Result<DateTime<Utc>, PathMakerError> {
        match filename.to_str() {
            Some(f) => match DateTime::parse_from_rfc3339(f) {
                Ok(d) => Ok(d.into()),
                Err(e) => Err(TimeParseError(f.to_string(), e)),
            },
            None => Err(NoFileNameErr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn dogfood() {
        let p = Identity::new();
        let t = Utc::now();
        assert_eq!(t, p.filename_to_time(&p.time_to_filename(&t)).unwrap());
    }
}
