use chrono::format::ParseError;

#[derive(Debug, PartialEq, Eq)]
pub enum PathMakerError {
    UnknownName(String),
    NoNameErr,
    TimeParseError(String, ParseError),
    FilenameTooShort(String),
    UnparsableYear(String),
    UnparsableMonth(String),
    UnparsableDay(String),
    ImpossibleTimestamp(u64),
}
pub use PathMakerError::*;
