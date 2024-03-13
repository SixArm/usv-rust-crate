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
    /// `
    fn next(&mut self) -> Option<Self::Item> {
        let mut content = String::new();
        let mut oc: Option<char>;

        loop {
            oc = self.chars.next();
            match oc {
                Some('\n') | Some('\r') => {},
                _ => loop {
                    match oc {
                        None => return None,
                        Some(c) => {
                            if c == self.c {
                                return Some(content)
                            } else {
                                if c == '␗' {
                                    return None
                                } else {
                                    content.push(c);
                                    if c == '␛' {
                                        match self.chars.next() {
                                            Some(c) => { content.push(c); }
                                            None => { return Some(content); }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    oc = self.chars.next();
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = "abc-def-ghi-";
        let iter = StringIterator {
            chars: input.chars(),
            c: '-',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(
            actual, vec![
                String::from("abc"),
                String::from("def"),
                String::from("ghi"),
            ]
        );
    }

    #[test]
    fn liners() {
        let input = "\r\nabc-\r\ndef-\r\nghi-";
        let iter = StringIterator {
            chars: input.chars(),
            c: '-',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(
            actual, vec![
                String::from("abc"),
                String::from("def"),
                String::from("ghi"),
            ]
        );
    }

    #[test]
    fn end_of_transmission() {
        let input = "abc-def-ghi-␗zzz-zzz-zzz-";
        let iter = StringIterator {
            chars: input.chars(),
            c: '-',
        };
        let actual: Vec<String> = iter.collect();
        assert_eq!(
            actual, vec![
                String::from("abc"),
                String::from("def"),
                String::from("ghi"),
            ]
        );
    }

}
