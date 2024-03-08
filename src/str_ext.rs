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
        svec,
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
        let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
        let actual: Tokens = input.tokens().collect();
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
        let actual: Units = input.units().collect();
        assert_eq!(
            actual,
            svec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"]
        );
    }

    #[test]
    fn record_iterator_with_units_records_groups_files() {
        let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
        let actual: Records = input.records().collect();
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
        let actual: Groups = input.groups().collect();
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
        let actual: Files = input.files().collect();
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
}