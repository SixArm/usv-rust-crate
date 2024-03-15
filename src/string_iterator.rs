/// StringIterator reads characters until a target character is found.
#[derive(Debug)]
pub struct StringIterator<'a> {
    pub chars: std::str::Chars<'a>,
    pub c: char,
}

/// StringIterator reads characters until a target character is found.
impl<'a> std::iter::Iterator for StringIterator<'a> {
    type Item = String;

    /// Get the next string, until a target character is found, 
    /// or end of transmission block is found.
    /// 
    /// Example:
    /// 
    /// ```
    /// use usv::*;
    /// let input = "abc-def-ghi-";
    /// let iter = StringIterator {
    ///     chars: input.chars(),
    ///     c: '-',
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
                    if c == self.c {
                        return Some(content)
                    } else {
                        match c {
                            
                            // End of Transmission Block => return immediately.
                            '␗' => return None,

                            // Escape => make the next character a content character.
                            '␛' => {
                                content.push(c);
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
                                            if c == self.c {
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
    fn simple() {
        let input = "abc-def-ghi-";
        let iter = StringIterator {
            chars: input.chars(),
            c: '-',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["abc", "def", "ghi"]);
    }

    #[test]
    fn leading_liners() {
        let input = "\r\nabc-\r\ndef-\r\nghi-";
        let iter = StringIterator {
            chars: input.chars(),
            c: '-',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["abc", "def", "ghi"]);
    }

    #[test]
    fn trailing_liners() {
        let input = "abc\r\n-def\r\n-ghi-\r\n";
        let iter = StringIterator {
            chars: input.chars(),
            c: '-',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["abc", "def", "ghi"]);
    }

    // #[test]
    // fn surrounding_liners() {
    //     let input = "\r\nabc\r\n-\r\ndef\r\n-\r\nghi\r\n-\r\n";
    //     let iter = StringIterator {
    //         chars: input.chars(),
    //         c: '-',
    //     };
    //     let actual: Vec<String> = iter.collect();
    //     assert_eq!(
    //         actual, svec![
    //             "abc",
    //             "def",
    //             "ghi",
    //         ]
    //     );
    // }

    #[test]
    fn end_of_transmission() {
        let input = "abc-def-ghi-␗zzz-zzz-zzz-";
        let iter = StringIterator {
            chars: input.chars(),
            c: '-',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(actual, svec!["abc", "def", "ghi"]);
    }

}
