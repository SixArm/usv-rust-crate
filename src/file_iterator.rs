use crate::*;

#[derive(Debug)]
pub struct FileIterator<'a> {
    pub iterator: TokenIterator<'a>,
}

impl<'a> std::iter::Iterator for FileIterator<'a> {
    type Item = File;

    fn next(&mut self) -> Option<Self::Item> {
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
                        Token::UnitSeparator => {}
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
                        Token::FileSeparator |
                        Token::EndOfTransmissionBlock => {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_records_groups_files() {
        let input = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let iter = FileIterator {
            iterator: TokenIterator {
                chars: input.chars(),
                ..Default::default()
            }
        };
        let actual: Files = iter.collect();
        assert_eq!(
            actual,
            [
                vec![
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
                ],
                vec![
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
            ]
        );
    }

}
