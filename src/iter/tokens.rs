use crate as usv;

#[derive(Debug)]
pub struct Tokens<'a> {
    pub chars: std::str::Chars<'a>,
}

impl<'a> From<std::str::Chars<'a>> for Tokens<'a> {
    fn from(chars: std::str::Chars<'a>) -> Self {
        Self { chars }
    }
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(str: &'a str) -> Self {
        Self { chars: str.chars() }
    }
}

impl<'a> std::iter::Iterator for Tokens<'a> {
    type Item = usv::Token;

    fn next(&mut self) -> Option<Self::Item> {

        // Use the content string to push each character for typical data.
        // When the processing loop encounters any end-of-content aspect,
        // such as a separator or end-of-file, then return the content.
        let mut content = String::new();

        loop {
            match self.chars.next() {
                Some(c) => {
                    match c {
                        '\u{001F}' | '␟' => { return Some(usv::Token::Unit(content)) },
                        '\u{001E}' | '␞' => { return Some(usv::Token::RecordSeparator) },
                        '\u{001D}' | '␝' => { return Some(usv::Token::GroupSeparator) },
                        '\u{001C}' | '␜' => { return Some(usv::Token::FileSeparator) },
                        '\u{0004}' | '␄' => { return Some(usv::Token::EndOfTransmission) },
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
    use crate as usv;
    use crate::examples::*;

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
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
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
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
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
        let expect = [usv::Token::Unit(usv::Unit::from(""))];
        // Control
        let input = "\u{001F}";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␟";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
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
        let expect = [usv::Token::RecordSeparator];
        // Control
        let input = "\u{001E}";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␞";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
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
        let expect = [usv::Token::GroupSeparator];
        // Control
        let input = "\u{001D}";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␝";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
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
        let expect = [usv::Token::FileSeparator];
        // Control
        let input = "␜";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␜";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
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
        let expect = [usv::Token::EndOfTransmission];
        // Control
        let input = "\u{0004}";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␄";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
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
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("a")),
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
        let input = EXAMPLE_UNITS_STYLE_SYMBOLS;
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("a")),
                usv::Token::Unit(usv::Unit::from("b")),
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
        let input = EXAMPLE_RECORDS_STYLE_SYMBOLS;
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("a")),
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::Unit(usv::Unit::from("d")),
                usv::Token::RecordSeparator,
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
        let input = EXAMPLE_GROUPS_STYLE_SYMBOLS;
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("a")),
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::Unit(usv::Unit::from("d")),
                usv::Token::RecordSeparator,
                usv::Token::GroupSeparator,
                usv::Token::Unit(usv::Unit::from("e")),
                usv::Token::Unit(usv::Unit::from("f")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("g")),
                usv::Token::Unit(usv::Unit::from("h")),
                usv::Token::RecordSeparator,
                usv::Token::GroupSeparator,
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
        let input = EXAMPLE_FILES_STYLE_SYMBOLS;
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("a")),
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::Unit(usv::Unit::from("d")),
                usv::Token::RecordSeparator,
                usv::Token::GroupSeparator,
                usv::Token::Unit(usv::Unit::from("e")),
                usv::Token::Unit(usv::Unit::from("f")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("g")),
                usv::Token::Unit(usv::Unit::from("h")),
                usv::Token::RecordSeparator,
                usv::Token::GroupSeparator,
                usv::Token::FileSeparator,
                usv::Token::Unit(usv::Unit::from("i")),
                usv::Token::Unit(usv::Unit::from("j")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("k")),
                usv::Token::Unit(usv::Unit::from("l")),
                usv::Token::RecordSeparator,
                usv::Token::GroupSeparator,
                usv::Token::Unit(usv::Unit::from("m")),
                usv::Token::Unit(usv::Unit::from("n")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("o")),
                usv::Token::Unit(usv::Unit::from("p")),
                usv::Token::RecordSeparator,
                usv::Token::GroupSeparator,
                usv::Token::FileSeparator,
            ]
        );
    }

    #[test]
    fn escape_then_typical_character() {
        let input = "ab␛xc␟";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("abc")),
            ]
        );
    }

    #[test]
    fn escape_then_special_character() {
        let input = "ab␛␟cd␟";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("ab␟cd")),
            ]
        );
    }

    #[test]
    fn escape_twice() {
        let input = "ab␛␛cd␟";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("ab␛cd")),
            ]
        );
    }

    #[test]
    fn escape_eol_per_unit() {
        let input = "a␟␛\nb␟␞␛\nc␟␛\nd␟␞␛\n";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("a")),
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::Unit(usv::Unit::from("d")),
                usv::Token::RecordSeparator,
            ]
        );
    }

    #[test]
    fn escape_eol_per_record() {
        let input = "a␟b␟␞␛\nc␟d␟␞␛\n";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("a")),
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::Unit(usv::Unit::from("d")),
                usv::Token::RecordSeparator,
            ]
        );
    }

}
