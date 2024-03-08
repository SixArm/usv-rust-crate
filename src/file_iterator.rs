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
                        Token::FileSeparator => {
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
        let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
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
                        svec!["a", "b"],
                        svec!["c", "d"],
                    ],
                    vec![
                        svec!["e", "f"],
                        svec!["g", "h"],
                    ],
                ],
                vec![
                    vec![
                        svec!["i", "j"],
                        svec!["k", "l"]
                    ],
                    vec![
                        svec!["m", "n"],
                        svec!["o", "p"],
                    ],
                ],
            ]
        );
    }

}
