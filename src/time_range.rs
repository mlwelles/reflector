use std::time::SystemTime;

#[derive(Debug)]
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
