use chrono::format::ParseError;

#[derive(Debug, PartialEq, Eq)]
pub enum PathMakerError {
    UnknownName(String),
    Unimplemented,
    NoNameErr,
    NoFileNameErr,
    TimeParseError(String, ParseError),
    AmbiguousTimeError(String),
    WithTimeError(String, i32, u32, u32),
    FilenameTooShort(String),
    UnparsableYear(String),
    UnparsableMonth(String),
    UnparsableDay(String),
    ImpossibleTimestamp(u64),
}
pub use PathMakerError::*;
