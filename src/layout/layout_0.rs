use crate::style::Style;
use crate::layout::LayoutTrait;

pub struct Layout0;
impl LayoutTrait for Layout0 {
    fn layout(&self, style: &Style) -> Style {
        Style {
            unit_separator:       style.unit_separator.clone(),
            record_separator:     style.record_separator.clone(),
            group_separator:      style.group_separator.clone(),
            file_separator:       style.file_separator.clone(),
            escape:               style.escape.clone(),
            end_of_transmission:  style.end_of_transmission.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let style = Layout0.layout(&Style::default());
        assert_eq!(style.unit_separator, "␟");
        assert_eq!(style.record_separator, "␞");
        assert_eq!(style.group_separator, "␝");
        assert_eq!(style.file_separator, "␜");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "␄");
    }

}
