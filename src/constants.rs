pub const UNIT_SEPARATOR_NAME: &str = "Unit Separator (US)";
pub const UNIT_SEPARATOR_BRACE: &str = "{US}";
pub const UNIT_SEPARATOR_CONTROL_CHAR: char = '\u{001F}';
pub const UNIT_SEPARATOR_CONTROL_STR: &str = "\u{001F}";
pub const UNIT_SEPARATOR_SYMBOL_CHAR: char = '␟';
pub const UNIT_SEPARATOR_SYMBOL_STR: &str = "␟";

pub const RECORD_SEPARATOR_NAME: &str = "Record Separator (RS)";
pub const RECORD_SEPARATOR_BRACE: &str = "{RS}";
pub const RECORD_SEPARATOR_CONTROL_CHAR: char = '\u{001E}';
pub const RECORD_SEPARATOR_CONTROL_STR: &str = "\u{001E}";
pub const RECORD_SEPARATOR_SYMBOL_CHAR: char = '␞';
pub const RECORD_SEPARATOR_SYMBOL_STR: &str = "␞";

pub const GROUP_SEPARATOR_NAME: &str = "Group Separator (GS)";
pub const GROUP_SEPARATOR_BRACE: &str = "{GS}";
pub const GROUP_SEPARATOR_CONTROL_CHAR: char = '\u{001D}';
pub const GROUP_SEPARATOR_CONTROL_STR: &str = "\u{001D}";
pub const GROUP_SEPARATOR_SYMBOL_CHAR: char = '␝';
pub const GROUP_SEPARATOR_SYMBOL_STR: &str = "␝";

pub const FILE_SEPARATOR_NAME: &str = "File Separator (FS)";
pub const FILE_SEPARATOR_BRACE: &str = "{FS}";
pub const FILE_SEPARATOR_CONTROL_CHAR: char = '\u{001C}';
pub const FILE_SEPARATOR_CONTROL_STR: &str = "\u{001C}";
pub const FILE_SEPARATOR_SYMBOL_CHAR: char = '␜';
pub const FILE_SEPARATOR_SYMBOL_STR: &str = "␜";

pub const ESCAPE_NAME: &str = "Escape (ESC)";
pub const ESCAPE_BRACE: &str = "{ESC}";
pub const ESCAPE_CONTROL_CHAR: char = '\u{001B}';
pub const ESCAPE_CONTROL_STR: &str = "\u{001B}";
pub const ESCAPE_SYMBOL_CHAR: char = '␛';
pub const ESCAPE_SYMBOL_STR: &str = "␛";

pub const END_OF_TRANSMISSION_NAME: &str = "End Of Transmission (EOT)";
pub const END_OF_TRANSMISSION_BRACE: &str = "{EOT}";
pub const END_OF_TRANSMISSION_CONTROL_CHAR: char = '\u{0004}';
pub const END_OF_TRANSMISSION_CONTROL_STR: &str = "\u{0004}";
pub const END_OF_TRANSMISSION_SYMBOL_CHAR: char = '␄';
pub const END_OF_TRANSMISSION_SYMBOL_STR: &str = "␄";

/// Style::braces() example of USV 2x2x2x2 data. This can be useful for demos and tests.
pub const STYLE_BRACES_EXAMPLE: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}i{US}j{US}{RS}k{US}l{US}{RS}{GS}m{US}n{US}{RS}o{US}p{US}{RS}{GS}{FS}";

/// Style::controls() example of USV 2x2x2x2 data. This can be useful for demos and tests.
pub const STYLE_CONTROLS_EXAMPLE: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}i\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}";

/// Style::symbols() example of USV 2x2x2x2 data. This can be useful for demos and tests.
pub const STYLE_SYMBOLS_EXAMPLE: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";

/// Style::liners() example of USV 2x2x2x2 data. This can be useful for demos and tests.
pub const STYLE_LINERS_EXAMPLE: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\ne\n␟\nf\n␟\n\n␞\ng\n␟\nh\n␟\n\n␞\n\n␝\n\n␜\ni\n␟\nj\n␟\n\n␞\nk\n␟\nl\n␟\n\n␞\n\n␝\nm\n␟\nn\n␟\n\n␞\no\n␟\np\n␟\n\n␞\n\n␝\n\n␜\n";

/// Style::sheets() example of USV 2x2x2x2 data. This can be useful for demos and tests.
pub const STYLE_SHEETS_EXAMPLE: &str = "a␟b␟␞\nc␟d␟␞\n␝\ne␟f␟␞\ng␟h␟␞\n␝\n␜\ni␟j␟␞\nk␟l␟␞\n␝\nm␟n␟␞\no␟p␟␞\n␝\n␜\n";
