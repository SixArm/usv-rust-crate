use crate::token::Token;

#[derive(Debug)]
pub struct TokenIterator<'a> {
    pub chars: std::str::Chars<'a>,
}

impl<'a> Default for TokenIterator<'a> {
    fn default() -> TokenIterator<'a> {
        TokenIterator {
            chars: "".chars(),
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
            match self.chars.next() {
                Some(c) => {
                    match c {
                        '\u{001F}' | '␟' => { return Some(Token::Unit(content)) },
                        '\u{001E}' | '␞' => { return Some(Token::RecordSeparator) },
                        '\u{001D}' | '␝' => { return Some(Token::GroupSeparator) },
                        '\u{001C}' | '␜' => { return Some(Token::FileSeparator) },
                        '\u{0004}' | '␄' => { return Some(Token::EndOfTransmission) },
                        '\u{001B}' | '␛' => {
                            match self.chars.next() {
                                Some(c) => {
                                    match c {
                                        '\u{001F}' | '␟' |
                                        '\u{001E}' | '␞' |
                                        '\u{001D}' | '␝' |
                                        '\u{001C}' | '␜' |
                                        '\u{0004}' | '␄' |
                                        '\u{001B}' | '␛' => {
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
    use crate::examples::*;
    use crate::{Token, Tokens, Unit};
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
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(
            actual,
            []
        );
    }

    /// A string of one unit separator returns an empty unit.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a unit separator.
    ///
    /// Expect: a unit that contains an empty string.
    ///
    #[test]
    fn unit_separator() {
        let expect = [Token::Unit(Unit::from(""))];
        // Control
        let input = "\u{001F}";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␟";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
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
    fn record_separator_control() {
        let expect = [Token::RecordSeparator];
        // Control
        let input = "\u{001E}";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␞";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
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
    fn group_separator_symbol() {
        let expect = [Token::GroupSeparator];
        // Control
        let input = "\u{001D}";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␝";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
    }

    /// A string of one file separator a file separator.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one file separator.
    ///
    /// Expect: a file separator.
    ///
    #[test]
    fn file_separator() {
        let expect = [Token::FileSeparator];
        // Control
        let input = "␜";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␜";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
    }

    /// A string of one end of transmission returns an end of transmission.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one unit separator.
    ///
    /// Expect: an end of transmission.
    ///
    #[test]
    fn end_of_transmission() {
        let expect = [Token::EndOfTransmission];
        // Control
        let input = "\u{0004}";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␄";
        let iter = TokenIterator {
            chars: input.chars(),
        };
        let actual: Vec<Token> = iter.collect();
        assert_eq!(actual, expect);
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
    /// Expect: typical units.
    ///
    #[test]
    fn units() {
        let input = EXAMPLE_STYLE_SYMBOLS_UNITS;
        let iter = TokenIterator {
            chars: input.chars(),
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

    /// A string of typical records will return typical records.
    ///
    /// This is an example documentation test.
    ///
    /// Input: a string of typical records.
    ///
    /// Expect: typical records.
    ///
    #[test]
    fn records() {
        let input = EXAMPLE_STYLE_SYMBOLS_RECORDS;
        let iter = TokenIterator {
            chars: input.chars(),
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

    /// A string of typical groups will return typical groups.
    ///
    /// This is an example documentation test.
    ///
    /// Input: a string of typical groups.
    ///
    /// Expect: typical groups.
    ///
    #[test]
    fn groups() {
        let input = EXAMPLE_STYLE_SYMBOLS_GROUPS;
        let iter = TokenIterator {
            chars: input.chars(),
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
            ]
        );
    }

    /// A string of typical files will return typical files.
    ///
    /// This is an example documentation test.
    ///
    /// Input: a string of typical files.
    ///
    /// Expect: typical files.
    ///
    #[test]
    fn token_iterator_with_files() {
        let input = EXAMPLE_STYLE_SYMBOLS_FILES;
        let iter = TokenIterator {
            chars: input.chars(),
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
