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
//!
//! This USV Rust crate provides style capabilities:
//!
//! * StyleTrait provides the style() function that creates a style.

use crate::constants::*;

pub trait StyleTrait {
    fn style(&self) -> Style;
}

pub mod style_symbols;
pub mod style_controls;
pub mod style_braces;

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
