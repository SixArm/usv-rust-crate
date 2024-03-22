//! Create a style with all controls, such as "\u{001F}" for Unit Separator.

use crate::style::*;

pub struct StyleControls;
impl StyleTrait for StyleControls {
    fn style() -> Style {
        Style {
            unit_separator: String::from(UNIT_SEPARATOR_CONTROL),
            record_separator: String::from(RECORD_SEPARATOR_CONTROL),
            group_separator: String::from(GROUP_SEPARATOR_CONTROL),
            file_separator: String::from(FILE_SEPARATOR_CONTROL),
            escape: String::from(ESCAPE_CONTROL),
            end_of_transmission: String::from(END_OF_TRANSMISSION_CONTROL),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let style = StyleControls::style();
        assert_eq!(style.unit_separator, "\u{001F}");
        assert_eq!(style.record_separator, "\u{001E}");
        assert_eq!(style.group_separator, "\u{001D}");
        assert_eq!(style.file_separator, "\u{001C}");
        assert_eq!(style.escape, "\u{001B}");
        assert_eq!(style.end_of_transmission, "\u{0004}");
    }

}
