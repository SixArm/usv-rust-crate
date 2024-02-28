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

        if let Some(c) = self.next {
            self.next = None;
            return match c {
                '␟' => Some(Token::UnitSeparator),
                '␞' => Some(Token::RecordSeparator),
                '␝' => Some(Token::GroupSeparator),
                '␜' => Some(Token::FileSeparator),
                _ => panic!("{}", c)
            }
        }

        // Use the content string to push each character for typical data.
        // When the processing loop encounters any end-of-content aspect,
        // such as a separator or end-of-file, then return the content.
        let mut content = String::new();

        loop {
            let next = self.chars.next();
            match next {
                Some(c) => {
                    match c {
                        '␟' | '␞' | '␝' | '␜' => {
                            self.next = next;
                            return Some(Token::UnitString(content))
                        },
                        '␗' => {
                            return Some(Token::EndOfTransmissionBlock)
                        },
                        '␛' => {
                            match self.chars.next() {
                                Some(c) => {
                                    match c {
                                        '␟' | '␞' | '␝' | '␜' | '␗' | '␛' => {
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
    /// that contains an empty string, then a unit separator.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a unit separator.
    ///
    /// Expect: a unit that contains an empty string, then a unit separator.
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
                Token::UnitString("".into()),
                Token::UnitSeparator,
            ]
        );
    }

    /// A string of one record separator returns one unit
    /// that contains an empty string, then a record separator.
    ///
    /// This is because the separator implicitly ends the unit.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one record separator.
    ///
    /// Expect: a unit that contains an empty string, then a record separator.
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
                Token::UnitString("".into()),
                Token::RecordSeparator,
            ]
        );
    }

    /// A string of one group separator returns one unit
    /// that contains an empty string, then a group separator.
    ///
    /// This is because the separator implicitly ends the unit.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one group separator.
    ///
    /// Expect: a unit that contains an empty string, then a group separator.
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
                Token::UnitString("".into()),
                Token::GroupSeparator,
            ]
        );
    }

    /// A string of one file separator returns one unit
    /// that contains an empty string, then a file separator.
    ///
    /// This is because the separator implicitly ends the unit.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one file separator.
    ///
    /// Expect: a unit that contains an empty string, then a file separator.
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
                Token::UnitString("".into()),
                Token::FileSeparator,
            ]
        );
    }

    /// A string of one end of transmission returns an empty list.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one unit separator.
    ///
    /// Expect: a unit that contains an empty string, then a file separator.
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

    /// A string of typical characters of any length then a unit separator
    /// will return a unit of the typical characters and a unit separator.
    ///
    /// This is a typical test.
    ///
    /// Input: a string of typical characters then a unit separator.
    ///
    /// Expect: a unit and unit separator.
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
                Token::UnitString("a".into()),
                Token::UnitSeparator,
            ]
        );
    }

    /// A string of typical units will return typical results.
    ///
    /// This is an example documentation test.
    ///
    /// Input: a string of typical units.
    ///
    /// Expect: typical units and unit separators.
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
                Token::UnitString("a".into()),
                Token::UnitSeparator,
                Token::UnitString("b".into()),
                Token::UnitSeparator,
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
        let input = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::UnitString("a".into()),
                Token::UnitSeparator,
                Token::UnitString("b".into()),
                Token::RecordSeparator,
                Token::UnitString("c".into()),
                Token::UnitSeparator,
                Token::UnitString("d".into()),
                Token::GroupSeparator,
                Token::UnitString("e".into()),
                Token::UnitSeparator,
                Token::UnitString("f".into()),
                Token::RecordSeparator,
                Token::UnitString("g".into()),
                Token::UnitSeparator,
                Token::UnitString("h".into()),
                Token::FileSeparator,
                Token::UnitString("i".into()),
                Token::UnitSeparator,
                Token::UnitString("j".into()),
                Token::RecordSeparator,
                Token::UnitString("k".into()),
                Token::UnitSeparator,
                Token::UnitString("l".into()),
                Token::GroupSeparator,
                Token::UnitString("m".into()),
                Token::UnitSeparator,
                Token::UnitString("n".into()),
                Token::RecordSeparator,
                Token::UnitString("o".into()),
                Token::UnitSeparator,
                Token::UnitString("p".into()),
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
                Token::UnitString("abc".into()),
                Token::UnitSeparator,
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
                Token::UnitString("ab␟cd".into()),
                Token::UnitSeparator,
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
                Token::UnitString("ab␛cd".into()),
                Token::UnitSeparator,
            ]
        );
    }

    #[test]
    fn escape_eol_per_unit() {
        let input = "a␟␛\nb␞␛\nc␟␛\nd␞␛\n";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::UnitString("a".into()),
                Token::UnitSeparator,
                Token::UnitString("b".into()),
                Token::RecordSeparator,
                Token::UnitString("c".into()),
                Token::UnitSeparator,
                Token::UnitString("d".into()),
                Token::RecordSeparator,
            ]
        );
    }

    #[test]
    fn escape_eol_per_record() {
        let input = "a␟b␞␛\nc␟d␞␛\n";
        let iter = TokenIterator {
            chars: input.chars(),
            ..Default::default()
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            [
                Token::UnitString("a".into()),
                Token::UnitSeparator,
                Token::UnitString("b".into()),
                Token::RecordSeparator,
                Token::UnitString("c".into()),
                Token::UnitSeparator,
                Token::UnitString("d".into()),
                Token::RecordSeparator,
            ]
        );
    }

}
