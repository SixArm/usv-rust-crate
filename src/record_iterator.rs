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
                        Token::UnitSeparator => {}
                        Token::RecordSeparator |
                        Token::GroupSeparator |
                        Token::FileSeparator |
                        Token::EndOfTransmissionBlock => {
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
                    if !units.is_empty() {
                        return Some(units)
                    } else {
                        return None
                    }
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
        let input = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let iter = RecordIterator {
            iterator: TokenIterator {
                chars: input.chars(),
                ..Default::default()
            }
        };
        let actual: Records = iter.collect();
        assert_eq!(
            actual,
            [
                vec![
                    Unit::from("a"),
                    Unit::from("b"),
                ],
                vec![
                    Unit::from("c"),
                    Unit::from("d"),
                ],
                vec![
                    Unit::from("e"),
                    Unit::from("f"),
                ],
                vec![
                    Unit::from("g"),
                    Unit::from("h"),
                ],
                vec![
                    Unit::from("i"),
                    Unit::from("j"),
                ],
                vec![
                    Unit::from("k"),
                    Unit::from("l"),
                ],
                vec![
                    Unit::from("m"),
                    Unit::from("n"),
                ],
                vec![
                    Unit::from("o"),
                    Unit::from("p"),
                ],
            ]
        );
    }

}
