use crate::style::Style;
use crate::layout::LayoutTrait;

pub struct Layout0;
impl LayoutTrait for Layout0 {
    fn layout(&self, style: &Style) -> Style {
        style.clone()
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
