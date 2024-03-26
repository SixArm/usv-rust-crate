use crate as usv;
use crate::*;

#[derive(Debug)]
pub struct Units<'a> {
    pub iterator: usv::iter::Tokens<'a>,
}

impl<'a> From<usv::iter::Tokens<'a>> for Units<'a> {
    fn from(iterator: usv::iter::Tokens<'a>) -> Self {
        Self { iterator }
    }
}

impl<'a> From<std::str::Chars<'a>> for Units<'a> {
    fn from(chars: std::str::Chars<'a>) -> Self {
        Self::from(usv::iter::Tokens::from(chars))
    }
}

impl<'a> From<&'a str> for Units<'a> {
    fn from(str: &'a str) -> Self {
        Self::from(usv::iter::Tokens::from(str))
    }
}

impl<'a> std::iter::Iterator for Units<'a> {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iterator.next();
            match next {
                Some(token) => {
                    match token {
                        Token::Unit(s) => {
                            return Some(s)
                        },
                        _ => {},
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
        let input = EXAMPLE_UNITS_STYLE_SYMBOLS;
        let actual: usv::Units = usv::iter::Units::from(input).collect();
        assert_eq!(actual, EXAMPLE_ARRAY_UNITS);
    }

}