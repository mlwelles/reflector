use super::*;

// https://en.wikipedia.org/wiki/Geostationary_Operational_Environmental_Satellite
// example URLs:
//   ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor/ABI_TrueColor_20231014_1500z.png
//   ftp://ftp.nnvl.noaa.gov/GOES/MERGED_TrueColor/MERGED_TrueColor_20231014_1510z.png
//   ftp://ftp.nnvl.noaa.gov/GOES/WST_TrueColor/WST_TrueColor_20231014_1510z.png
// these don't work:
//   ftp://ftp.nnvl.noaa.gov/GOES/HIMAWARI/simplecontrast/HIMDAILY2023-10-14-112524.JPG

// FIXME: this name is too general -- there are tons of GOES satellites which work in
// different ways
#[derive(Clone)]
pub struct GOES {
    pub prefix: String,
}

const SUFFIX: &str = "z.png";

impl GOES {
    fn new(prefix: &str) -> GOES {
        let prefix = String::from(prefix);
        GOES { prefix }
    }
}

impl Default for GOES {
    fn default() -> GOES {
        GOES::new("")
    }
}

impl PathMaker for GOES {
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
    fn goes_dogfood() {
        let p = GOES::new("testing_");
        let t = Utc::now();
        let f = p.time_to_filename(&t);
        let tt = p.filename_to_time(&f).unwrap();
        assert_eq!(f, p.time_to_filename(&tt));
    }
}
