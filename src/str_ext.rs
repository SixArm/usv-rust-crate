use crate::{
    TokenIterator,
    UnitIterator,
    RecordIterator,
    GroupIterator,
    FileIterator,
};

pub trait StrExt {
    fn tokens(&self) -> TokenIterator;
    fn units(&self) -> UnitIterator;
    fn records(&self) -> RecordIterator;
    fn groups(&self) -> GroupIterator;
    fn files(&self) -> FileIterator;
}

impl StrExt for str {

    fn tokens(&self) -> TokenIterator {
        TokenIterator {
            chars: self.chars(),
            ..Default::default()
        }
    }

    fn units(&self) -> UnitIterator {
        UnitIterator {
            iterator: self.tokens()
        }
    }

    fn records(&self) -> RecordIterator {
        RecordIterator {
            iterator: self.tokens()
        }
    }

    fn groups(&self) -> GroupIterator {
        GroupIterator {
            iterator: self.tokens()
        }
    }

    fn files(&self) -> FileIterator {
        FileIterator {
            iterator: self.tokens()
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Token, 
        Tokens,
        Units,
        Records,
        Groups,
        Files,
    };

    #[test]
    fn token_iterator_with_units_records_groups_files() {
        let input: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let actual: Tokens = input.tokens().collect();
        assert_eq!(
            actual,
            [
                Token::UnitString("a".into()),
                Token::UnitSeparator,
                Token::UnitString("b".into()),
                Token::RecordSeparator,
                Token::UnitString("c".into()),
                Token::UnitSeparator,
                Token::UnitString("d".into()),
                Token::GroupSeparator,
                Token::UnitString("e".into()),
                Token::UnitSeparator,
                Token::UnitString("f".into()),
                Token::RecordSeparator,
                Token::UnitString("g".into()),
                Token::UnitSeparator,
                Token::UnitString("h".into()),
                Token::FileSeparator,
                Token::UnitString("i".into()),
                Token::UnitSeparator,
                Token::UnitString("j".into()),
                Token::RecordSeparator,
                Token::UnitString("k".into()),
                Token::UnitSeparator,
                Token::UnitString("l".into()),
                Token::GroupSeparator,
                Token::UnitString("m".into()),
                Token::UnitSeparator,
                Token::UnitString("n".into()),
                Token::RecordSeparator,
                Token::UnitString("o".into()),
                Token::UnitSeparator,
                Token::UnitString("p".into()),
                Token::FileSeparator,
            ]
        );
    }

    #[test]
    fn unit_iterator_with_units_records_groups_files() {
        let input: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let actual: Units = input.units().collect();
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

    #[test]
    fn record_iterator_with_units_records_groups_files() {
        let input: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let actual: Records = input.records().collect();
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

    #[test]
    fn group_iterator_with_units_records_groups_files() {
        let input: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let actual: Groups = input.groups().collect();
        assert_eq!(
            actual,
            [
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
        );
    }

    #[test]
    fn file_iterator_with_units_records_groups_files() {
        let input: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let actual: Files = input.files().collect();
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