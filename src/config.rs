//! Mirror configuration.

use serde::Deserialize;
use std::default::Default;
use std::env::Args;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sources: Vec<SourceConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub abbrev: String,
    pub remote: String,
    pub local: String,
    pub pathmaker: String,
    /// period between captures
    pub period: u64,
    /// seconds after midnight to offset all times
    pub offset: Option<u64>,
    pub loop_period: Option<u64>,
    pub flatten: Option<bool>,
}

impl SourceConfig {
    pub fn sdo() -> Self {
        Self {
            name: "Solar Data Observatory".to_string(),
            abbrev: "sdo".to_string(),
            remote: "https://sdo.gsfc.nasa.gov/assets/img/dailymov".to_string(),
            local: "/net/sopa/winshare/sat/sdo".to_string(),
            pathmaker: "SDO _1024_0094.ogv".to_string(),
            flatten: Some(true),
            period: 24 * 60 * 60, // 24 hours, expressed as seconds
            offset: Some((21 * 60 * 60) + (5 * 60)), // 21:05 -- this would work if midnight was defined at UTC
            // offset: Some((23 * 60 * 60) + (5 * 60)), // 21:05 + TZ is more than 24, get as close as possible gah
            loop_period: Some(24 * 60 * 60 * 28), // 28 days
        }
    }

    pub fn goes_abi() -> Self {
        Self {
            name: "GOES ABI_TrueColor".to_string(),
            abbrev: "goesabi".to_string(),
            remote: "ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor".to_string(),
            local: "/net/sopa/winshare/sat/abi_truecolor".to_string(),
            pathmaker: "GOES-R".to_string(),
            flatten: None,
            period: 5 * 60 * 10,
            offset: None,
            loop_period: Some(24 * 60 * 60), // 24 hours
        }
    }
}

impl fmt::Display for SourceConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "<SourceConfig {} l: {} r: {}>",
            self.name, self.local, self.remote
        )
    }
}

impl Default for Config {
    fn default() -> Config {
        let srcs = vec![SourceConfig::sdo(), SourceConfig::goes_abi()];
        Config { sources: srcs }
    }
}

#[derive(Debug)]
pub enum SourceSearchError {
    NotImplemented,
    NoMatchForName(String),
    EmptyName,
}

impl FromStr for Config {
    type Err = SourceSearchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // crate non_empty_string ?
        if s.is_empty() {
            return Err(SourceSearchError::EmptyName);
        }

        let mut mm: Vec<SourceConfig> = vec![];
        for src in Config::default().sources {
            // FIXME: lowercase too?
            if src.name == s || src.abbrev == s {
                mm.push(src);
            }
            // FIXME: lowercase too?
        }

        match mm.len() {
            0 => Err(SourceSearchError::NoMatchForName(s.to_string())),
            _ => Ok(Config { sources: mm }),
        }
    }
}

// should this be a tryfrom?
impl From<Args> for Config {
    fn from(mut args: Args) -> Config {
        let default = Config::default();
        match args.len() {
            1 => default,
            2 => {
                if let Some(first) = args.nth(1) {
                    match Config::from_str(&first) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("no matches for {}: {:?}", first, e);
                            default
                        }
                    }
                } else {
                    eprintln!("arg counting logic fail");
                    default
                }
            }
            _ => {
                eprintln!("unimplemented");
                default
            }
        }
    }
}

// TODO: From<Vec<String>> or some such

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror::Mirror;
    use crate::{display_systime, CaptureMissing, TimeRange};
    use std::time::{Duration, SystemTime};

    #[test]
    fn from_str() {
        Config::from_str("Solar Data Observatory").unwrap();
        Config::from_str("sdo").unwrap();
        // Config::from_str("SDO").unwrap();
    }

    fn assert_valid_mirror(m: &Mirror) {
        let now = SystemTime::now();
        let lr = m.loop_range();
        let expect = TimeRange::new(now - m.loop_period, now).unwrap();
        assert!(lr.equal_by_seconds(&expect), "expect {} == {}", lr, expect);
        let cap = m.loop_captures();
        assert!(
            cap.len_all() > 20,
            "length {} doesn't meet reasonable minimum captures",
            cap.len_all()
        );

        // assert the latest in the capture missing list exists
        if let Some(c) = cap.missing.back() {
            assert!(
                c.time < now,
                "{} vs {}",
                display_systime(c.time),
                display_systime(now)
            );
            assert!(
                m.exists(&c.resource).unwrap(),
                "{c} at {} doesn't exist",
                m.url(&c.resource).unwrap(),
            );
        }

        // assert the latest in the capture list exists
        // if let Some(c) = cap.list.back() {
        //     assert!(m.remote_client.exists(c));
        // }
    }

    fn assert_has_captures(m: &Mirror) {
        let mut cap = m.loop_captures();
        assert!(!cap.is_empty());
        let fc = cap.next().unwrap(); // first capture
        assert!(fc.valid(), "first capture is valid");
    }

    fn assert_missing(m: &mut Mirror, miss: &CaptureMissing) {
        assert!(m.local.get(&miss.path).is_err());
        assert!(!miss.resource.is_empty());
    }

    fn assert_alpha_omega(mut m: &mut Mirror) {
        let mut cl = m.loop_captures();
        let cap = cl.next().unwrap();
        assert!(cap.valid(), "capture valid");
        assert!(cap.path.exists(), "first capture path exists");

        if let Some(miss) = cl.missing.pop_front() {
            assert_missing(&mut m, &miss);
            m.get_missing(&miss).expect("get_missing(front) results");
        }

        if let Some(miss) = cl.missing.pop_back() {
            assert_missing(&mut m, &miss);
            m.get_missing(&miss).expect("get_missing(back) results");
        }
    }

    #[test]
    fn sdo() {
        let s = SourceConfig::sdo();
        let mut m = Mirror::try_from(s).unwrap();
        assert_valid_mirror(&m);

        // hardcoded sanity check
        let lp = Duration::new(28 * 24 * 60 * 60, 0);
        assert_eq!(m.loop_period, lp);

        // unsafely assume *something* is in our repository
        assert_has_captures(&m);

        assert_alpha_omega(&mut m);
    }

    #[test]
    fn abi_truecolor() {
        let s = SourceConfig::sdo();
        let mut m = Mirror::try_from(s).unwrap();
        assert_valid_mirror(&m);

        // FIXME: unsafe assumption
        assert_has_captures(&m);

        assert_alpha_omega(&mut m);
    }
}
