//! Mirror configuration.

use serde::Deserialize;
use std::default::Default;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sources: Vec<SourceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub remote: String,
    pub local: String,
    pub pathmaker: String,
    /// period between captures
    pub period: u64,
    /// seconds after midnight to offset all times
    pub seed_past_midnight: Option<u64>,
    pub flatten: Option<bool>,
}

impl Default for Config {
    fn default() -> Config {
        let srcs = vec![
            SourceConfig {
                name: "Solar Data Observatory".to_string(),
                remote: "https://sdo.gsfc.nasa.gov/assets/img/dailymov".to_string(),
                local: "/home/adam/tmp/sat/sdo".to_string(),
                pathmaker: "SDO".to_string(),
                flatten: Some(true),
                period: 5 * 60 * 60 * 24,
                seed_past_midnight: None,
            },
            SourceConfig {
                name: "GOES ABI_TrueColor".to_string(),
                remote: "ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor".to_string(),
                local: "/home/adam/tmp/sat/abi_truecolor".to_string(),
                pathmaker: "GOES-R".to_string(),
                flatten: None,
                period: 5 * 60 * 10,
                seed_past_midnight: None,
            },
        ];
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
        if s.len() < 1 {
            return Err(SourceSearchError::EmptyName);
        }

        let mut mm: Vec<SourceConfig> = vec![];
        for src in Config::default().sources {
            if src.name == s {
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
