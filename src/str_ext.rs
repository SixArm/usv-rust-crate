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
        Unit,
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
                Token::Unit(Unit::from("a")),
                Token::UnitSeparator,
                Token::Unit(Unit::from("b")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("c")),
                Token::UnitSeparator,
                Token::Unit(Unit::from("d")),
                Token::GroupSeparator,
                Token::Unit(Unit::from("e")),
                Token::UnitSeparator,
                Token::Unit(Unit::from("f")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("g")),
                Token::UnitSeparator,
                Token::Unit(Unit::from("h")),
                Token::FileSeparator,
                Token::Unit(Unit::from("i")),
                Token::UnitSeparator,
                Token::Unit(Unit::from("j")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("k")),
                Token::UnitSeparator,
                Token::Unit(Unit::from("l")),
                Token::GroupSeparator,
                Token::Unit(Unit::from("m")),
                Token::UnitSeparator,
                Token::Unit(Unit::from("n")),
                Token::RecordSeparator,
                Token::Unit(Unit::from("o")),
                Token::UnitSeparator,
                Token::Unit(Unit::from("p")),
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

    #[test]
    fn record_iterator_with_units_records_groups_files() {
        let input: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let actual: Records = input.records().collect();
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

    #[test]
    fn group_iterator_with_units_records_groups_files() {
        let input: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
        let actual: Groups = input.groups().collect();
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