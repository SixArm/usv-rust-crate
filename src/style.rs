use crate::constants::*;

pub const STYLE_BRACES_EXAMPLE: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}i{US}j{US}{RS}k{US}l{US}{RS}{GS}m{US}n{US}{RS}o{US}p{US}{RS}{GS}{FS}\n";
pub const STYLE_CONTROLS_EXAMPLE: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}i\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}";
pub const STYLE_SYMBOLS_EXAMPLE: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
pub const STYLE_SHEETS_EXAMPLE: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\ne\n␟\nf\n␟\n\n␞\ng\n␟\nh\n␟\n\n␞\n\n␝\n\n␜\ni\n␟\nj\n␟\n\n␞\nk\n␟\nl\n␟\n\n␞\n\n␝\nm\n␟\nn\n␟\n\n␞\no\n␟\np\n␟\n\n␞\n\n␝\n\n␜\n\n";

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
            unit_separator: String::from(UNIT_SEPARATOR_SYMBOL_STR),
            record_separator: String::from(RECORD_SEPARATOR_SYMBOL_STR),
            group_separator: String::from(GROUP_SEPARATOR_SYMBOL_STR),
            file_separator: String::from(FILE_SEPARATOR_SYMBOL_STR),
            escape: String::from(ESCAPE_SYMBOL_STR),
            end_of_transmission: String::from(END_OF_TRANSMISSION_SYMBOL_STR),
        }
    }
}

impl Style {

    // Create a style with all braces, such as "{US}" for Unit Separator.
    // TODO optimize to a constant
    pub fn braces() -> Style {
        Style {
            unit_separator: String::from(UNIT_SEPARATOR_BRACE),
            record_separator: String::from(RECORD_SEPARATOR_BRACE),
            group_separator: String::from(GROUP_SEPARATOR_BRACE),
            file_separator: String::from(FILE_SEPARATOR_BRACE),
            escape: String::from(ESCAPE_BRACE),
            end_of_transmission: String::from(END_OF_TRANSMISSION_BRACE),
        }
    }

    // Create a style with all controls, such as "\u{001F}" for Unit Separator.
    // TODO optimize to a constant
    pub fn controls() -> Style {
        Style {
            unit_separator: String::from(UNIT_SEPARATOR_CONTROL_STR),
            record_separator: String::from(RECORD_SEPARATOR_CONTROL_STR),
            group_separator: String::from(GROUP_SEPARATOR_CONTROL_STR),
            file_separator: String::from(FILE_SEPARATOR_CONTROL_STR),
            escape: String::from(ESCAPE_CONTROL_STR),
            end_of_transmission: String::from(END_OF_TRANSMISSION_CONTROL_STR),
        }
    }

    // Create a style with all symbols, such as "␟" for Unit Separator.
    // TODO optimize to a constant
    pub fn symbols() -> Style {
        Style {
            unit_separator: String::from(UNIT_SEPARATOR_SYMBOL_STR),
            record_separator: String::from(RECORD_SEPARATOR_SYMBOL_STR),
            group_separator: String::from(GROUP_SEPARATOR_SYMBOL_STR),
            file_separator: String::from(FILE_SEPARATOR_SYMBOL_STR),
            escape: String::from(ESCAPE_SYMBOL_STR),
            end_of_transmission: String::from(END_OF_TRANSMISSION_SYMBOL_STR),
        }
    }

    // Create a style with liners wrapping every symbol, such as "\n␟\n" for Unit Separator.
    // TODO optimize to a constant
    pub fn liners() -> Style {
        Style {
            unit_separator: format!("\n{}\n", UNIT_SEPARATOR_SYMBOL_STR),
            record_separator: format!("\n{}\n", RECORD_SEPARATOR_SYMBOL_STR),
            group_separator: format!("\n{}\n", GROUP_SEPARATOR_SYMBOL_STR),
            file_separator: format!("\n{}\n", FILE_SEPARATOR_SYMBOL_STR),
            escape: format!("\n{}\n", ESCAPE_SYMBOL_STR),
            end_of_transmission: format!("\n{}\n", END_OF_TRANSMISSION_SYMBOL_STR),
        }
    }

    // Create a style similar to spreadsheet sheets, such as "␟" for Unit Separator and "␟\n" for Record Separator.
    // TODO optimize to a constant
    pub fn sheets() -> Style {
        Style {
            unit_separator: format!("{}", UNIT_SEPARATOR_SYMBOL_STR),
            record_separator: format!("{}\n", RECORD_SEPARATOR_SYMBOL_STR),
            group_separator: format!("{}\n", GROUP_SEPARATOR_SYMBOL_STR),
            file_separator: format!("{}\n", FILE_SEPARATOR_SYMBOL_STR),
            escape: format!("{}\n", ESCAPE_SYMBOL_STR),
            end_of_transmission: format!("{}\n", END_OF_TRANSMISSION_SYMBOL_STR),
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

    #[test]
    fn braces_test() {
        let style = Style::braces();
        assert_eq!(style.unit_separator, "{US}");
        assert_eq!(style.record_separator, "{RS}");
        assert_eq!(style.group_separator, "{GS}");
        assert_eq!(style.file_separator, "{FS}");
        assert_eq!(style.escape, "{ESC}");
        assert_eq!(style.end_of_transmission, "{EOT}");
    }

    #[test]
    fn controls_test() {
        let style = Style::controls();
        assert_eq!(style.unit_separator, "\u{001F}");
        assert_eq!(style.record_separator, "\u{001E}");
        assert_eq!(style.group_separator, "\u{001D}");
        assert_eq!(style.file_separator, "\u{001C}");
        assert_eq!(style.escape, "\u{001B}");
        assert_eq!(style.end_of_transmission, "\u{0004}");
    }

    #[test]
    fn symbols_test() {
        let style = Style::symbols();
        assert_eq!(style.unit_separator, "␟");
        assert_eq!(style.record_separator, "␞");
        assert_eq!(style.group_separator, "␝");
        assert_eq!(style.file_separator, "␜");
        assert_eq!(style.escape, "␛");
        assert_eq!(style.end_of_transmission, "␄");
    }

    #[test]
    fn liners_test() {
        let style = Style::liners();
        assert_eq!(style.unit_separator, "\n␟\n");
        assert_eq!(style.record_separator, "\n␞\n");
        assert_eq!(style.group_separator, "\n␝\n");
        assert_eq!(style.file_separator, "\n␜\n");
        assert_eq!(style.escape, "\n␛\n");
        assert_eq!(style.end_of_transmission, "\n␄\n");
    }

    #[test]
    fn sheets_test() {
        let style = Style::sheets();
        assert_eq!(style.unit_separator, "␟");
        assert_eq!(style.record_separator, "␞\n");
        assert_eq!(style.group_separator, "␝\n");
        assert_eq!(style.file_separator, "␜\n");
        assert_eq!(style.escape, "␛\n");
        assert_eq!(style.end_of_transmission, "␄\n");
    }

}
