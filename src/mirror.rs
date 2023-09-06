use std::time::Duration;

use crate::config::SourceConfig;

pub struct Mirror {
    pub name: String,
    pub period: Duration,
}

impl Mirror {
    pub fn new(cfg: SourceConfig) -> Mirror {
        Mirror {
            name: cfg.name,
            period: cfg.period,
        }
    }
}
