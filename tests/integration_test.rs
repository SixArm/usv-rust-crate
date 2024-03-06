use usv::*;

#[test]
fn base_iterator_with_units_records_groups_files() {
    let input = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p␜";
    let iter = TokenIterator {
        chars: input.chars(),
        ..Default::default()
    };
    let actual: Vec<Token> = iter.collect();
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

#[test]
fn record_iterator_with_units_records_groups_files() {
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

#[test]
fn file_iterator_with_units_records_groups_files() {
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
