//! Create a style with all braces, such as "{US}" for Unit Separator.

use crate::style::*;

pub struct StyleBraces;
impl StyleTrait for StyleBraces {
    fn style() -> Style {
        Style {
            unit_separator: String::from(UNIT_SEPARATOR_BRACE),
            record_separator: String::from(RECORD_SEPARATOR_BRACE),
            group_separator: String::from(GROUP_SEPARATOR_BRACE),
            file_separator: String::from(FILE_SEPARATOR_BRACE),
            escape: String::from(ESCAPE_BRACE),
            end_of_transmission: String::from(END_OF_TRANSMISSION_BRACE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let style = StyleBraces::style();
        assert_eq!(style.unit_separator, "{US}");
        assert_eq!(style.record_separator, "{RS}");
        assert_eq!(style.group_separator, "{GS}");
        assert_eq!(style.file_separator, "{FS}");
        assert_eq!(style.escape, "{ESC}");
        assert_eq!(style.end_of_transmission, "{EOT}");
    }

}
