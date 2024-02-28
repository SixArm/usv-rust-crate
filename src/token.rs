#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Markers
    UnitSeparator,
    RecordSeparator,
    GroupSeparator,
    FileSeparator,
    EndOfTransmissionBlock,
    Escape,
    // Content
    UnitChar(char),
    UnitString(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::UnitSeparator => write!(f, "␟"),
            Token::RecordSeparator => write!(f, "␞"),
            Token::GroupSeparator => write!(f, "␝"),
            Token::FileSeparator => write!(f, "␜"),
            Token::EndOfTransmissionBlock => write!(f, "␗"),
            Token::Escape => write!(f, "␛"),
            Token::UnitChar(c) => write!(f, "{}", c),
            Token::UnitString(s) => write!(f, "{}", s),
        }
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '␟' => Token::UnitSeparator,
            '␞' => Token::RecordSeparator,
            '␝' => Token::GroupSeparator,
            '␜' => Token::FileSeparator,
            '␗' => Token::EndOfTransmissionBlock,
            '␛' => Token::Escape,
            c => Token::UnitChar(c)
        }
    }
}
