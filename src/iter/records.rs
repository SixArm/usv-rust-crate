use crate as usv;
use crate::*;

#[derive(Debug)]
pub struct Records<'a> {
    pub iterator: usv::iter::Tokens<'a>,
}

impl<'a> From<usv::iter::Tokens<'a>> for Records<'a> {
    fn from(iterator: usv::iter::Tokens<'a>) -> Self {
        Self { iterator }
    }
}

impl<'a> From<std::str::Chars<'a>> for Records<'a> {
    fn from(chars: std::str::Chars<'a>) -> Self {
        Self::from(usv::iter::Tokens::from(chars))
    }
}

impl<'a> From<&'a str> for Records<'a> {
    fn from(str: &'a str) -> Self {
        Self::from(usv::iter::Tokens::from(str))
    }
}

impl<'a> std::iter::Iterator for Records<'a> {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let mut units = Units::new();
        loop {
            let next = self.iterator.next();
            match next {
                Some(token) => {
                    match token {
                        Token::Unit(unit) => {
                            units.push(unit)
                        },
                        Token::RecordSeparator => {
                            if !units.is_empty() {
                                return Some(units)
                            } else {
                                return None
                            }
                        },
                        _ => {}
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

    #[test]
    fn units_records_groups_files() {
        let input = EXAMPLE_RECORDS_STYLE_SYMBOLS;
        let actual: usv::Records = usv::iter::Records::from(input).collect();
        assert_eq!(actual, EXAMPLE_ARRAY_RECORDS);
    }

}
