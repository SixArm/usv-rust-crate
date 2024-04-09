use crate::style::*;

/// Map a style into a style.
pub trait MapStyle {
    fn map_style(&self, style: &Style) -> Style;
}
