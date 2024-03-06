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
        let input = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let iter = UnitIterator {
            iterator: TokenIterator {
                chars: input.chars(),
                ..Default::default()
            }
        };
        let actual: Units = iter.collect();
        assert_eq!(
            actual,
            [
                Unit::from("a"),
                Unit::from("b"),
                Unit::from("c"),
                Unit::from("d"),
                Unit::from("e"),
                Unit::from("f"),
                Unit::from("g"),
                Unit::from("h"),
                Unit::from("i"),
                Unit::from("j"),
                Unit::from("k"),
                Unit::from("l"),
                Unit::from("m"),
                Unit::from("n"),
                Unit::from("o"),
                Unit::from("p"),
            ]
        );
    }

}