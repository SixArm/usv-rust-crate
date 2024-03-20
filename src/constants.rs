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

/// Example Style::braces() with USV units. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_UNITS: &str = "a{US}b{US}";

/// Example Style::braces() with USV records. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_RECORDS: &str = "a{US}b{US}{RS}c{US}d{US}{RS}";

/// Example Style::braces() with USV groups. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_GROUPS: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}";

/// Example Style::braces() with USV files. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_FILES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}i{US}j{US}{RS}k{US}l{US}{RS}{GS}m{US}n{US}{RS}o{US}p{US}{RS}{GS}{FS}";

/// Example Style::controls() with USV units. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_UNITS: &str = "a\u{001F}b\u{001F}";

/// Example Style::controls() with USV records. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_RECORDS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}";

/// Example Style::controls() with USV groups. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_GROUPS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}";

/// Example Style::controls() with USV files. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_FILES: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}i\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}";

/// Example Style::symbols() with USV units. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_UNITS: &str = "a␟b␟";

/// Example Style::symbols() with USV records. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_RECORDS: &str = "a␟b␟␞c␟d␟␞";

/// Example Style::symbols() with USV groups. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_GROUPS: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";

/// Example Style::symbols() with USV files. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_FILES: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";

/// Example Style::liners() with USV units. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_LINERS_UNITS: &str = "a\n␟\nb\n␟\n";

/// Example Style::liners() with USV records. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_LINERS_RECORDS: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n";

/// Example Style::liners() with USV groups. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_LINERS_GROUPS: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\ne\n␟\nf\n␟\n\n␞\ng\n␟\nh\n␟\n\n␞\n\n␝\n";

/// Example Style::liners() with USV files. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_LINERS_FILES: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\ne\n␟\nf\n␟\n\n␞\ng\n␟\nh\n␟\n\n␞\n\n␝\n\n␜\ni\n␟\nj\n␟\n\n␞\nk\n␟\nl\n␟\n\n␞\n\n␝\nm\n␟\nn\n␟\n\n␞\no\n␟\np\n␟\n\n␞\n\n␝\n\n␜\n";

/// Example Style::sheets() with USV units. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SHEETS_UNITS: &str = "a␟b␟";

/// Example Style::sheets() with USV records. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SHEETS_RECORDS: &str = "a␟b␟␞\nc␟d␟␞\n";

/// Example Style::sheets() with USV groups. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SHEETS_GROUPS: &str = "a␟b␟␞\nc␟d␟␞\n␝\ne␟f␟␞\ng␟h␟␞\n␝\n";

/// Example Style::sheets() with USV files. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SHEETS_FILES: &str = "a␟b␟␞\nc␟d␟␞\n␝\ne␟f␟␞\ng␟h␟␞\n␝\n␜\ni␟j␟␞\nk␟l␟␞\n␝\nm␟n␟␞\no␟p␟␞\n␝\n␜\n";
