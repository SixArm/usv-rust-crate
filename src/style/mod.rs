//! USV style is the terminology for how marks are displayed.
//!
//! The style helps make USV more useful for more people.
//!
//! The style is not part of the USV specification.
//!
//! The style is the set of these strings:
//!
//! * unit_separator
//! * record_separator
//! * group_separator
//! * file_separator
//! * escape
//! * end_of_transmission
//!
//! Currently these are the styles:
//!
//! * style-symbols: Use symbols such as "␟" for Unit Separator.
//!   This style tends to be easies for visual text editing.
//!
//! * style-controls : Use controls such as "\u001F" for Unit Separator.
//!   This style is the most-similar to ASCII Separated Values (ASV).
//!
//! * style-braces : Use braces such as "{US}" for Unit Separator.
//!   This style is to help plain text readers, and is not USV output.
//!
//! You can create your own styles that are equivalent to the above.

pub struct Style {
    pub unit_separator: String,
    pub record_separator: String,
    pub group_separator: String,
    pub file_separator: String,
    pub escape: String,
    pub end_of_transmission: String
}

impl Default for Style {
    fn default() -> Self {
        Self {
            unit_separator: String::from("␟"),
            record_separator: String::from("␞"),
            group_separator: String::from("␝"),
            file_separator: String::from("␜"),
            escape: String::from("␛"),
            end_of_transmission: String::from("␄"),
        }
    }
}

pub mod map_style; pub use map_style::*;
pub mod style_symbols; pub use style_symbols::*;
pub mod style_controls; pub use style_controls::*;
pub mod style_braces; pub use style_braces::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let style = Style::default();
        assert_eq!(style.unit_separator, "␟");
        assert_eq!(style.record_separator, "␞");
        assert_eq!(style.group_separator, "␝");
        assert_eq!(style.file_separator, "␜");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "␄");
    }

}
