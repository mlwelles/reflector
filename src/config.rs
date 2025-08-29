//! Mirror configuration.

use log::{info, warn};
use serde::Deserialize;
use std::default::Default;
use std::env::Args;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct LoopCount(u8);

impl LoopCount {
    fn incr(&mut self) {
        self.0 += 1;
    }
}

impl Default for LoopCount {
    fn default() -> Self {
        Self(1)
    }
}

// note: be sure to update ../test/config.rs, specifically the serialized TOML representation,
// if anything other than field order changes
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub sources: SourceConfigs,
    pub verbose: bool,
    pub loops: LoopCount,
}

impl FromStr for Config {
    type Err = SourceSearchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match SourceConfig::from_str(s) {
            Ok(sc) => Ok(Self {
                sources: SourceConfigs::new(sc),
                ..Default::default()
            }),
            Err(e) => Err(e),
        }
    }
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
            local: "/net/sopa/scratch/sat/sdo".to_string(),
            pathmaker: "SDO _1024_0094.ogv".to_string(),
            flatten: Some(true),
            period: 24 * 60 * 60, // 24 hours, expressed as seconds
            offset: Some((21 * 60 * 60) + (5 * 60)), // 21:05 -- this would work if midnight was defined at UTC
            // offset: Some((23 * 60 * 60) + (5 * 60)), // 21:05 + TZ is more than 24, get as close as possible gah
            loop_period: Some(24 * 60 * 60 * 28), // 28 days
        }
    }

    pub fn sdo_0335() -> Self {
        let mut s = Self::sdo();
        s.pathmaker = "SDO _1024_0335.ogv".to_string();
        s.abbrev = "sdo_0335".to_string();
        s
    }

    pub fn goes_abi() -> Self {
        Self {
            name: "GOES ABI_TrueColor".to_string(),
            abbrev: "goesabi".to_string(),
            remote: "ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor".to_string(),
            local: "/net/sopa/scratch/sat/abi_truecolor".to_string(),
            pathmaker: "GOES-R".to_string(),
            flatten: None,
            period: 5 * 60 * 10, // eh?
            offset: None,
            loop_period: Some(24 * 60 * 60), // 24 hours
        }
    }
}

#[derive(Debug)]
pub enum SourceSearchError {
    NotImplemented,
    NoMatchForName(String),
    EmptyName,
}

impl FromStr for SourceConfig {
    type Err = SourceSearchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(SourceSearchError::EmptyName);
        }

        for src in SourceConfigs::default() {
            // FIXME: lowercase too?
            if src.name == s || src.abbrev == s {
                return Ok(src);
            }
        }

        Err(SourceSearchError::NoMatchForName(s.to_string()))
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

#[derive(Clone, Debug, serde::Deserialize)]
pub struct SourceConfigs(Vec<SourceConfig>);

impl SourceConfigs {
    fn new(sc: SourceConfig) -> Self {
        SourceConfigs(vec![sc])
    }

    fn empty() -> Self {
        SourceConfigs(vec![])
    }

    fn push(&mut self, c: SourceConfig) {
        self.0.push(c)
    }

    #[allow(dead_code)]
    fn len(self) -> usize {
        self.0.len()
    }
}

impl Default for SourceConfigs {
    fn default() -> Self {
        Self(vec![
            SourceConfig::sdo(),
            SourceConfig::sdo_0335(),
            SourceConfig::goes_abi(),
        ])
    }
}

impl IntoIterator for SourceConfigs {
    type Item = SourceConfig;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug)]
pub enum ConfigArgsError {
    NotImplemented,
    UnknownSource(String),
    NoSourcesFound,
    UnknownOption(String),
}

