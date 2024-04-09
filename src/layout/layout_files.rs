use crate::style::*;

pub struct LayoutFiles;
impl MapStyle for LayoutFiles {
    fn map_style(&self, style: &Style) -> Style {
        Style {
            unit_separator:       style.unit_separator.clone(),
            record_separator:     style.record_separator.clone(),
            group_separator:      style.group_separator.clone(),
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
        let style = usv::style::style_symbols();
        let style = LayoutFiles.map_style(&style);
        assert_eq!(style.unit_separator, "␟");
        assert_eq!(style.record_separator, "␞");
        assert_eq!(style.group_separator, "␝");
        assert_eq!(style.file_separator, "␜\n");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "␄\n");
    }

}
