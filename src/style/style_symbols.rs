//! Create a style with all symbols, such as "␟" for Unit Separator.

use crate::style::*;

pub struct StyleSymbols;
impl StyleTrait for StyleSymbols {
    fn style() -> Style {
        Style {
            unit_separator: String::from(UNIT_SEPARATOR_SYMBOL),
            record_separator: String::from(RECORD_SEPARATOR_SYMBOL),
            group_separator: String::from(GROUP_SEPARATOR_SYMBOL),
            file_separator: String::from(FILE_SEPARATOR_SYMBOL),
            escape: String::from(ESCAPE_SYMBOL),
            end_of_transmission: String::from(END_OF_TRANSMISSION_SYMBOL),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let style = StyleSymbols::style();
        assert_eq!(style.unit_separator, "␟");
        assert_eq!(style.record_separator, "␞");
        assert_eq!(style.group_separator, "␝");
        assert_eq!(style.file_separator, "␜");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "␄");
    }

}
