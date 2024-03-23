use crate::style::Style;
use crate::layout::LayoutTrait;

pub struct LayoutRecords;
impl LayoutTrait for LayoutRecords {
    fn layout(&self, style: &Style) -> Style {
        Style { 
            unit_separator:       style.unit_separator.clone(),
            record_separator:     format!("{}\n", style.record_separator),
            group_separator:      format!("{}\n", style.group_separator),
            file_separator:       format!("{}\n", style.file_separator),
            escape:               style.escape.clone(),
            end_of_transmission:  format!("{}\n", style.end_of_transmission),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let style = LayoutRecords.layout(&Style::default());
        assert_eq!(style.unit_separator, "␟");
        assert_eq!(style.record_separator, "␞\n");
        assert_eq!(style.group_separator, "␝\n");
        assert_eq!(style.file_separator, "␜\n");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "␄\n");
    }

}
