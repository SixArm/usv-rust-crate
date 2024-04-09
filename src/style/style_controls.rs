use crate::style::*;

/// Create a style with all controls, such as "\u{001F}" for Unit Separator.
pub fn style_controls() -> Style {
    Style {
        unit_separator: String::from("\u{001F}"),
        record_separator: String::from("\u{001E}"),
        group_separator: String::from("\u{001D}"),
        file_separator: String::from("\u{001C}"),
        escape: String::from("\u{001B}"),
        end_of_transmission: String::from("\u{0004}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let style = style_controls();
        assert_eq!(style.unit_separator, "\u{001F}");
        assert_eq!(style.record_separator, "\u{001E}");
        assert_eq!(style.group_separator, "\u{001D}");
        assert_eq!(style.file_separator, "\u{001C}");
        assert_eq!(style.escape, "\u{001B}");
        assert_eq!(style.end_of_transmission, "\u{0004}");
    }

}
