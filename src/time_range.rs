use std::time::SystemTime;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TimeRange {
    from: SystemTime,
    to: SystemTime,
}

#[derive(Debug)]
pub enum FactoryError {
    FromAfterTo,
}
use FactoryError::*;

impl TimeRange {
    pub fn new(from: SystemTime, to: SystemTime) -> Result<Self, FactoryError> {
        if from > to {
            Err(FromAfterTo)
        } else {
            Ok(Self { from, to })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn cmp() {
        let five_seconds = Duration::new(5, 0);
        let now = SystemTime::now();
        assert!(
            TimeRange::new(now - five_seconds * 4, now - five_seconds * 3).unwrap()
                < TimeRange::new(now - five_seconds, now).unwrap()
        );
    }
}
