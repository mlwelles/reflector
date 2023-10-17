// SDO are 24 hour captures; example URLs
// https://sdo.gsfc.nasa.gov/assets/img/dailymov/2023/09/23/20230923_1024_0094.ogv
// https://sdo.gsfc.nasa.gov/assets/img/dailymov/2023/09/23/20230923_1024_0131.ogv
// https://sdo.gsfc.nasa.gov/assets/img/dailymov/2023/10/13/20231013_588_SDO_VO2.mp4

use super::*;
#[allow(unused_imports)]
use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;
use std::ffi::OsStr;

#[derive(Clone)]
pub struct Sdo {
    pub suffix: String,
}

impl Sdo {
    fn new(suffix: &str) -> Sdo {
        let suffix = String::from(suffix);
        Sdo { suffix }
    }
}

// default here is pretty useless but helpful to have for testing/mocking
impl Default for Sdo {
    fn default() -> Sdo {
        Sdo::new("")
    }
}

impl PathMaker for Sdo {
    fn time_to_filename(&self, time: &DateTime<Utc>) -> OsString {
        format!("{}{}", time.format("%Y/%m/%d/%Y%m%d"), self.suffix).into()
    }

    fn filename_to_time(&self, filename: &OsStr) -> Result<DateTime<Utc>, PathMakerError> {
        let filename = filename.to_str().unwrap();
        // remove suffix, if any
        let base = filename.strip_suffix(&self.suffix).unwrap_or(filename);

        // remove everything before the last slash
        let pre = Regex::new(r".*/(?<rem>[^/]+)$").unwrap();
        let base = pre.replace(base, "$rem");

        // we expect YYYYMMDD
        match base.len() {
            l if l < 8 => return Err(FilenameTooShort(base.to_string())),
            l if l > 8 => return Err(FilenameTooLong(base.to_string())),
            _ => (),
        }
        let year: i32 = match base[0..4].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableYear(base[0..3].to_string())),
        };
        let mon: u32 = match base[4..6].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableMonth(base[4..5].to_string())),
        };
        let day: u32 = match base[6..8].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableMonth(base[4..5].to_string())),
        };
        make_utc(year, mon, day, 0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_to_filename() {
        let p = Sdo::default();
        let t = Utc.with_ymd_and_hms(2023, 09, 23, 0, 1, 32).unwrap();
        let expect = "2023/09/23/20230923";
        assert_eq!(OsString::from(expect), p.time_to_filename(&t));

        let p = Sdo::new("_588_SDO_VO2.mp4");
        let expect = "2023/10/13/20231013_588_SDO_VO2.mp4";
        let t = Utc.with_ymd_and_hms(2023, 10, 13, 0, 0, 0).unwrap();
        assert_eq!(OsString::from(expect), p.time_to_filename(&t));
    }

    #[test]
    fn filename_to_time() {
        let p = Sdo::new("_suffix");
        let f = OsString::from("2023/12/31/20231231_suffix");
        let expect = Utc.with_ymd_and_hms(2023, 12, 31, 0, 0, 0).unwrap();
        assert_eq!(expect, p.filename_to_time(&f).unwrap());
    }

    #[test]
    fn sdo_dogfood() {
        let p = Sdo::new("_some_random.ogv");
        let t = Utc::now();
        let f = p.time_to_filename(&t);
        let tt = p.filename_to_time(&f).unwrap();
        assert_eq!(f, p.time_to_filename(&tt));
    }
}
