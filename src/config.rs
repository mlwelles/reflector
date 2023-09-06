#![deny(warnings)]
#![allow(dead_code)]

use serde::Deserialize;
use std::default::Default;

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
    pub period: u64,
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
            },
            SourceConfig {
                name: "GOES ABI_TrueColor".to_string(),
                remote: "ftp://ftp.nnvl.noaa.gov/GOES/ABI_TrueColor".to_string(),
                local: "/home/adam/tmp/sat/abi_truecolor".to_string(),
                pathmaker: "GOES".to_string(),
                flatten: None,
                period: 5 * 60 * 10,
            },
        ];
        Config { sources: srcs }
    }
}
