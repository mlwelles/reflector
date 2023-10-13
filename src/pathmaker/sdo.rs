use super::*;
use chrono::offset::LocalResult;
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
        match Utc.with_ymd_and_hms(year, mon, day, 0, 0, 0) {
            LocalResult::Single(t) => Ok(t),
            LocalResult::Ambiguous(..) => Err(AmbiguousTimeError(filename.to_string())),
            LocalResult::None => Err(WithTimeError(filename.to_string(), year, mon, day)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn time_to_filename() {
        let p = SDO::default();
        let t = Utc.with_ymd_and_hms(2020, 11, 10, 0, 1, 32).unwrap();
        let expect = format!("{}", t.format("%Y%m%d"));
        assert_eq!(OsString::from(expect), p.time_to_filename(&t));
    }

    #[test]
    fn dogfood() {
        let p = SDO::default();
        let t = Utc::now();
        let f = p.time_to_filename(&t);
        let tt = p.filename_to_time(&f).unwrap();
        assert_eq!(f, p.time_to_filename(&tt));
    }
}
