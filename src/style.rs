use crate::constants::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Style {
    pub file_separator: String,
    pub group_separator: String,
    pub record_separator: String,
    pub unit_separator: String,
    pub escape: String,
    pub end_of_transmission: String,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            file_separator: String::from(FILE_SEPARATOR_SYMBOL_STR),
            group_separator: String::from(GROUP_SEPARATOR_SYMBOL_STR),
            record_separator: String::from(RECORD_SEPARATOR_SYMBOL_STR),
            unit_separator: String::from(UNIT_SEPARATOR_SYMBOL_STR),
            escape: String::from(ESCAPE_SYMBOL_STR),
            end_of_transmission: String::from(END_OF_TRANSMISSION_SYMBOL_STR),
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
