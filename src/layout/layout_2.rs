use crate::style::Style;
use crate::layout::LayoutTrait;

pub struct Layout2;
impl LayoutTrait for Layout2 {
    fn layout(&self, style: &Style) -> Style {
        Style { 
            unit_separator:       format!("\n\n{}\n\n", style.unit_separator),
            record_separator:     format!("\n\n{}\n\n", style.record_separator),
            group_separator:      format!("\n\n{}\n\n", style.group_separator),
            file_separator:       format!("\n\n{}\n\n", style.file_separator),
            escape:               style.escape.clone(),
            end_of_transmission:  format!("\n\n{}\n\n", style.end_of_transmission),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let style = Layout2.layout(&Style::default());
        assert_eq!(style.unit_separator, "\n\n␟\n\n");
        assert_eq!(style.record_separator, "\n\n␞\n\n");
        assert_eq!(style.group_separator, "\n\n␝\n\n");
        assert_eq!(style.file_separator, "\n\n␜\n\n");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "\n\n␄\n\n");
    }

}
