use crate as usv;
use crate::*;

#[derive(Debug)]
pub struct Groups<'a> {
    pub iterator: crate::iter::Tokens<'a>,
}

impl<'a> From<crate::iter::Tokens<'a>> for Groups<'a> {
    fn from(iterator: crate::iter::Tokens<'a>) -> Self {
        Self { iterator }
    }
}

impl<'a> From<std::str::Chars<'a>> for Groups<'a> {
    fn from(chars: std::str::Chars<'a>) -> Self {
        Self::from(crate::iter::Tokens::from(chars))
    }
}

impl<'a> From<&'a str> for Groups<'a> {
    fn from(str: &'a str) -> Self {
        Self::from(usv::iter::Tokens::from(str))
    }
}

impl<'a> std::iter::Iterator for Groups<'a> {
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        let mut units = Units::new();
        let mut records = Records::new();
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
                                records.push(units);
                                units = Units::new();
                            }
                        }
                        Token::GroupSeparator => {
                            if !units.is_empty() {
                                records.push(units);
                                units = Units::new();
                            }
                            if !records.is_empty() {
                                units.truncate(0);
                                return Some(records)
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
    fn test() {
        let input = EXAMPLE_STYLE_SYMBOLS_GROUPS;
        let actual: usv::Groups = usv::iter::Groups::from(input).collect();
        assert_eq!(actual, EXAMPLE_ARRAY_GROUPS);
    }

}
