use crate::*;

#[derive(Debug)]
pub struct GroupIterator<'a> {
    pub iterator: TokenIterator<'a>,
}

impl<'a> std::iter::Iterator for GroupIterator<'a> {
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
                        Token::UnitSeparator => {}
                        Token::RecordSeparator => {
                            if !units.is_empty() {
                                records.push(units);
                                units = Units::new();
                            }
                        }
                        Token::GroupSeparator |
                        Token::FileSeparator |
                        Token::EndOfTransmissionBlock => {
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
        let iter = GroupIterator {
            iterator: TokenIterator {
                chars: input.chars(),
                ..Default::default()
            }
        };
        let actual: Groups = iter.collect();
        assert_eq!(
            actual,
            [
                vec![
                    vec![
                        Unit::from("a"),
                        Unit::from("b"),
                    ],
                    vec![
                        Unit::from("c"),
                        Unit::from("d"),
                    ],
                ],
                vec![
                    vec![
                        Unit::from("e"),
                        Unit::from("f"),
                    ],
                    vec![
                        Unit::from("g"),
                        Unit::from("h"),
                    ],
                ],
                vec![
                    vec![
                        Unit::from("i"),
                        Unit::from("j"),
                    ],
                    vec![
                        Unit::from("k"),
                        Unit::from("l"),
                    ]
                ],
                vec![
                    vec![
                        Unit::from("m"),
                        Unit::from("n"),
                    ],
                    vec![
                        Unit::from("o"),
                        Unit::from("p"),
                    ],
                ],
            ],
        );
    }

}
