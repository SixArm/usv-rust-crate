use crate::{END_OF_TRANSMISSION_SYMBOL_STR, ESCAPE_SYMBOL_STR, FILE_SEPARATOR_SYMBOL_STR, GROUP_SEPARATOR_SYMBOL_STR, RECORD_SEPARATOR_SYMBOL_STR, UNIT_SEPARATOR_SYMBOL_STR};

#[allow(dead_code)]
pub struct Style<'a> {
    file_separator: &'a str,
    group_separator: &'a str,
    record_separator: &'a str,
    unit_separator: &'a str,
    escape: &'a str,
    end_of_transmission: &'a str,
}

impl<'a> Default for Style<'a> {
    fn default() -> Self {
        Self {
            file_separator: FILE_SEPARATOR_SYMBOL_STR,
            group_separator: GROUP_SEPARATOR_SYMBOL_STR,
            record_separator: RECORD_SEPARATOR_SYMBOL_STR,
            unit_separator: UNIT_SEPARATOR_SYMBOL_STR,
            escape: ESCAPE_SYMBOL_STR,
            end_of_transmission: END_OF_TRANSMISSION_SYMBOL_STR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_test() {
        let style = Style::default();
        assert_eq!(style.file_separator, FILE_SEPARATOR_SYMBOL_STR);
        assert_eq!(style.group_separator, GROUP_SEPARATOR_SYMBOL_STR);
        assert_eq!(style.record_separator, RECORD_SEPARATOR_SYMBOL_STR);
        assert_eq!(style.unit_separator, UNIT_SEPARATOR_SYMBOL_STR);
        assert_eq!(style.escape, ESCAPE_SYMBOL_STR);
        assert_eq!(style.end_of_transmission, END_OF_TRANSMISSION_SYMBOL_STR);
    }

}
