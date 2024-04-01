use crate as usv;
use crate::*;

#[derive(Debug)]
pub struct Files<'a> {
    pub iterator: usv::iter::Tokens<'a>,
}

impl<'a> From<usv::iter::Tokens<'a>> for Files<'a> {
    fn from(iterator: usv::iter::Tokens<'a>) -> Self {
        Self { iterator }
    }
}

impl<'a> From<std::str::Chars<'a>> for Files<'a> {
    fn from(chars: std::str::Chars<'a>) -> Self {
        Self::from(usv::iter::Tokens::from(chars))
    }
}

impl<'a> From<&'a str> for Files<'a> {
    fn from(str: &'a str) -> Self {
        Self::from(usv::iter::Tokens::from(str))
    }
}

impl<'a> std::iter::Iterator for Files<'a> {
    type Item = File;

    fn next(&mut self) -> Option<Self::Item> {
        let mut end_flag = false;
        let mut units = Units::new();
        let mut records = Records::new();
        let mut groups = Groups::new();
        loop {
            let next = self.iterator.next();
            match next {
                Some(token) => {
                    match token {
                        Token::Unit(unit) => {
                            units.push(unit)
                        },
                        Token::UnitSeparator => {},
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
                                groups.push(records);
                                units = Units::new();
                                records = Records::new();
                            }
                        }
                        _ => end_flag = true,
                    }
                },
                None => end_flag = true,
            }
            if end_flag {
                if !units.is_empty() {
                    records.push(units);
                    units = Units::new();
                }
                if !records.is_empty() {
                    groups.push(records);
                    units = Units::new();
                    records = Records::new();
                }
                if !groups.is_empty() {
                    units.truncate(0);
                    records.truncate(0);
                    return Some(groups)
                } else {
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
        let input = EXAMPLE_FILES_STYLE_SYMBOLS;
        let actual: usv::Files = usv::iter::Files::from(input).collect();
        assert_eq!(actual, EXAMPLE_ARRAY_FILES);
    }

}
