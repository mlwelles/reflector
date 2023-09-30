use super::*;
use chrono::{DateTime, TimeZone, Utc};
use std::ffi::OsStr;

// SDO are 24 hour captures; example URLs
// https://sdo.gsfc.nasa.gov/assets/img/dailymov/2023/09/23/20230923_1024_0094.ogv
// https://sdo.gsfc.nasa.gov/assets/img/dailymov/2023/09/23/20230923_1024_0131.ogv
// https://sdo.gsfc.nasa.gov/assets/img/dailymov/2023/09/23/20230923_588_SDO_VO3.mp4
#[derive(Clone)]
pub struct SDO {
    pub suffix: String,
}

impl Default for SDO {
    fn default() -> SDO {
        let suffix = String::from("");
        SDO { suffix }
    }
}

impl PathMaker for SDO {
    fn time_to_filename(&self, time: &DateTime<Utc>) -> OsString {
        format!("{}{}", time.format("%Y%m%d"), self.suffix).into()
    }

    fn filename_to_time(&self, filename: &OsStr) -> Result<DateTime<Utc>, PathMakerError> {
        let filename = filename.to_str().unwrap();
        let base = match filename.strip_suffix(&self.suffix) {
            Some(base) => base,
            None => filename,
        };
        // we expect YYYYMMDD
        if base.len() != 8 {
            return Err(FilenameTooShort(base.to_string()));
        }
        let year: i32 = match base[0..3].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableYear(base[0..3].to_string())),
        };
        let mon: u32 = match base[4..5].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableMonth(base[4..5].to_string())),
        };
        let day: u32 = match base[6..7].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableMonth(base[4..5].to_string())),
        };
        Ok(Utc.with_ymd_and_hms(year, mon, day, 0, 0, 0).unwrap())
    }
}
