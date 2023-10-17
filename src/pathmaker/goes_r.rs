// https://en.wikipedia.org/wiki/Geostationary_Operational_Environmental_Satellite
// https://www.goes-r.gov/downloads/resources/documents/GOES-RSeriesDataBook.pdf
// example URLs:
//   ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor/ABI_TrueColor_20231014_1500z.png
//   ftp://ftp.nnvl.noaa.gov/GOES/MERGED_TrueColor/MERGED_TrueColor_20231014_1510z.png
//   ftp://ftp.nnvl.noaa.gov/GOES/WST_TrueColor/WST_TrueColor_20231014_1510z.png

use super::*;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct GoesR {
    pub prefix: String,
}

const SUFFIX: &str = "z.png";
const TIME_FMT: &str = "%Y%m%d_%H%M";

impl GoesR {
    fn new(prefix: &str) -> GoesR {
        let prefix = String::from(prefix);
        GoesR { prefix }
    }
}

impl Default for GoesR {
    fn default() -> GoesR {
        GoesR::new("")
    }
}

impl PathMaker for GoesR {
    fn time_to_filename(&self, time: &DateTime<Utc>) -> OsString {
        format!("{}{}{}", self.prefix, time.format(TIME_FMT), SUFFIX).into()
    }

    fn filename_to_time(&self, filename: &OsStr) -> Result<DateTime<Utc>, PathMakerError> {
        let filename = filename.to_str().unwrap();
        // remove prefix, if any
        let filename = filename.strip_prefix(&self.prefix).unwrap_or(filename);
        // ditto suffix
        let filename = filename.strip_suffix(SUFFIX).unwrap_or(filename);

        match filename.len() {
            l if l < 13 => return Err(FilenameTooShort(filename.to_string())),
            l if l > 13 => return Err(FilenameTooLong(filename.to_string())),
            _ => (),
        }

        // FIXME: the following is god-awful

        let year: i32 = match filename[0..4].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableYear(filename[0..3].to_string())),
        };
        let mon: u32 = match filename[4..6].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableMonth(filename[4..5].to_string())),
        };
        let day: u32 = match filename[6..8].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableMonth(filename[4..5].to_string())),
        };
        match filename.chars().nth(8) {
            Some('_') => (),
            None => return Err(MissingSeparator('_', filename.to_string())),
            _ => return Err(MysteryError("".to_string())),
        }
        let hour: u32 = match filename[9..11].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableHour(filename[9..11].to_string())),
        };
        let min: u32 = match filename[11..13].parse() {
            Ok(x) => x,
            Err(_) => return Err(UnparsableMinute(filename[11..13].to_string())),
        };
        make_utc(year, mon, day, hour, min, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn time_to_filename() {
        let p = GoesR::default();
        let t = Utc.with_ymd_and_hms(2023, 09, 23, 12, 0, 0).unwrap();
        let expect = "20230923_1200z.png";
        assert_eq!(OsString::from(expect), p.time_to_filename(&t));
    }

    #[test]
    fn filename_to_time() {
        let p = GoesR::new("ABI_TrueColor_");
        let f = OsString::from("ABI_TrueColor_20231014_1500z.png");
        let expect = Utc.with_ymd_and_hms(2023, 10, 14, 15, 0, 0).unwrap();
        assert_eq!(expect, p.filename_to_time(&f).unwrap());
    }

    #[test]
    fn goes16_dogfood() {
        let p = GoesR::new("testing_");
        let t = Utc::now();
        let f = p.time_to_filename(&t);
        let tt = p.filename_to_time(&f).unwrap();
        assert_eq!(f, p.time_to_filename(&tt));
    }
}
