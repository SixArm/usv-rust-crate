/// StringIterator is the base iterator for all of the domain-specific
/// iterators i.e. str_ext.rs .files(), .groups(), .records(), .units().
#[derive(Debug)]
pub struct StringIterator<'a> {

    /// Character iterator that provides next().
    pub chars: std::str::Chars<'a>,

    /// Control character such as U+001F Unit Separator
    pub control: char,

    /// Symbol character such as  U+241F Symbol for Unit Separator
    pub symbol: char,

}

/// StringIterator reads characters until a target character is found.
impl<'a> std::iter::Iterator for StringIterator<'a> {
    type Item = String;

    /// Get the next string, until a target character is found,
    /// or End of Transmission is found.
    ///
    /// Example:
    ///
    /// ```
    /// use usv::*;
    /// let input = "abc␟def␟ghi␟";
    /// let iter = StringIterator {
    ///     chars: input.chars(),
    ///     control: '\u{001F}',
    ///     symbol: '␟',
    /// };
    /// let actual: Vec<String> = iter.collect();
    /// assert_eq!(
    ///     actual,
    ///     vec![
    ///         String::from("abc"),
    ///         String::from("def"),
    ///         String::from("ghi"),
    ///     ]
    /// );
    /// ```
    ///
    fn next(&mut self) -> Option<Self::Item> {

        // The current option character that's being processed.
        let mut oc: Option<char>;

        // String builder for content characters.
        let mut content = String::new();

        // Skip liner characters before any content.
        loop {
            oc = self.chars.next();
            match oc {
                Some(c) => {
                    match c {
                        '\n' | '\r' => {},
                        _ => break,
                    }
                },
                _ => break,
            };
        }

        loop {
            match oc {
                None => return None,
                Some(c) => {
                    if c == self.control || c == self.symbol {
                        return Some(content)
                    } else {
                        match c {

                            // End of Transmission => return immediately.
                            '\u{0004}' | '␄' => return None,

                            // Escape => make the next character a content character.
                            '\u{001B}' | '␛' => {
                                match self.chars.next() {
                                    Some(c) => {
                                        content.push(c)
                                    },
                                    None => {
                                        return None
                                    },
                                }
                                oc = self.chars.next();
                            },

                            // Newline or Return => aggregate potential liners to accept or reject.
                            '\n' | '\r' => {
                                let mut liner = String::new();
                                liner.push(c);
                                loop {
                                    oc = self.chars.next();
                                    match oc {
                                        Some(c) => {
                                            if c == self.control || c == self.symbol {
                                                break;
                                            }
                                            match c {
                                                '\n' | '\r' => {
                                                    liner.push(c);
                                                },
                                                _ => {
                                                    content.push_str(&liner);
                                                }
                                            }
                                        },
                                        None => break,
                                    };
                                }
                            }

                            // Typical character => append to the content string builder.
                            _ => {
                                content.push(c);
                                oc = self.chars.next();
                            },

                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svec;

    #[test]
    fn separators() {
        // Input is any mix of content, control separators, symbol separators, no liners.
        let input = "abc-def+ghi-jkl+";
        let iter = StringIterator {
            chars: input.chars(),
            control: '-',
            symbol: '+',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["abc", "def", "ghi", "jkl"]);
    }

    #[test]
    fn liners() {
        // Input is any mix of leading liners and trailing liners.
        let input = "\r\nabc\r\n-\r\ndef\r\n+\r\nghi\r\n-\r\njkl\r\n+\r\n";
        let iter = StringIterator {
            chars: input.chars(),
            control: '-',
            symbol: '+',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["abc", "def", "ghi", "jkl"]);
    }

    #[test]
    fn escape_with_control() {
        // Input is any mix of typical data with an embedded escaped separator.
        let input = "a\u{001B}-b\u{001B}-c-";
        let iter = StringIterator {
            chars: input.chars(),
            control: '-',
            symbol: '+',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["a-b-c"]);
    }

    #[test]
    fn escape_with_symbol() {
        // Input is any mix of typical data with an embedded escaped separator.
        let input = "a␛-b␛-c-";
        let iter = StringIterator {
            chars: input.chars(),
            control: '-',
            symbol: '+',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["a-b-c"]);
    }

    #[test]
    fn end_of_transmission_with_control() {
        // Input is any mix of typical data, then End of Transmission, then chaff.
        let input = "abc-def-␄zzz-zzz-zzz-";
        let iter = StringIterator {
            chars: input.chars(),
            control: '-',
            symbol: '+',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["abc", "def"]);
    }

    #[test]
    fn end_of_transmission_with_symbol() {
        // Input is any mix of typical data, then End of Transmission, then chaff.
        let input = "abc-def-\u{0004}zzz-zzz-zzz-";
        let iter = StringIterator {
            chars: input.chars(),
            control: '-',
            symbol: '+',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["abc", "def"]);
    }

}
