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
