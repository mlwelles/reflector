// https://en.wikipedia.org/wiki/Geostationary_Operational_Environmental_Satellite
// https://www.goes-r.gov/downloads/resources/documents/GOES-RSeriesDataBook.pdf
// example URLs:
//   ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor/ABI_TrueColor_20231014_1500z.png
//   ftp://ftp.nnvl.noaa.gov/GOES/MERGED_TrueColor/MERGED_TrueColor_20231014_1510z.png
//   ftp://ftp.nnvl.noaa.gov/GOES/WST_TrueColor/WST_TrueColor_20231014_1510z.png

use super::*;

#[derive(Clone)]
pub struct GoesR {
    pub prefix: String,
}

const SUFFIX: &str = "z.png";

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
        format!("{}{}{}", self.prefix, time.format("%Y%m%d_%h%m"), SUFFIX).into()
    }

    fn filename_to_time(&self, filename: &OsStr) -> Result<DateTime<Utc>, PathMakerError> {
        Err(Unimplemented)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goes16_dogfood() {
        let p = GoesR::new("testing_");
        let t = Utc::now();
        let f = p.time_to_filename(&t);
        let tt = p.filename_to_time(&f).unwrap();
        assert_eq!(f, p.time_to_filename(&tt));
    }
}
