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
        let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
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
                svec!["a", "b"],
                svec!["c", "d"],
                svec!["e", "f"],
                svec!["g", "h"],
                svec!["i", "j"],
                svec!["k", "l"],
                svec!["m", "n"],
                svec!["o", "p"],
            ]
        );
    }

}
