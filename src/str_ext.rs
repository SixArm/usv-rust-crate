use crate::string_iterator::StringIterator;

/// Iterator definitions for units, records, groups, files.
pub trait StrExt {
    fn units(&self) -> StringIterator;
    fn records(&self) -> StringIterator;
    fn groups(&self) -> StringIterator;
    fn files(&self) -> StringIterator;
}

/// Iterator implementations for units, records, groups, files.
impl StrExt for str {

    fn files(&self) -> StringIterator {
        StringIterator {
            chars: self.chars(),
            control: '\u{001C}',
            symbol: '␜',
        }
    }

    fn groups(&self) -> StringIterator {
        StringIterator {
            chars: self.chars(),
            control: '\u{001D}',
            symbol: '␝',
        }
    }

    fn records(&self) -> StringIterator {
        StringIterator {
            chars: self.chars(),
            control: '\u{001E}',
            symbol: '␞',
        }
    }

    fn units(&self) -> StringIterator {
        StringIterator {
            chars: self.chars(),
            control: '\u{001F}',
            symbol: '␟',
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svec;

    #[test]
    fn files() {
        let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝\u{001C}i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
        let actual: Vec<String> = input.files().collect();
        assert_eq!(actual, svec!["a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝", "i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝"]);
    }

    #[test]
    fn groups() {
        let input = "a␟b␟␞c␟d␟␞\u{001D}e␟f␟␞g␟h␟␞␝";
        let actual: Vec<String> = input.groups().collect();
        assert_eq!(actual, svec!["a␟b␟␞c␟d␟␞", "e␟f␟␞g␟h␟␞"]);
    }

    #[test]
    fn records() {
        let input =  "a␟b␟\u{001E}c␟d␟␞";
        let actual: Vec<String> = input.records().collect();
        assert_eq!(actual, svec!["a␟b␟", "c␟d␟"]);
    }

    #[test]
    fn units() {
        let input = "a\u{001F}b␟";
        let actual: Vec<String> = input.units().collect();
        assert_eq!(actual, svec!["a", "b"]);
    }

}
