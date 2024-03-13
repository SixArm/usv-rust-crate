use crate::string_iterator::StringIterator;

/// Iterator definitions for units, records, groups, files.
pub trait StringExt {
    fn units(&self) -> StringIterator;
    fn records(&self) -> StringIterator;
    fn groups(&self) -> StringIterator;
    fn files(&self) -> StringIterator;
}

/// Iterator implementations for units, records, groups, files.
impl StringExt for String {

    fn units(&self) -> StringIterator {
        StringIterator {
            chars: self.chars(),
            c: '␟',
        }
    }

    fn records(&self) -> StringIterator {
        StringIterator {
            chars: self.chars(),
            c: '␞',
        }
    }

    fn groups(&self) -> StringIterator {
        StringIterator {
            chars: self.chars(),
            c: '␝',
        }
    }

    fn files(&self) -> StringIterator {
        StringIterator {
            chars: self.chars(),
            c: '␜',
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units() {
        let input = String::from("a␟b␟");
        let actual: Vec<String> = input.units().collect();
        assert_eq!(
            actual,
            vec![
                String::from("a"),
                String::from("b"),
            ]
        );
    }

    #[test]
    fn records() {
        let input = String::from( "a␟b␟␞c␟d␟␞");
        let actual: Vec<String> = input.records().collect();
        assert_eq!(
            actual,
            vec![
                String::from("a␟b␟"),
                String::from("c␟d␟"),
            ]
        );
    }

    #[test]
    fn groups() {
        let input = String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝");
        let actual: Vec<String> = input.groups().collect();
        assert_eq!(
            actual,
            vec![
                String::from("a␟b␟␞c␟d␟␞"),
                String::from("e␟f␟␞g␟h␟␞"),
            ]
        );
    }

    #[test]
    fn files() {
        let input = String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜");
        let actual: Vec<String> = input.files().collect();
        assert_eq!(
            actual,
            vec![
                String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝"),
                String::from("i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝"),
            ]
        );
    }
}
