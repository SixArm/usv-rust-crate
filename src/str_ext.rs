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
    use crate::examples::*;

    #[test]
    fn tokens_test() {
        let actual: TokenIterator = EXAMPLE_STYLE_SYMBOLS_FILES.tokens();
    }

    #[test]
    fn units_test() {
        let actual: UnitIterator = EXAMPLE_STYLE_SYMBOLS_UNITS.units();
    }

    #[test]
    fn records_test() {
        let actual: RecordIterator = EXAMPLE_STYLE_SYMBOLS_RECORDS.records();
    }

    #[test]
    fn groups_test() {
        let actual: GroupIterator = EXAMPLE_STYLE_SYMBOLS_GROUPS.groups();
    }

    #[test]
    fn files_test() {
        let actual: FileIterator = EXAMPLE_STYLE_SYMBOLS_FILES.files();
    }

}