use crate::constants::*;

pub trait StyleTrait {
    fn style() -> Style;
}

pub enum StyleEnum {
    Symbols,
    Controls,
    Braces,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Style {
    pub unit_separator: String,
    pub record_separator: String,
    pub group_separator: String,
    pub file_separator: String,
    pub escape: String,
    pub end_of_transmission: String,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            unit_separator: String::from(UNIT_SEPARATOR_SYMBOL),
            record_separator: String::from(RECORD_SEPARATOR_SYMBOL),
            group_separator: String::from(GROUP_SEPARATOR_SYMBOL),
            file_separator: String::from(FILE_SEPARATOR_SYMBOL),
            escape: String::from(ESCAPE_SYMBOL),
            end_of_transmission: String::from(END_OF_TRANSMISSION_SYMBOL),
        }
    }
}

pub mod style_symbols;
pub mod style_controls;
pub mod style_braces;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_test() {
        let style = Style::default();
        assert_eq!(style.unit_separator, "␟");
        assert_eq!(style.record_separator, "␞");
        assert_eq!(style.group_separator, "␝");
        assert_eq!(style.file_separator, "␜");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "␄");
    }


}
