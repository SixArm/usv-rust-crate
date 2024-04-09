use crate::style::*;

/// Create a style with all braces, such as "{US}" for Unit Separator.
pub fn style_braces() -> Style {
    Style {
        unit_separator: String::from("{US}"),
        record_separator: String::from("{RS}"),
        group_separator: String::from("{GS}"),
        file_separator: String::from("{FS}"),
        escape: String::from("{ESC}"),
        end_of_transmission: String::from("{EOT}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let style = style_braces();
        assert_eq!(style.unit_separator, "{US}");
        assert_eq!(style.record_separator, "{RS}");
        assert_eq!(style.group_separator, "{GS}");
        assert_eq!(style.file_separator, "{FS}");
        assert_eq!(style.escape, "{ESC}");
        assert_eq!(style.end_of_transmission, "{EOT}");
    }

}
