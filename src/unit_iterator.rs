use crate::*;

#[derive(Debug, Default)]
pub struct UnitIterator<'a> {
    pub iterator: TokenIterator<'a>,
}

impl<'a> std::iter::Iterator for UnitIterator<'a> {
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

    #[test]
    fn units_records_groups_files() {
        let input = EXAMPLE_STYLE_SYMBOLS_UNITS;
        let iter = UnitIterator {
            iterator: TokenIterator {
                chars: input.chars(),
                ..Default::default()
            }
        };
        let actual: Units = iter.collect();
        assert_eq!(actual, EXAMPLE_ARRAY_UNITS);
    }

}