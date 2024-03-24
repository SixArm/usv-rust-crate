use crate::*;

#[derive(Debug)]
pub struct RecordIterator<'a> {
    pub iterator: TokenIterator<'a>,
}

impl<'a> std::iter::Iterator for RecordIterator<'a> {
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

    #[test]
    fn units_records_groups_files() {
        let input = EXAMPLE_STYLE_SYMBOLS_RECORDS;
        let iter = RecordIterator {
            iterator: TokenIterator {
                chars: input.chars(),
                ..Default::default()
            }
        };
        let actual: Records = iter.collect();
        assert_eq!(actual, EXAMPLE_ARRAY_RECORDS);
    }

}
