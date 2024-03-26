//! Token is an enumeration of the USV data structures.
//!
//! Note that Escape (ESC) is not a token, because it is within unit content.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unit(String),
    UnitSeparator,
    RecordSeparator,
    GroupSeparator,
    FileSeparator,
    EndOfTransmission,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Unit(s) => write!(f, "{}", s),
            Token::UnitSeparator => write!(f, "␟"),
            Token::RecordSeparator => write!(f, "␞"),
            Token::GroupSeparator => write!(f, "␝"),
            Token::FileSeparator => write!(f, "␜"),
            Token::EndOfTransmission => write!(f, "␄"),
        }
    }
}
