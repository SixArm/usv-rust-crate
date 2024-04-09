use crate::style::*;

pub struct LayoutUnits;
impl MapStyle for LayoutUnits {
    fn map_style(&self, style: &Style) -> Style {
        Style {
            unit_separator:       format!("{}\n", style.unit_separator),
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
    use crate as usv;

    #[test]
    fn test() {
        let style = LayoutUnits.map_style(&usv::style::Style::default());
        assert_eq!(style.unit_separator, "␟\n");
        assert_eq!(style.record_separator, "␞\n");
        assert_eq!(style.group_separator, "␝\n");
        assert_eq!(style.file_separator, "␜\n");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "␄\n");
    }

}
