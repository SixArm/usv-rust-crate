use usv::*;
mod common;

#[test]
fn base_iterator_with_units_records_groups_files() {
    let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
    let iter = TokenIterator {
        chars: input.chars(),
        ..Default::default()
    };
    let actual: Vec<Token> = iter.collect();
    assert_eq!(
        actual,
        [
            Token::Unit(Unit::from("a")),
            Token::Unit(Unit::from("b")),
            Token::RecordSeparator,
            Token::Unit(Unit::from("c")),
            Token::Unit(Unit::from("d")),
            Token::RecordSeparator,
            Token::GroupSeparator,
            Token::Unit(Unit::from("e")),
            Token::Unit(Unit::from("f")),
            Token::RecordSeparator,
            Token::Unit(Unit::from("g")),
            Token::Unit(Unit::from("h")),
            Token::RecordSeparator,
            Token::GroupSeparator,
            Token::FileSeparator,
            Token::Unit(Unit::from("i")),
            Token::Unit(Unit::from("j")),
            Token::RecordSeparator,
            Token::Unit(Unit::from("k")),
            Token::Unit(Unit::from("l")),
            Token::RecordSeparator,
            Token::GroupSeparator,
            Token::Unit(Unit::from("m")),
            Token::Unit(Unit::from("n")),
            Token::RecordSeparator,
            Token::Unit(Unit::from("o")),
            Token::Unit(Unit::from("p")),
            Token::RecordSeparator,
            Token::GroupSeparator,
            Token::FileSeparator,
        ]
    );
}

#[test]
fn unit_iterator_with_units_records_groups_files() {
    let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
    let iter = UnitIterator {
        iterator: TokenIterator {
            chars: input.chars(),
        }
    };
    let actual: Units = iter.collect();
    assert_eq!(
        actual,
        svec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"]
    );
}

#[test]
fn record_iterator_with_units_records_groups_files() {
    let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
    let iter = RecordIterator {
        iterator: TokenIterator {
            chars: input.chars(),
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

#[test]
fn group_iterator_with_units_records_groups_files() {
    let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
    let iter = GroupIterator {
        iterator: TokenIterator {
            chars: input.chars(),
        }
    };
    let actual: Groups = iter.collect();
    assert_eq!(
        actual,
        [
            vec![
                svec!["a", "b"],
                svec!["c", "d"],
            ],
            vec![
                svec!["e", "f"],
                svec!["g", "h"],
            ],
            vec![
                svec!["i", "j"],
                svec!["k", "l"]
            ],
            vec![
                svec!["m", "n"],
                svec!["o", "p"],
            ],
        ]
    );
}

#[test]
fn file_iterator_with_units_records_groups_files() {
    let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
    let iter = FileIterator {
        iterator: TokenIterator {
            chars: input.chars(),
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
                    svec!["k", "l"],
                ],
                vec![
                    svec!["m", "n"],
                    svec!["o", "p"],
                ],
            ],
        ]
    );
}
