use crate::style::*;

/// Create a style with all symbols, such as "␟" for Unit Separator.
pub fn style_symbols() -> Style {
    Style {
        unit_separator: String::from("␟"),
        record_separator: String::from("␞"),
        group_separator: String::from("␝"),
        file_separator: String::from("␜"),
        escape: String::from("␛"),
        end_of_transmission: String::from("␄"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let style = style_symbols();
        assert_eq!(style.unit_separator, "␟");
        assert_eq!(style.record_separator, "␞");
        assert_eq!(style.group_separator, "␝");
        assert_eq!(style.file_separator, "␜");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "␄");
    }

}
