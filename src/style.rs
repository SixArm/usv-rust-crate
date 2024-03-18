use crate::constants::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Style<'a> {
    pub file_separator: &'a str,
    pub group_separator: &'a str,
    pub record_separator: &'a str,
    pub unit_separator: &'a str,
    pub escape: &'a str,
    pub end_of_transmission: &'a str,
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
