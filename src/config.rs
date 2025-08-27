//! Mirror configuration.

use log::{info, warn};
use serde::Deserialize;
use std::default::Default;
use std::env::{self, Args};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use crate::source::{Source, SourceSearchError};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub sources: Vec<Source>,
}


impl Config {
    pub fn new(path: &str) -> Result<Config, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;
        toml::from_str(&content).map_err(|e| format!("Failed to parse TOML: {}", e))
    }
    pub fn source(&self, abbrev: &str) -> Option<&Source> {
        self.sources.iter().find(|s| s.abbrev == abbrev)
    }

    pub fn sdo(&self) -> Source {
        self.source("sdo")
            .cloned()
            .unwrap_or_else(|| Source::sdo_fallback())
    }

    pub fn sdo_0335(&self) -> Source {
        self.source("sdo_0335")
            .cloned()
            .unwrap_or_else(|| Source::sdo_0335_fallback())
    }

    pub fn goes_abi(&self) -> Source {
        self.source("goesabi")
            .cloned()
            .unwrap_or_else(|| Source::goes_abi_fallback())
    }
}


impl Default for Config {

    fn default() -> Config {
        // Try REFLECTOR_CONFIG environment variable first
        if let Ok(config_path) = env::var("REFLECTOR_CONFIG") {
            if Path::new(&config_path).exists() {
                return Config::new(&config_path).expect(&format!("Failed to parse config: {}", config_path));
            }
        }

        // Try $HOME/.config/reflector/reflector-config.toml
        if let Ok(home) = env::var("HOME") {
            let config = format!("{}/.config/reflector/reflector-config.toml", home);
            if Path::new(&config).exists() {
                return Config::new(&config).expect(&format!("Error loading config: {}", config))
            }
        }

        // Next, try reflector-config.toml in the current working directory.
        let config = "reflector-config.toml";
        if Path::new(config).exists() {
            return Config::new(&config).expect(&format!("Error loading config: {}", config))
        }
        warn!("No config file found, using hardcoded defaults");
        Config {
            sources: vec![
                Source::sdo_fallback(),
                Source::sdo_0335_fallback(),
                Source::goes_abi_fallback(),
            ]
        }
    }
}


impl FromStr for Config {
    type Err = SourceSearchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // crate non_empty_string ?
        if s.is_empty() {
            return Err(SourceSearchError::EmptyName);
        }

        let mut mm: Vec<Source> = vec![];
        for src in Config::default().sources {
            if src.name.to_ascii_lowercase().as_str() == s.to_ascii_lowercase().as_str()
                || src.abbrev.to_ascii_lowercase().as_str() == s.to_ascii_lowercase().as_str()
            {
                mm.push(src);
            }
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
                        Ok(c) => {
                            info!("matched on {}", first);
                            c
                        }
                        Err(e) => {
                            warn!("no matches for {}: {:?}", first, e);
                            default
                        }
                    }
                } else {
                    warn!("arg counting logic fail");
                    default
                }
            }
            _ => {
                warn!("unimplemented");
                default
            }
        }
    }
}

// TODO: From<Vec<String>> or some such
#[cfg(test)]
mod tests {
    use crate::config::*;
    #[test]
    fn from_str() {
        Config::from_str("Solar Data Observatory").unwrap();
        Config::from_str("sdo").unwrap();
        // Config::from_str("SDO").unwrap();
    }

    #[test]
    fn parse_source_create_local_parent_true() {
        let toml_content = r#"
[[sources]]
name = "Test Source"
abbrev = "test"
remote = "https://example.com"
local = "/tmp/test"
pathmaker = "test"
period = 3600
create_local_parent = true
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.sources.len(), 1);
        assert_eq!(config.sources[0].create_local_parent, Some(true));
    }

    #[test]
    fn parse_source_create_local_parent_false() {
        let toml_content = r#"
[[sources]]
name = "Test Source"
abbrev = "test"
remote = "https://example.com"
local = "/tmp/test"
pathmaker = "test"
period = 3600
create_local_parent = false
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.sources.len(), 1);
        assert_eq!(config.sources[0].create_local_parent, Some(false));
    }

    #[test]
    fn parse_source_create_local_parent_none() {
        let toml_content = r#"
[[sources]]
name = "Test Source"
abbrev = "test"
remote = "https://example.com"
local = "/tmp/test"
pathmaker = "test"
period = 3600
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.sources.len(), 1);
        assert_eq!(config.sources[0].create_local_parent, None);
    }
}