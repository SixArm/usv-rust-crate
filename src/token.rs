#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Separators
    UnitSeparator,
    RecordSeparator,
    GroupSeparator,
    FileSeparator,
    // Modifiers
    EndOfTransmissionBlock,
    //SynchronousIdle,
    Escape,
    // Content
    Unit(crate::Unit),
    Char(char),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::UnitSeparator => write!(f, "␟"),
            Token::RecordSeparator => write!(f, "␞"),
            Token::GroupSeparator => write!(f, "␝"),
            Token::FileSeparator => write!(f, "␜"),
            Token::EndOfTransmissionBlock => write!(f, "␗"),
            //Token::SynchronousIdle => write!(f, "␖"),
            Token::Escape => write!(f, "␛"),
            Token::Char(c) => write!(f, "{}", c),
            Token::Unit(s) => write!(f, "{}", s),
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
            //'␖' => Token::SynchronousIdle,
            '␛' => Token::Escape,
            c => Token::Char(c)
        }
    }
}
