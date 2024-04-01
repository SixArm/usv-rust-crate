use crate as usv;

#[derive(Debug)]
pub struct Tokens<'a> {
    /// The characters that are being processed.
    pub chars: std::str::Chars<'a>,
    /// The next token that's ready to be sent.
    pub next: Option<usv::Token>,
}

impl<'a> From<std::str::Chars<'a>> for Tokens<'a> {
    fn from(chars: std::str::Chars<'a>) -> Self {
        Self { 
            chars, 
            next: None 
        }
    }
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(str: &'a str) -> Self {
        Self { 
            chars: str.chars(),
            next: None
        }
    }
}

impl<'a> std::iter::Iterator for Tokens<'a> {
    type Item = usv::Token;

    /// Get the next token.
    ///
    /// Example:
    ///
    /// ```
    /// use usv::*;
    /// let input = "a␟";
    /// let iter = usv::iter::Tokens {
    ///     chars: input.chars(),
    ///     next: None,
    /// };
    /// let tokens: Tokens = iter.collect();
    /// assert_eq!(
    ///     tokens,
    ///     [
    ///         usv::Token::Unit(String::from("a")),
    ///         usv::Token::UnitSeparator,
    ///     ]
    /// );
    /// ```
    ///
    fn next(&mut self) -> Option<Self::Item> {

        // If there's a pending token, then return it immediately.
        if self.next.is_some() {
            let next = self.next.to_owned();
            self.next = None;
            return next
        }

        // Use the content string to push each character for typical data.
        // When the processing loop encounters any end-of-content aspect,
        // such as a separator or end-of-file, then return the content.
        let mut content = String::new();

        // If there's a missing next time, then set this to true.
        let mut none_flag = false;

        loop {
            match self.chars.next() {
                Some(c) => {
                    match c {
                        '\u{001F}' | '␟' => self.next = Some(usv::Token::UnitSeparator),
                        '\u{001E}' | '␞' => self.next = Some(usv::Token::RecordSeparator),
                        '\u{001D}' | '␝' => self.next = Some(usv::Token::GroupSeparator),
                        '\u{001C}' | '␜' => self.next = Some(usv::Token::FileSeparator),
                        '\u{0004}' | '␄' => self.next = Some(usv::Token::EndOfTransmission),
                        '\u{001B}' | '␛' => {
                            match self.chars.next() {
                                Some(c) => content.push(c),
                                None => none_flag = true
                            }
                        },
                        _ => content.push(c)
                    }
                },
                None => none_flag = true
            }
            if self.next.is_some() {
                return Some(usv::Token::Unit(String::from(content.trim())))
            }
            if none_flag {
                if content.len() > 0 {
                    return Some(usv::Token::Unit(String::from(content.trim())))
                }
                return None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as usv;
    use crate::examples::*;

    /// An empty string returns an empty list.
    /// 
    /// This is because there are no separators.
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

    /// A string of one unit separator returns an empty unit then a unit separator.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a unit separator.
    ///
    /// Expect: an empty unit and unit separator.
    ///
    #[test]
    fn unit_separator() {
        let expect = [
            usv::Token::Unit(String::from("")),
            usv::Token::UnitSeparator,
        ];
        // Control
        let input = "\u{001F}";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␟";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
    }

    /// A string of one record separator returns an empty unit then a record separator.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one record separator.
    ///
    /// Expect: an empty unit and record separator.
    ///
    #[test]
    fn record_separator() {
        let expect = [
            usv::Token::Unit(String::from("")),
            usv::Token::RecordSeparator,
        ];
        // Control
        let input = "\u{001E}";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␞";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
    }

    /// A string of one group separator returns an empty unit then a group separator.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one group separator.
    ///
    /// Expect: an empty unit and group separator.
    ///
    #[test]
    fn group_separator() {
        let expect = [
            usv::Token::Unit(String::from("")),
            usv::Token::GroupSeparator,
        ];
        // Control
        let input = "\u{001D}";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␝";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
    }

    /// A string of one file separator return an empty unit then a file separator.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one file separator.
    ///
    /// Expect: an empty unit then a file separator.
    ///
    #[test]
    fn file_separator() {
        let expect = [
            usv::Token::Unit(String::from("")),
            usv::Token::FileSeparator,
        ];
        // Control
        let input = "␜";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␜";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
    }

    /// A string of one end of transmission returns an empty unit then end of transmission.
    ///
    /// This is a fundamental test.
    ///
    /// Input: a string of one unit separator.
    ///
    /// Expect: an empty unit and end of transmission.
    ///
    #[test]
    fn end_of_transmission() {
        let expect = [
            usv::Token::Unit(String::from("")),
            usv::Token::EndOfTransmission
        ];
        // Control
        let input = "\u{0004}";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "␄";
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
    }

    /// A string of a typical unit will return a typical unit.
    ///
    /// This is a typical test.
    ///
    /// Input: a string of a typical unit.
    ///
    /// Expect: a typical unit.
    ///
    #[test]
    fn unit() {
        let input = EXAMPLE_UNIT_STYLE_SYMBOLS;
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
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("b")),
            ]
        );
    }

