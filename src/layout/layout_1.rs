use crate::style::*;

pub struct Layout1;
impl MapStyle for Layout1 {
    fn map_style(&self, style: &Style) -> Style {
        Style {
            unit_separator:       format!("\n{}\n", style.unit_separator),
            record_separator:     format!("\n{}\n", style.record_separator),
            group_separator:      format!("\n{}\n", style.group_separator),
            file_separator:       format!("\n{}\n", style.file_separator),
            escape:               style.escape.clone(),
            end_of_transmission:  format!("\n{}\n", style.end_of_transmission),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as usv;

    #[test]
    fn test() {
        let style = usv::style::style_symbols();
        let style = Layout1.map_style(&style);
        assert_eq!(style.unit_separator, "\n␟\n");
        assert_eq!(style.record_separator, "\n␞\n");
        assert_eq!(style.group_separator, "\n␝\n");
        assert_eq!(style.file_separator, "\n␜\n");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "\n␄\n");
    }

}
