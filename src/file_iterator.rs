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
                        Token::UnitString(_) => {
                            units.push(token)
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
                            Token::UnitString("a".into()),
                            Token::UnitString("b".into()),
                        ],
                        vec![
                            Token::UnitString("c".into()),
                            Token::UnitString("d".into()),
                        ],
                    ],
                    vec![
                        vec![
                            Token::UnitString("e".into()),
                            Token::UnitString("f".into()),
                        ],
                        vec![
                            Token::UnitString("g".into()),
                            Token::UnitString("h".into()),
                        ],
                    ],
                ],
                vec![
                    vec![
                        vec![
                            Token::UnitString("i".into()),
                            Token::UnitString("j".into()),
                        ],
                        vec![
                            Token::UnitString("k".into()),
                            Token::UnitString("l".into()),
                        ]
                    ],
                    vec![
                        vec![
                            Token::UnitString("m".into()),
                            Token::UnitString("n".into()),
                        ],
                        vec![
                            Token::UnitString("o".into()),
                            Token::UnitString("p".into()),
                        ],
                    ],
                ],
            ]
        );
    }

}
