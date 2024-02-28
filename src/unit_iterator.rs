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
                        Token::UnitString(s) => {
                            return Some(Token::UnitString(s))
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
                Token::UnitString("a".into()),
                Token::UnitString("b".into()),
                Token::UnitString("c".into()),
                Token::UnitString("d".into()),
                Token::UnitString("e".into()),
                Token::UnitString("f".into()),
                Token::UnitString("g".into()),
                Token::UnitString("h".into()),
                Token::UnitString("i".into()),
                Token::UnitString("j".into()),
                Token::UnitString("k".into()),
                Token::UnitString("l".into()),
                Token::UnitString("m".into()),
                Token::UnitString("n".into()),
                Token::UnitString("o".into()),
                Token::UnitString("p".into()),
            ]
        );
    }

}