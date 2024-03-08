use crate::token::Token;

#[derive(Debug)]
pub struct TokenIterator<'a> {
    pub chars: std::str::Chars<'a>,
    pub next: Option<char>,
}

impl<'a> Default for TokenIterator<'a> {
    fn default() -> TokenIterator<'a> {
        TokenIterator {
            chars: "".chars(),
            next: None,
        }
    }
}

impl<'a> std::iter::Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {

        // Use the content string to push each character for typical data.
        // When the processing loop encounters any end-of-content aspect,
        // such as a separator or end-of-file, then return the content.
        let mut content = String::new();

        loop {
            let next = self.chars.next();
            match next {
                Some(c) => {
                    match c {
                        '␟' => { return Some(Token::Unit(content)) },
                        '␞' => { return Some(Token::RecordSeparator) },
                        '␝' => { return Some(Token::GroupSeparator) },
                        '␜' => { return Some(Token::FileSeparator) },
                        '␗' => { return Some(Token::EndOfTransmissionBlock) },
                        '␖' => { return Some(Token::SynchronousIdle) },
                        '␛' => {
                            match self.chars.next() {
                                Some(c) => {
                                    match c {
                                        '␟' | '␞' | '␝' | '␜' | '␗' | '␖' | '␛' => {
                                            content.push(c);
                                        },
                                        _ => {},
                                    }
                                }
                                None => {
                                    return None
                                }
                            }
                        },
                        _ => {
                            content.push(c);
                        },
                    }
                },
                None => {
                    return None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Unit;
    use crate::token_iterator::TokenIterator;

    /// An empty string returns an empty list,
    /// because there are no separators.
    ///
    /// This is an edge case test.
    ///
    /// Input: an empty string.
    ///
    /// Expect: an empty list.
    ///
    #[test]
    fn empty() {
        let input = "";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            []
        );
    }

    /// A string of all typical characters returns an empty list,
    /// because the string has no separators i.e. the string is chaff.
    ///
    /// This is an edge case test.
    ///
    /// Input: a string of typical characters of any length.
    ///
    /// Expect: an empty list.
    ///
    #[test]
    fn chaff() {
        let input = "a";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            []
        );
    }

    /// A string of one unit separator returns one unit
    /// that contains an empty string.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a unit separator.
    ///
    /// Expect: a unit that contains an empty string.
    ///
    #[test]
    fn unit_separator() {
        let input = "␟";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::Unit(Unit::from("")),
            ]
        );
    }

    /// A string of one record separator returns a record separator.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one record separator.
    ///
    /// Expect: a record separator.
    ///
    #[test]
    fn record_separator() {
        let input = "␞";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::RecordSeparator,
            ]
        );
    }

    /// A string of one group separator returns a group separator.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one group separator.
    ///
    /// Expect: a group separator.
    ///
    #[test]
    fn group_separator() {
        let input = "␝";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::GroupSeparator,
            ]
        );
    }

    /// A string of one file separator returns a file separator.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one file separator.
    ///
    /// Expect: a file separator.
    ///
    #[test]
    fn file_separator() {
        let input = "␜";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::FileSeparator,
            ]
        );
    }

    /// A string of one end of transmission block returns an end of transmission block.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one unit separator.
    ///
    /// Expect: an end of transmission block.
    ///
    #[test]
    fn end_of_transmission_block() {
        let input = "␗";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::EndOfTransmissionBlock,
            ]
        );
    }

    /// A string of one synchronous idle returns a synchronous idle..
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one synchronous idle.
    ///
    /// Expect: a synchronous idle.
    ///
    #[test]
    fn synchronous_idle() {
        let input = "␖";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::SynchronousIdle,
            ]
        );
    }

    /// A string of typical characters of any length then a unit separator
    /// will return a unit of the typical characters.
    ///
    /// This is a typical test.
    ///
    /// Input: a string of typical characters then a unit separator.
    ///
    /// Expect: a unit.
    ///
    #[test]
    fn unit() {
        let input = "a␟";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::Unit(Unit::from("a")),
            ]
        );
    }

    /// A string of typical units will return typical units.
    ///
    /// This is an example documentation test.
    ///
    /// Input: a string of typical units.
    ///
    /// Expect: typical units .
    ///
    #[test]
    fn units() {
        let input = "a␟b␟";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::Unit(Unit::from("a")),
                Token::Unit(Unit::from("b")),
            ]
        );
    }

    /// A string of typical units, records, groups, files, etc.
    /// will return typical units, records, groups, files.
    /// 
    /// This is an example documentation test.
    ///
    /// Input: a string of empty units.
    ///
    /// Expect: empty units and unit separators.
    ///
    #[test]
    fn units_records_groups_files() {
        let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::Unit(Unit::from("a")),
                Token::Unit(Unit::from("b")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("c")),
                Token::Unit(Unit::from("d")),
                Token::RecordSeparator,
                Token::GroupSeparator,
                Token::Unit(Unit::from("e")),
                Token::Unit(Unit::from("f")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("g")),
                Token::Unit(Unit::from("h")),
                Token::RecordSeparator,
                Token::GroupSeparator,
                Token::FileSeparator,
                Token::Unit(Unit::from("i")),
                Token::Unit(Unit::from("j")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("k")),
                Token::Unit(Unit::from("l")),
                Token::RecordSeparator,
                Token::GroupSeparator,
                Token::Unit(Unit::from("m")),
                Token::Unit(Unit::from("n")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("o")),
                Token::Unit(Unit::from("p")),
                Token::RecordSeparator,
                Token::GroupSeparator,
                Token::FileSeparator,
            ]
        );
    }

    #[test]
    fn escape_then_typical_character() {
        let input = "ab␛xc␟";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::Unit(Unit::from("abc")),
            ]
        );
    }

    #[test]
    fn escape_then_special_character() {
        let input = "ab␛␟cd␟";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::Unit(Unit::from("ab␟cd")),
            ]
        );
    }

    #[test]
    fn escape_twice() {
        let input = "ab␛␛cd␟";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::Unit(Unit::from("ab␛cd")),
            ]
        );
    }

    #[test]
    fn escape_eol_per_unit() {
        let input = "a␟␛\nb␟␞␛\nc␟␛\nd␟␞␛\n";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::Unit(Unit::from("a")),
                Token::Unit(Unit::from("b")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("c")),
                Token::Unit(Unit::from("d")),
                Token::RecordSeparator,
            ]
        );
    }

    #[test]
    fn escape_eol_per_record() {
        let input = "a␟b␟␞␛\nc␟d␟␞␛\n";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::Unit(Unit::from("a")),
                Token::Unit(Unit::from("b")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("c")),
                Token::Unit(Unit::from("d")),
                Token::RecordSeparator,
            ]
        );
    }

}