    /// A string of a typical record will return a typical record.
    ///
    /// This is an example documentation test.
    ///
    /// Input: a string of a typical record.
    ///
    /// Expect: a typical record.
    ///
    #[test]
    fn record() {
        let input = EXAMPLE_RECORD_STYLE_SYMBOLS;
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("a")),
                usv::Token::UnitSeparator,
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
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("d")),
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
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("d")),
                usv::Token::GroupSeparator,
                usv::Token::Unit(usv::Unit::from("e")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("f")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("g")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("h")),
            ]
        );
    }

    /// A string of typical spreadsheet sheets will return groups.
    ///
    /// This is an example documentation test.
    ///
    /// Input: a string of typical spreadsheet sheets.
    ///
    /// Expect: groups, where each group starts with a sheet name.
    ///
    #[test]
    fn sheets() {
        let input = EXAMPLE_SHEETS_STYLE_SYMBOLS;
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("Sheet1")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("a")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("d")),
                usv::Token::GroupSeparator,
                usv::Token::Unit(usv::Unit::from("Sheet2")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("e")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("f")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("g")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("h")),
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
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("d")),
                usv::Token::GroupSeparator,
                usv::Token::Unit(usv::Unit::from("e")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("f")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("g")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("h")),
                usv::Token::FileSeparator,
                usv::Token::Unit(usv::Unit::from("i")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("j")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("k")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("l")),
                usv::Token::GroupSeparator,
                usv::Token::Unit(usv::Unit::from("m")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("n")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("o")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("p")),
            ]
        );
    }


    /// A string of typical spreadsheet folios will return files.
    ///
    /// This is an example documentation test.
    ///
    /// Input: a string of typical spreadsheet folios.
    ///
    /// Expect: folios, where each folio group starts with a sheet name.
    ///
    #[test]
    fn token_iterator_with_folios() {
        let input = EXAMPLE_FOLIOS_STYLE_SYMBOLS;
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(
            actual,
            [
                usv::Token::Unit(usv::Unit::from("Sheet1")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("a")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("b")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("c")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("d")),
                usv::Token::GroupSeparator,
                usv::Token::Unit(usv::Unit::from("Sheet2")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("e")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("f")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("g")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("h")),
                usv::Token::FileSeparator,
                usv::Token::Unit(usv::Unit::from("Sheet1")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("i")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("j")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("k")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("l")),
                usv::Token::GroupSeparator,
                usv::Token::Unit(usv::Unit::from("Sheet2")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("m")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("n")),
                usv::Token::RecordSeparator,
                usv::Token::Unit(usv::Unit::from("o")),
                usv::Token::UnitSeparator,
                usv::Token::Unit(usv::Unit::from("p")),
            ]
        );
    }

    #[test]
    fn escape_then_typical_character() {
        // Control
        let input = "ab\u{001B}xc";
        let expect = [usv::Token::Unit(usv::Unit::from("abxc"))];
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "ab␛xc";
        let expect = [usv::Token::Unit(usv::Unit::from("abxc"))];
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
    }

    #[test]
    fn escape_then_special_character() {
        // Control
        let input = "ab\u{001B}\u{001F}cd";
        let expect = [usv::Token::Unit(usv::Unit::from("ab\u{001F}cd"))];
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "ab␛␟cd";
        let expect = [usv::Token::Unit(usv::Unit::from("ab␟cd"))];
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
    }

    #[test]
    fn escape_then_escape() {
        // Control
        let input = "ab\u{001B}\u{001B}cd";
        let expect = [usv::Token::Unit(usv::Unit::from("ab\u{001B}cd"))];
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
        // Symbol
        let input = "ab␛␛cd";
        let expect = [usv::Token::Unit(usv::Unit::from("ab␛cd"))];
        let actual: usv::Tokens = usv::iter::Tokens::from(input).collect();
        assert_eq!(actual, expect);
    }

}