/// a very naive command line argument processor
impl TryFrom<Args> for Config {
    type Error = ConfigArgsError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let mut c = Config::default();
        match args.len() {
            1 => Ok(c),
            _ => {
                let mut sources = SourceConfigs::empty();
                for a in args.skip(1) {
                    if a.starts_with('-') {
                        match a.chars().next() {
                            Some('v') => c.verbose = true,
                            Some('l') => c.loops.incr(),
                            _ => return Err(ConfigArgsError::UnknownOption(a)),
                        };
                    } else {
                        match SourceConfig::from_str(&a) {
                            Ok(s) => {
                                info!("matched on {}", a);
                                sources.push(s);
                            }
                            Err(e) => {
                                warn!("no matches for {}: {:?}", a, e);
                                return Err(ConfigArgsError::UnknownSource(a));
                            }
                        }
                    }
                }

                let l = sources.clone().len();
                c.sources = sources;
                if l > 0 {
                    Ok(c)
                } else {
                    Err(ConfigArgsError::NoSourcesFound)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror::Mirror;
    use crate::{display_duration, display_systime, CaptureMissing, StandardTimeRange, TimeRange};
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
            cap.len_all() > 5,
            "length {} doesn't meet reasonable minimum captures",
            cap.len_all()
        );

        // assert the latest in the capture missing list exists
        if let Some(c) = cap.missing.back() {
            assert!(
                c.time < now,
                "{} vs {}",
                display_systime(&c.time),
                display_systime(&now)
            );
            assert!(
                m.exists(&c.resource).unwrap(),
                "{c} at {} doesn't exist at time {} (now {})",
                m.url(&c.resource).unwrap(),
                display_systime(&c.time),
                display_systime(&now)
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

    fn assert_alpha_omega(m: &mut Mirror) {
        let mut cl = m.loop_captures();
        let cap = cl.next().unwrap();
        assert!(cap.valid(), "capture valid");
        assert!(cap.path.exists(), "first capture path exists");

        if let Some(miss) = cl.missing.pop_front() {
            assert_missing(m, &miss);
            m.get_missing(&miss).expect("get_missing(front) results");
        }

        if let Some(miss) = cl.missing.pop_back() {
            assert_missing(m, &miss);
            m.get_missing(&miss).expect("get_missing(back) results");
        }
    }

    #[test]
    #[ignore]
    fn sdo_mirror() {
        let s = SourceConfig::sdo();
        let mut m = Mirror::try_from(s).unwrap();
        assert_valid_mirror(&m);

        // hardcoded sanity check
        let lp = Duration::new(28 * 24 * 60 * 60, 0);
        assert_eq!(m.loop_period, lp, "actual vs expected loop period");

        // unsafely assume *something* is in our repository
        assert_has_captures(&m);

        assert_alpha_omega(&mut m);
    }

    #[test]
    #[ignore]
    fn sdo_capturelist() {
        let m = Mirror::try_from(SourceConfig::sdo()).unwrap();
        let c = m.loop_captures();
        // this mirror uses 24 hours per capture with a 28 day loop period
        assert_eq!(28, c.len_all());

        let latest = c.latest().unwrap();
        let since = latest.time.elapsed().unwrap();
        assert!(
            since < Duration::new(60 * 60 * 24, 0),
            "{} not recent",
            display_duration(&since),
        );

        let r = TimeRange::from(StandardTimeRange::AllDayYesterday);
        let c = m.captures_in_range(&r);
        println!("all day yesterday captures: {}", c);
        assert!(!c.is_empty());
        assert_eq!(1, c.len_all());

        // SDO captures around 9:15pm per day, thus a time range from
        // midnight to 9pm (21 hours) should have no captures
        let r = TimeRange::from((r.from, r.to - Duration::from_secs(3 * 60 * 60)));
        let c = m.captures_in_range(&r);
        println!("partial day yesterday captures: {}", c);
        assert!(c.is_empty());
        assert_eq!(0, c.len_all());
    }

    #[test]
    #[ignore]
    fn abi_truecolor() {
        let s = SourceConfig::goes_abi();
        let mut m = Mirror::try_from(s).unwrap();
        assert_valid_mirror(&m);

        // FIXME: unsafe assumption
        assert_has_captures(&m);

        assert_alpha_omega(&mut m);
    }
}
