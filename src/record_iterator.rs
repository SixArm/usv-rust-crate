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
                        Token::UnitString(_) => {
                            units.push(token)
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
                    Token::UnitString("a".into()),
                    Token::UnitString("b".into()),
                ],
                vec![
                    Token::UnitString("c".into()),
                    Token::UnitString("d".into()),
                ],
                vec![
                    Token::UnitString("e".into()),
                    Token::UnitString("f".into()),
                ],
                vec![
                    Token::UnitString("g".into()),
                    Token::UnitString("h".into()),
                ],
                vec![
                    Token::UnitString("i".into()),
                    Token::UnitString("j".into()),
                ],
                vec![
                    Token::UnitString("k".into()),
                    Token::UnitString("l".into()),
                ],
                vec![
                    Token::UnitString("m".into()),
                    Token::UnitString("n".into()),
                ],
                vec![
                    Token::UnitString("o".into()),
                    Token::UnitString("p".into()),
                ],
            ]
        );
    }

}
