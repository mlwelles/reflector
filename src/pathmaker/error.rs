use chrono::format::ParseError;

#[derive(Debug, PartialEq, Eq)]
pub enum PathMakerError {
    UnknownName(String),
    Unimplemented,
    NoNameErr,
    NoFileNameErr,
    TimeParseError(String, ParseError),
    AmbiguousTimeError(i32, u32, u32, u32, u32, u32),
    NoTimeError(i32, u32, u32, u32, u32, u32),
    FilenameTooShort(String),
    FilenameTooLong(String),
    UnparsableYear(String),
    UnparsableMonth(String),
    UnparsableDay(String),
    UnparsableHour(String),
    UnparsableMinute(String),
    ImpossibleTimestamp(u64),
    MissingSeparator(char, String),
    MysteryError(String),
}
pub use PathMakerError::*;
