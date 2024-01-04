//! Mirror configuration.

use serde::Deserialize;
use std::default::Default;
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
    pub seed_past_midnight: Option<u64>,
    pub loop_period: Option<u64>,
    pub flatten: Option<bool>,
}

impl SourceConfig {
    pub fn sdo() -> Self {
        Self {
            name: "Solar Data Observatory".to_string(),
            abbrev: "sdo".to_string(),
            remote: "https://sdo.gsfc.nasa.gov/assets/img/dailymov".to_string(),
            local: "/home/adam/tmp/sat/sdo".to_string(),
            pathmaker: "SDO _1024_0094.ogv".to_string(),
            flatten: Some(true),
            period: 24 * 60 * 60, // 24 hours, expressed as seconds
            seed_past_midnight: None,
            loop_period: Some(24 * 60 * 60 * 28), // 28 days
        }
    }

    pub fn goes_abi() -> Self {
        Self {
            name: "GOES ABI_TrueColor".to_string(),
            abbrev: "goesabi".to_string(),
            remote: "ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor".to_string(),
            local: "/home/adam/tmp/sat/abi_truecolor".to_string(),
            pathmaker: "GOES-R".to_string(),
            flatten: None,
            period: 5 * 60 * 10,
            seed_past_midnight: None,
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
        if s.is_empty() {
            return Err(SourceSearchError::EmptyName);
        }

        let mut mm: Vec<SourceConfig> = vec![];
        for src in Config::default().sources {
            if src.name == s || src.abbrev == s {
                mm.push(src);
            }
        }

        match mm.len() {
            0 => Err(SourceSearchError::NoMatchForName(s.to_string())),
            _ => Ok(Config { sources: mm }),
        }
    }
}

// TODO: From<Vec<String>> or some such

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror::Mirror;
    use crate::TimeRange;
    use std::time::{Duration, SystemTime};

    #[test]
    fn from_str() {
        Config::from_str("Solar Data Observatory").unwrap();
        Config::from_str("sdo").unwrap();
        // Config::from_str("SDO").unwrap();
    }

    #[test]
    fn sdo() {
        let s = SourceConfig::sdo();
        let sd = Mirror::try_from(s).unwrap();
        let now = SystemTime::now();

        let lr = sd.loop_range();
        let expect = TimeRange::new(now - sd.loop_period, now).unwrap();
        assert!(lr.equal_by_seconds(&expect));
        // hardcoded sanity check
        let lp = Duration::new(28 * 24 * 60 * 60, 0);
        assert_eq!(sd.loop_period, lp);
        let expect = TimeRange::new(now - lp, now).unwrap();
        assert!(lr.equal_by_seconds(&expect), "expect {} == {}", lr, expect);

        let mut cap = sd.loop_captures();
        assert!(!cap.is_empty());
        assert!(
            cap.len_all() > 20,
            "lenth {} doesn't meet reasonable minimum captures",
            cap.len_all()
        );

        let fc = cap.next().unwrap(); // first capture
        assert!(fc.valid())
    }
}
