//! Examples of USV strings with styles and units. These can be useful for demos and tests.

//// General purpose examples, without any specific style, without any specific layout.

/// Example USV unit.
pub const EXAMPLE_UNIT: &str = "a";

/// Example USV units.
pub const EXAMPLE_UNITS: &str = "a␟b";

/// Example USV record.
pub const EXAMPLE_RECORD: &str = "a␟b";

/// Example USV records.
pub const EXAMPLE_RECORDS: &str = "a␟b␞c␟d";

/// Example USV group.
pub const EXAMPLE_GROUP: &str = "a␟b␞c␟d";

/// Example USV groups.
pub const EXAMPLE_GROUPS: &str = "a␟b␞c␟d␝e␟f␞g␟h";

/// Example USV file.
pub const EXAMPLE_FILE: &str = "a␟b␞c␟d␝e␟f␞g␟h";

/// Example USV files.
pub const EXAMPLE_FILES: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p";

//// With style symbols

/// Example USV unit with style symbols.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS: &str = "a";

/// Example USV units with style symbols.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS: &str = "a␟b";

/// Example USV record with style symbols.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS: &str = "a␟b";

/// Example USV records with style symbols.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS: &str = "a␟b␞c␟d";

/// Example USV group with style symbols.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS: &str = "a␟b␞c␟d";

/// Example USV groups with style symbols.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS: &str = "a␟b␞c␟d␝e␟f␞g␟h";

/// Example USV file with style symbols.
pub const EXAMPLE_FILE_STYLE_SYMBOLS: &str = "a␟b␞c␟d␝e␟f␞g␟h";

/// Example USV files with style symbols.
pub const EXAMPLE_FILES_STYLE_SYMBOLS: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p";

//// With style controls

/// Example USV unit with style controls.
pub const EXAMPLE_UNIT_STYLE_CONTROLS: &str = "a";

/// Example USV units with style controls.
pub const EXAMPLE_UNITS_STYLE_CONTROLS: &str = "a\u{001F}b";

/// Example USV record with style controls.
pub const EXAMPLE_RECORD_STYLE_CONTROLS: &str = "a\u{001F}b";

/// Example USV records with style controls.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS: &str = "a\u{001F}b\u{001E}c\u{001F}d";

/// Example USV group with style controls.
pub const EXAMPLE_GROUP_STYLE_CONTROLS: &str = "a\u{001F}b\u{001E}c\u{001F}d";

/// Example USV groups with style controls.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h";

/// Example USV file with style controls.
pub const EXAMPLE_FILE_STYLE_CONTROLS: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h";

/// Example USV files with style controls.
pub const EXAMPLE_FILES_STYLE_CONTROLS: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h\u{001C}i\u{001F}j\u{001E}k\u{001F}l\u{001D}m\u{001F}n\u{001E}o\u{001F}p";

//// With style braces

/// Example USV unit with style braces.
pub const EXAMPLE_UNIT_STYLE_BRACES: &str = "a";

/// Example USV units with style braces.
pub const EXAMPLE_UNITS_STYLE_BRACES: &str = "a{US}b";

/// Example USV record with style braces.
pub const EXAMPLE_RECORD_STYLE_BRACES: &str = "a{US}b";

/// Example USV records with style braces.
pub const EXAMPLE_RECORDS_STYLE_BRACES: &str = "a{US}b{RS}c{US}d";

/// Example USV group with style braces.
pub const EXAMPLE_GROUP_STYLE_BRACES: &str = "a{US}b{RS}c{US}d";

/// Example USV groups with style braces.
pub const EXAMPLE_GROUPS_STYLE_BRACES: &str = "a{US}b{RS}c{US}d{GS}e{US}f{RS}g{US}h";

/// Example USV file with style braces.
pub const EXAMPLE_FILE_STYLE_BRACES: &str = "a{US}b{RS}c{US}d{GS}e{US}f{RS}g{US}h";

/// Example USV files with style braces.
pub const EXAMPLE_FILES_STYLE_BRACES: &str = "a{US}b{RS}c{US}d{GS}e{US}f{RS}g{US}h{FS}i{US}j{RS}k{US}l{GS}m{US}n{RS}o{US}p";

//// With style symbols and layout 0

/// Example USV unit with style symbols and layout 0.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_0: &str = "a";

/// Example USV units with style symbols and layout 0.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b";

/// Example USV record with style symbols and layout 0.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b";

/// Example USV records with style symbols and layout 0.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␞c␟d";

/// Example USV group with style symbols and layout 0.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␞c␟d";

/// Example USV groups with style symbols and layout 0.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␞c␟d␝e␟f␞g␟h";

/// Example USV file with style symbols and layout 0.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␞c␟d␝e␟f␞g␟h";

/// Example USV files with style symbols and layout 0.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p";

//// With style symbols and layout 1

/// Example USV unit with style symbols and layout 1.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_1: &str = "a\n␟\n";

/// Example USV units with style symbols and layout 1.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_1: &str = "a\n␟\nb\n␟\n";

/// Example USV record with style symbols and layout 1.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_1: &str = "a\n␟\nb\n␟\n\n␞\n";

/// Example USV records with style symbols and layout 1.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_1: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n";

/// Example USV group with style symbols and layout 1.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_1: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\n";

/// Example USV groups with style symbols and layout 1.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_1: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\ne\n␟\nf\n␟\n\n␞\ng\n␟\nh\n␟\n\n␞\n\n␝\n";

/// Example USV file with style symbols and layout 1.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_1: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\ne\n␟\nf\n␟\n\n␞\ng\n␟\nh\n␟\n\n␞\n\n␝\n\n␜\n";

/// Example USV files with style symbols and layout 1.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_1: &str = "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\ne\n␟\nf\n␟\n\n␞\ng\n␟\nh\n␟\n\n␞\n\n␝\n\n␜\ni\n␟\nj\n␟\n\n␞\nk\n␟\nl\n␟\n\n␞\n\n␝\nm\n␟\nn\n␟\n\n␞\no\n␟\np\n␟\n\n␞\n\n␝\n\n␜\n";

//// With style symbols and layout 2

/// Example USV unit with style symbols and layout 2.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_2: &str = "a\n\n␟\n\n";

/// Example USV units with style symbols and layout 2.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_2: &str = "a\n\n␟\n\nb\n\n␟\n\n";

/// Example USV record with style symbols and layout 2.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_2: &str = "a\n\n␟\n\nb\n\n␟\n\n\n\n␞\n\n";

/// Example USV records with style symbols and layout 2.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_2: &str = "a\n\n␟\n\nb\n\n␟\n\n\n\n␞\n\nc\n\n␟\n\nd\n\n␟\n\n\n\n␞\n\n";

/// Example USV group with style symbols and layout 2.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_2: &str = "a\n\n␟\n\nb\n\n␟\n\n\n\n␞\n\nc\n\n␟\n\nd\n\n␟\n\n\n\n␞\n\n\n\n␝\n\n";

/// Example USV groups with style symbols and layout 2.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_2: &str = "a\n\n␟\n\nb\n\n␟\n\n\n\n␞\n\nc\n\n␟\n\nd\n\n␟\n\n\n\n␞\n\n\n\n␝\n\ne\n\n␟\n\nf\n\n␟\n\n\n\n␞\n\ng\n\n␟\n\nh\n\n␟\n\n\n\n␞\n\n\n\n␝\n\n";

/// Example USV file with style symbols and layout 2.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_2: &str = "a\n\n␟\n\nb\n\n␟\n\n\n\n␞\n\nc\n\n␟\n\nd\n\n␟\n\n\n\n␞\n\n\n\n␝\n\ne\n\n␟\n\nf\n\n␟\n\n\n\n␞\n\ng\n\n␟\n\nh\n\n␟\n\n\n\n␞\n\n\n\n␝\n\n\n\n␜\n\n";

/// Example USV files with style symbols and layout 2.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_2: &str = "a\n\n␟\n\nb\n\n␟\n\n\n\n␞\n\nc\n\n␟\n\nd\n\n␟\n\n\n\n␞\n\n\n\n␝\n\ne\n\n␟\n\nf\n\n␟\n\n\n\n␞\n\ng\n\n␟\n\nh\n\n␟\n\n\n\n␞\n\n\n\n␝\n\n\n\n␜\n\ni\n\n␟\n\nj\n\n␟\n\n\n\n␞\n\nk\n\n␟\n\nl\n\n␟\n\n\n\n␞\n\n\n\n␝\n\nm\n\n␟\n\nn\n\n␟\n\n\n\n␞\n\no\n\n␟\n\np\n\n␟\n\n\n\n␞\n\n\n\n␝\n\n\n\n␜\n\n";

//// With style symbols and layout units

/// Example USV unit with style symbols and layout records.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n";

/// Example USV units with style symbols and layout records.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n";

/// Example USV record with style symbols and layout records.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\n";

/// Example USV records with style symbols and layout records.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n";

/// Example USV group with style symbols and layout records.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n␝\n";

/// Example USV groups with style symbols and layout records.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n␝\ne␟\nf␟\n␞\ng␟\nh␟\n␞\n␝\n";

/// Example USV file with style symbols and layout records.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n␝\ne␟\nf␟\n␞\ng␟\nh␟\n␞\n␝\n␜\n";

/// Example USV files with style symbols and layout records.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n␝\ne␟\nf␟\n␞\ng␟\nh␟\n␞\n␝\n␜\ni␟\nj␟\n␞\nk␟\nl␟\n␞\n␝\nm␟\nn␟\n␞\no␟\np␟\n␞\n␝\n␜\n";

//// With style symbols and layout records

/// Example USV unit with style symbols and layout records.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b";

/// Example USV units with style symbols and layout records.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b";

/// Example USV record with style symbols and layout records.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␞\n";

/// Example USV records with style symbols and layout records.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␞\nc␟d␞\n";

/// Example USV group with style symbols and layout records.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␞\nc␟d␞\n␝\n";

/// Example USV groups with style symbols and layout records.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␞\nc␟d␞\n␝\ne␟f␞\ng␟h␞\n␝\n";

/// Example USV file with style symbols and layout records.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␞\nc␟d␞\n␝\ne␟f␞\ng␟h␞\n␝\n␜\n";

/// Example USV files with style symbols and layout records.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␞\nc␟d␞\n␝\ne␟f␞\ng␟h␞\n␝\n␜\ni␟j␞\nk␟l␞\n␝\nm␟n␞\no␟p␞\n␝\n␜\n";

//// With style symbols and layout groups

/// Example USV unit with style symbols and layout records.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b";

/// Example USV units with style symbols and layout records.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b";

/// Example USV record with style symbols and layout records.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b";

/// Example USV records with style symbols and layout records.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␞c␟d";

/// Example USV group with style symbols and layout records.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␞c␟d␝\n";

/// Example USV groups with style symbols and layout records.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␞c␟d␝\ne␟f␞g␟h␝\n";

/// Example USV file with style symbols and layout records.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␞c␟d␝\ne␟f␞g␟h␝\n␜\n";

/// Example USV files with style symbols and layout records.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␞c␟d␝\ne␟f␞g␟h␝\n␜\ni␟j␞k␟l␝\nm␟n␞o␟p␝\n␜\n";

//// With style symbols and layout files

/// Example USV unit with style symbols and layout records.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b";

/// Example USV units with style symbols and layout records.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b";

/// Example USV record with style symbols and layout records.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b";

/// Example USV records with style symbols and layout records.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␞c␟d";

/// Example USV group with style symbols and layout records.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␞c␟d";

/// Example USV groups with style symbols and layout records.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␞c␟d␝e␟f␞g␟h";

/// Example USV file with style symbols and layout records.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␞c␟d␝e␟f␞g␟h␜\n";

/// Example USV files with style symbols and layout records.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␞c␟d␝e␟f␞g␟h␜\ni␟j␞k␟l␝m␟n␞o␟p␜\n";

//// With style controls and layout 0

/// Example USV unit with style controls and layout 0.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_0: &str = "a";

/// Example USV units with style controls and layout 0.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b";

/// Example USV record with style controls and layout 0.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b";

/// Example USV records with style controls and layout 0.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001E}c\u{001F}d";

/// Example USV group with style controls and layout 0.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001E}c\u{001F}d";

/// Example USV groups with style controls and layout 0.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h";

/// Example USV file with style controls and layout 0.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h";

/// Example USV files with style controls and layout 0.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h\u{001C}i\u{001F}j\u{001E}k\u{001F}l\u{001D}m\u{001F}n\u{001E}o\u{001F}p";

//// With style controls and layout 1

/// Example USV unit with style controls and layout 1.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_1: &str = "a\n\u{001F}\n";

/// Example USV units with style controls and layout 1.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_1: &str = "a\n\u{001F}\nb\n\u{001F}\n";

/// Example USV record with style controls and layout 1.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_1: &str = "a\n\u{001F}\nb\n\u{001F}\n\n\u{001E}\n";

/// Example USV records with style controls and layout 1.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_1: &str = "a\n\u{001F}\nb\n\u{001F}\n\n\u{001E}\nc\n\u{001F}\nd\n\u{001F}\n\n\u{001E}\n";

/// Example USV group with style controls and layout 1.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_1: &str = "a\n\u{001F}\nb\n\u{001F}\n\n\u{001E}\nc\n\u{001F}\nd\n\u{001F}\n\n\u{001E}\n\n\u{001D}\n";

/// Example USV groups with style controls and layout 1.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_1: &str = "a\n\u{001F}\nb\n\u{001F}\n\n\u{001E}\nc\n\u{001F}\nd\n\u{001F}\n\n\u{001E}\n\n\u{001D}\ne\n\u{001F}\nf\n\u{001F}\n\n\u{001E}\ng\n\u{001F}\nh\n\u{001F}\n\n\u{001E}\n\n\u{001D}\n";

/// Example USV file with style controls and layout 1.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_1: &str = "a\n\u{001F}\nb\n\u{001F}\n\n\u{001E}\nc\n\u{001F}\nd\n\u{001F}\n\n\u{001E}\n\n\u{001D}\ne\n\u{001F}\nf\n\u{001F}\n\n\u{001E}\ng\n\u{001F}\nh\n\u{001F}\n\n\u{001E}\n\n\u{001D}\n\n\u{001C}\n";

/// Example USV files with style controls and layout 1.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_1: &str = "a\n\u{001F}\nb\n\u{001F}\n\n\u{001E}\nc\n\u{001F}\nd\n\u{001F}\n\n\u{001E}\n\n\u{001D}\ne\n\u{001F}\nf\n\u{001F}\n\n\u{001E}\ng\n\u{001F}\nh\n\u{001F}\n\n\u{001E}\n\n\u{001D}\n\n\u{001C}\ni\n\u{001F}\nj\n\u{001F}\n\n\u{001E}\nk\n\u{001F}\nl\n\u{001F}\n\n\u{001E}\n\n\u{001D}\nm\n\u{001F}\nn\n\u{001F}\n\n\u{001E}\no\n\u{001F}\np\n\u{001F}\n\n\u{001E}\n\n\u{001D}\n\n\u{001C}\n";

//// With style controls and layout 2

/// Example USV unit with style controls and layout 2.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_2: &str = "a\n\n\u{001F}\n\n";

/// Example USV units with style controls and layout 2.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_2: &str = "a\n\n\u{001F}\n\nb\n\n\u{001F}\n\n";

/// Example USV record with style controls and layout 2.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_2: &str = "a\n\n\u{001F}\n\nb\n\n\u{001F}\n\n\n\n\u{001E}\n\n";

/// Example USV records with style controls and layout 2.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_2: &str = "a\n\n\u{001F}\n\nb\n\n\u{001F}\n\n\n\n\u{001E}\n\nc\n\n\u{001F}\n\nd\n\n\u{001F}\n\n\n\n\u{001E}\n\n";

/// Example USV group with style controls and layout 2.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_2: &str = "a\n\n\u{001F}\n\nb\n\n\u{001F}\n\n\n\n\u{001E}\n\nc\n\n\u{001F}\n\nd\n\n\u{001F}\n\n\n\n\u{001E}\n\n\n\n\u{001D}\n\n";

/// Example USV groups with style controls and layout 2.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_2: &str = "a\n\n\u{001F}\n\nb\n\n\u{001F}\n\n\n\n\u{001E}\n\nc\n\n\u{001F}\n\nd\n\n\u{001F}\n\n\n\n\u{001E}\n\n\n\n\u{001D}\n\ne\n\n\u{001F}\n\nf\n\n\u{001F}\n\n\n\n\u{001E}\n\ng\n\n\u{001F}\n\nh\n\n\u{001F}\n\n\n\n\u{001E}\n\n\n\n\u{001D}\n\n";

/// Example USV file with style controls and layout 2.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_2: &str = "a\n\n\u{001F}\n\nb\n\n\u{001F}\n\n\n\n\u{001E}\n\nc\n\n\u{001F}\n\nd\n\n\u{001F}\n\n\n\n\u{001E}\n\n\n\n\u{001D}\n\ne\n\n\u{001F}\n\nf\n\n\u{001F}\n\n\n\n\u{001E}\n\ng\n\n\u{001F}\n\nh\n\n\u{001F}\n\n\n\n\u{001E}\n\n\n\n\u{001D}\n\n\n\n\u{001C}\n\n";

/// Example USV files with style controls and layout 2.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_2: &str = "a\n\n\u{001F}\n\nb\n\n\u{001F}\n\n\n\n\u{001E}\n\nc\n\n\u{001F}\n\nd\n\n\u{001F}\n\n\n\n\u{001E}\n\n\n\n\u{001D}\n\ne\n\n\u{001F}\n\nf\n\n\u{001F}\n\n\n\n\u{001E}\n\ng\n\n\u{001F}\n\nh\n\n\u{001F}\n\n\n\n\u{001E}\n\n\n\n\u{001D}\n\n\n\n\u{001C}\n\ni\n\n\u{001F}\n\nj\n\n\u{001F}\n\n\n\n\u{001E}\n\nk\n\n\u{001F}\n\nl\n\n\u{001F}\n\n\n\n\u{001E}\n\n\n\n\u{001D}\n\nm\n\n\u{001F}\n\nn\n\n\u{001F}\n\n\n\n\u{001E}\n\no\n\n\u{001F}\n\np\n\n\u{001F}\n\n\n\n\u{001E}\n\n\n\n\u{001D}\n\n\n\n\u{001C}\n\n";

//// With style controls and layout units

/// Example USV unit with style controls and layout records.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n";

/// Example USV units with style controls and layout records.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n";

/// Example USV record with style controls and layout records.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n\u{001E}\n";

/// Example USV records with style controls and layout records.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n\u{001E}\nc\u{001F}\nd\u{001F}\n\u{001E}\n";

/// Example USV group with style controls and layout records.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n\u{001E}\nc\u{001F}\nd\u{001F}\n\u{001E}\n\u{001D}\n";

/// Example USV groups with style controls and layout records.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n\u{001E}\nc\u{001F}\nd\u{001F}\n\u{001E}\n\u{001D}\ne\u{001F}\nf\u{001F}\n\u{001E}\ng\u{001F}\nh\u{001F}\n\u{001E}\n\u{001D}\n";

/// Example USV file with style controls and layout records.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n\u{001E}\nc\u{001F}\nd\u{001F}\n\u{001E}\n\u{001D}\ne\u{001F}\nf\u{001F}\n\u{001E}\ng\u{001F}\nh\u{001F}\n\u{001E}\n\u{001D}\n\u{001C}\n";

/// Example USV files with style controls and layout records.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n\u{001E}\nc\u{001F}\nd\u{001F}\n\u{001E}\n\u{001D}\ne\u{001F}\nf\u{001F}\n\u{001E}\ng\u{001F}\nh\u{001F}\n\u{001E}\n\u{001D}\n\u{001C}\ni\u{001F}\nj\u{001F}\n\u{001E}\nk\u{001F}\nl\u{001F}\n\u{001E}\n\u{001D}\nm\u{001F}\nn\u{001F}\n\u{001E}\no\u{001F}\np\u{001F}\n\u{001E}\n\u{001D}\n\u{001C}\n";

//// With style controls and layout records

/// Example USV unit with style controls and layout records.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b";

/// Example USV units with style controls and layout records.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b";

/// Example USV record with style controls and layout records.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001E}\n";

/// Example USV records with style controls and layout records.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001E}\nc\u{001F}d\u{001E}\n";

/// Example USV group with style controls and layout records.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001E}\nc\u{001F}d\u{001E}\n\u{001D}\n";

/// Example USV groups with style controls and layout records.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001E}\nc\u{001F}d\u{001E}\n\u{001D}\ne\u{001F}f\u{001E}\ng\u{001F}h\u{001E}\n\u{001D}\n";

/// Example USV file with style controls and layout records.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001E}\nc\u{001F}d\u{001E}\n\u{001D}\ne\u{001F}f\u{001E}\ng\u{001F}h\u{001E}\n\u{001D}\n\u{001C}\n";

/// Example USV files with style controls and layout records.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001E}\nc\u{001F}d\u{001E}\n\u{001D}\ne\u{001F}f\u{001E}\ng\u{001F}h\u{001E}\n\u{001D}\n\u{001C}\ni\u{001F}j\u{001E}\nk\u{001F}l\u{001E}\n\u{001D}\nm\u{001F}n\u{001E}\no\u{001F}p\u{001E}\n\u{001D}\n\u{001C}\n";

//// With style controls and layout groups

/// Example USV unit with style controls and layout records.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b";

/// Example USV units with style controls and layout records.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b";

/// Example USV record with style controls and layout records.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b";

/// Example USV records with style controls and layout records.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001E}c\u{001F}d";

/// Example USV group with style controls and layout records.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}\n";

/// Example USV groups with style controls and layout records.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}\ne\u{001F}f\u{001E}g\u{001F}h\u{001D}\n";

/// Example USV file with style controls and layout records.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}\ne\u{001F}f\u{001E}g\u{001F}h\u{001D}\n\u{001C}\n";

/// Example USV files with style controls and layout records.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}\ne\u{001F}f\u{001E}g\u{001F}h\u{001D}\n\u{001C}\ni\u{001F}j\u{001E}k\u{001F}l\u{001D}\nm\u{001F}n\u{001E}o\u{001F}p\u{001D}\n\u{001C}\n";

//// With style controls and layout files

/// Example USV unit with style controls and layout records.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b";

/// Example USV units with style controls and layout records.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b";

/// Example USV record with style controls and layout records.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b";

/// Example USV records with style controls and layout records.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001E}c\u{001F}d";

/// Example USV group with style controls and layout records.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001E}c\u{001F}d";

/// Example USV groups with style controls and layout records.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h";

/// Example USV file with style controls and layout records.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h\u{001C}\n";

/// Example USV files with style controls and layout records.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h\u{001C}\ni\u{001F}j\u{001E}k\u{001F}l\u{001D}m\u{001F}n\u{001E}o\u{001F}p\u{001C}\n";

//// With style braces and layout 0

/// Example USV unit with style braces and layout 0.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_0: &str = "a";

/// Example USV units with style braces and layout 0.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_0: &str = "a{US}b";

/// Example USV record with style braces and layout 0.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_0: &str = "a{US}b";

/// Example USV records with style braces and layout 0.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{RS}c{US}d";

/// Example USV group with style braces and layout 0.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{RS}c{US}d";

/// Example USV groups with style braces and layout 0.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{RS}c{US}d{GS}e{US}f{RS}g{US}h";

/// Example USV file with style braces and layout 0.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{RS}c{US}d{GS}e{US}f{RS}g{US}h";

/// Example USV files with style braces and layout 0.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{RS}c{US}d{GS}e{US}f{RS}g{US}h{FS}i{US}j{RS}k{US}l{GS}m{US}n{RS}o{US}p";

//// With style braces and layout 1

/// Example USV unit with style braces and layout 1.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_1: &str = "a\n{US}\n";

/// Example USV units with style braces and layout 1.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_1: &str = "a\n{US}\nb\n{US}\n";

/// Example USV record with style braces and layout 1.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_1: &str = "a\n{US}\nb\n{US}\n\n{RS}\n";

/// Example USV records with style braces and layout 1.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_1: &str = "a\n{US}\nb\n{US}\n\n{RS}\nc\n{US}\nd\n{US}\n\n{RS}\n";

/// Example USV group with style braces and layout 1.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_1: &str = "a\n{US}\nb\n{US}\n\n{RS}\nc\n{US}\nd\n{US}\n\n{RS}\n\n{GS}\n";

/// Example USV groups with style braces and layout 1.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_1: &str = "a\n{US}\nb\n{US}\n\n{RS}\nc\n{US}\nd\n{US}\n\n{RS}\n\n{GS}\ne\n{US}\nf\n{US}\n\n{RS}\ng\n{US}\nh\n{US}\n\n{RS}\n\n{GS}\n";

/// Example USV file with style braces and layout 1.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_1: &str = "a\n{US}\nb\n{US}\n\n{RS}\nc\n{US}\nd\n{US}\n\n{RS}\n\n{GS}\ne\n{US}\nf\n{US}\n\n{RS}\ng\n{US}\nh\n{US}\n\n{RS}\n\n{GS}\n\n{FS}\n";

/// Example USV files with style braces and layout 1.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_1: &str = "a\n{US}\nb\n{US}\n\n{RS}\nc\n{US}\nd\n{US}\n\n{RS}\n\n{GS}\ne\n{US}\nf\n{US}\n\n{RS}\ng\n{US}\nh\n{US}\n\n{RS}\n\n{GS}\n\n{FS}\ni\n{US}\nj\n{US}\n\n{RS}\nk\n{US}\nl\n{US}\n\n{RS}\n\n{GS}\nm\n{US}\nn\n{US}\n\n{RS}\no\n{US}\np\n{US}\n\n{RS}\n\n{GS}\n\n{FS}\n";

//// With style braces and layout 2

/// Example USV unit with style braces and layout 2.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_2: &str = "a\n\n{US}\n\n";

/// Example USV units with style braces and layout 2.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_2: &str = "a\n\n{US}\n\nb\n\n{US}\n\n";

/// Example USV record with style braces and layout 2.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_2: &str = "a\n\n{US}\n\nb\n\n{US}\n\n\n\n{RS}\n\n";

/// Example USV records with style braces and layout 2.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_2: &str = "a\n\n{US}\n\nb\n\n{US}\n\n\n\n{RS}\n\nc\n\n{US}\n\nd\n\n{US}\n\n\n\n{RS}\n\n";

/// Example USV group with style braces and layout 2.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_2: &str = "a\n\n{US}\n\nb\n\n{US}\n\n\n\n{RS}\n\nc\n\n{US}\n\nd\n\n{US}\n\n\n\n{RS}\n\n\n\n{GS}\n\n";

/// Example USV groups with style braces and layout 2.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_2: &str = "a\n\n{US}\n\nb\n\n{US}\n\n\n\n{RS}\n\nc\n\n{US}\n\nd\n\n{US}\n\n\n\n{RS}\n\n\n\n{GS}\n\ne\n\n{US}\n\nf\n\n{US}\n\n\n\n{RS}\n\ng\n\n{US}\n\nh\n\n{US}\n\n\n\n{RS}\n\n\n\n{GS}\n\n";

/// Example USV file with style braces and layout 2.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_2: &str = "a\n\n{US}\n\nb\n\n{US}\n\n\n\n{RS}\n\nc\n\n{US}\n\nd\n\n{US}\n\n\n\n{RS}\n\n\n\n{GS}\n\ne\n\n{US}\n\nf\n\n{US}\n\n\n\n{RS}\n\ng\n\n{US}\n\nh\n\n{US}\n\n\n\n{RS}\n\n\n\n{GS}\n\n\n\n{FS}\n\n";

/// Example USV files with style braces and layout 2.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_2: &str = "a\n\n{US}\n\nb\n\n{US}\n\n\n\n{RS}\n\nc\n\n{US}\n\nd\n\n{US}\n\n\n\n{RS}\n\n\n\n{GS}\n\ne\n\n{US}\n\nf\n\n{US}\n\n\n\n{RS}\n\ng\n\n{US}\n\nh\n\n{US}\n\n\n\n{RS}\n\n\n\n{GS}\n\n\n\n{FS}\n\ni\n\n{US}\n\nj\n\n{US}\n\n\n\n{RS}\n\nk\n\n{US}\n\nl\n\n{US}\n\n\n\n{RS}\n\n\n\n{GS}\n\nm\n\n{US}\n\nn\n\n{US}\n\n\n\n{RS}\n\no\n\n{US}\n\np\n\n{US}\n\n\n\n{RS}\n\n\n\n{GS}\n\n\n\n{FS}\n\n";

//// With style braces and layout units

/// Example USV unit with style braces and layout units.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_UNITS: &str = "a{US}\nb{US}\n";

/// Example USV units with style braces and layout units.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_UNITS: &str = "a{US}\nb{US}\n";

/// Example USV record with style braces and layout units.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_UNITS: &str = "a{US}\nb{US}\n{RS}\n";

/// Example USV records with style braces and layout units.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_UNITS: &str = "a{US}\nb{US}\n{RS}\nc{US}\nd{US}\n{RS}\n";

/// Example USV group with style braces and layout units.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_UNITS: &str = "a{US}\nb{US}\n{RS}\nc{US}\nd{US}\n{RS}\n{GS}\n";

/// Example USV groups with style braces and layout units.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_UNITS: &str = "a{US}\nb{US}\n{RS}\nc{US}\nd{US}\n{RS}\n{GS}\ne{US}\nf{US}\n{RS}\ng{US}\nh{US}\n{RS}\n{GS}\n";

/// Example USV file with style braces and layout units.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_UNITS: &str = "a{US}\nb{US}\n{RS}\nc{US}\nd{US}\n{RS}\n{GS}\ne{US}\nf{US}\n{RS}\ng{US}\nh{US}\n{RS}\n{GS}\n{FS}\n";

/// Example USV files with style braces and layout units.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_UNITS: &str = "a{US}\nb{US}\n{RS}\nc{US}\nd{US}\n{RS}\n{GS}\ne{US}\nf{US}\n{RS}\ng{US}\nh{US}\n{RS}\n{GS}\n{FS}\ni{US}\nj{US}\n{RS}\nk{US}\nl{US}\n{RS}\n{GS}\nm{US}\nn{US}\n{RS}\no{US}\np{US}\n{RS}\n{GS}\n{FS}\n";

//// With style braces and layout records

/// Example USV unit with style braces and layout records.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b";

/// Example USV units with style braces and layout records.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b";

/// Example USV record with style braces and layout records.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{RS}\n";

/// Example USV records with style braces and layout records.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{RS}\nc{US}d{RS}\n";

/// Example USV group with style braces and layout records.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{RS}\nc{US}d{RS}\n{GS}\n";

/// Example USV groups with style braces and layout records.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{RS}\nc{US}d{RS}\n{GS}\ne{US}f{RS}\ng{US}h{RS}\n{GS}\n";

/// Example USV file with style braces and layout records.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{RS}\nc{US}d{RS}\n{GS}\ne{US}f{RS}\ng{US}h{RS}\n{GS}\n{FS}\n";

/// Example USV files with style braces and layout records.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{RS}\nc{US}d{RS}\n{GS}\ne{US}f{RS}\ng{US}h{RS}\n{GS}\n{FS}\ni{US}j{RS}\nk{US}l{RS}\n{GS}\nm{US}n{RS}\no{US}p{RS}\n{GS}\n{FS}\n";

//// With style braces and layout groups

/// Example USV unit with style braces and layout groups.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b";

/// Example USV units with style braces and layout groups.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b";

/// Example USV record with style braces and layout groups.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b";

/// Example USV records with style braces and layout groups.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{RS}c{US}d";

/// Example USV group with style braces and layout groups.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{RS}c{US}d{GS}\n";

/// Example USV groups with style braces and layout groups.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{RS}c{US}d{GS}\ne{US}f{RS}g{US}h{GS}\n";

/// Example USV file with style braces and layout groups.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{RS}c{US}d{GS}\ne{US}f{RS}g{US}h{GS}\n{FS}\n";

/// Example USV files with style braces and layout groups.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{RS}c{US}d{GS}\ne{US}f{RS}g{US}h{GS}\n{FS}\ni{US}j{RS}k{US}l{GS}\nm{US}n{RS}o{US}p{GS}\n{FS}\n";

//// With style braces and layout files

/// Example USV unit with style braces and layout files.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b";

/// Example USV units with style braces and layout files.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b";

/// Example USV record with style braces and layout files.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b";

/// Example USV records with style braces and layout files.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{RS}c{US}d";

/// Example USV group with style braces and layout files.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{RS}c{US}d";

/// Example USV groups with style braces and layout files.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{RS}c{US}d{GS}e{US}f{RS}g{US}h";

/// Example USV file with style braces and layout files.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{RS}c{US}d{GS}e{US}f{RS}g{US}h{FS}\n";

/// Example USV files with style braces and layout files.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{RS}c{US}d{GS}e{US}f{RS}g{US}h{FS}\ni{US}j{RS}k{US}l{GS}m{US}n{RS}o{US}p{FS}\n";

//// Arrays

/// Example array with USV units
pub const EXAMPLE_ARRAY_UNITS: [&str;2] = ["a","b"];

/// Example array with USV units
pub const EXAMPLE_ARRAY_RECORDS: [[&str;2];2] = [["a","b"],["c","d"]];

/// Example array with USV files
pub const EXAMPLE_ARRAY_GROUPS: [[[&str;2];2];2] = [[["a","b"],["c","d"]],[["e","f"],["g","h"]]];

/// Example array with USV files
pub const EXAMPLE_ARRAY_FILES: [[[[&str;2];2];2];2] = [[[["a","b"],["c","d"]],[["e","f"],["g","h"]]],[[["i","j"],["k","l"]],[["m","n"],["o","p"]]]];

//// Spreadsheets + style symbols

/// Example spreadsheet sheet with style symbols.
pub const EXAMPLE_SHEET_STYLE_SYMBOLS: &str = "Sheet1␞a␟b␞c␟d";

/// Example spreadsheet sheets with style symbols.
pub const EXAMPLE_SHEETS_STYLE_SYMBOLS: &str = "Sheet1␞a␟b␞c␟d␝Sheet2␞e␟f␞g␟h";

/// Example spreadsheet folio with style symbols.
pub const EXAMPLE_FOLIO_STYLE_SYMBOLS: &str = "Sheet1␞a␟b␞c␟d␝Sheet2␞e␟f␞g␟h";

/// Example spreadsheet folios with style symbols.
pub const EXAMPLE_FOLIOS_STYLE_SYMBOLS: &str = "Sheet1␞a␟b␞c␟d␝Sheet2␞e␟f␞g␟h␜Sheet1␞i␟j␞k␟l␝Sheet2␞m␟n␞o␟p";

//// Spreadsheets + style controls

/// Example spreadsheet sheet with style controls.
pub const EXAMPLE_SHEET_STYLE_CONTROLS: &str = "Sheet1\u{001E}a\u{001F}b\u{001E}c\u{001F}d";

/// Example spreadsheet sheets with style controls.
pub const EXAMPLE_SHEETS_STYLE_CONTROLS: &str = "Sheet1\u{001E}a\u{001F}b\u{001E}c\u{001F}d\u{001D}Sheet2\u{001E}e\u{001F}f\u{001E}g\u{001F}h";

/// Example spreadsheet folio with style controls.
pub const EXAMPLE_FOLIO_STYLE_CONTROLS: &str = "Sheet1\u{001E}a\u{001F}b\u{001E}c\u{001F}d\u{001D}Sheet2\u{001E}e\u{001F}f\u{001E}g\u{001F}h";

/// Example spreadsheet folios with style controls.
pub const EXAMPLE_FOLIOS_STYLE_CONTROLS: &str = "Sheet1\u{001E}a\u{001F}b\u{001E}c\u{001F}d\u{001D}Sheet2\u{001E}e\u{001F}f\u{001E}g\u{001F}h\u{001C}Sheet1\u{001E}i\u{001F}j\u{001E}k\u{001F}l\u{001E}Sheet2\u{001D}m\u{001F}n\u{001E}o\u{001F}p";

//// Spreadsheets + style braces

/// Example spreadsheet sheet with style braces.
pub const EXAMPLE_SHEET_STYLE_BRACES: &str = "Sheet1{RS}a{US}b{RS}c{US}d";

/// Example spreadsheet sheets with style braces.
pub const EXAMPLE_SHEETS_STYLE_BRACES: &str = "Sheet1{RS}a{US}b{RS}c{US}d{GS}Sheet2{RS}e{US}f{RS}g{US}h";

/// Example spreadsheet folio with style braces.
pub const EXAMPLE_FOLIO_STYLE_BRACES: &str = "Sheet1{RS}a{US}b{RS}c{US}d{GS}Sheet2{RS}e{US}f{RS}g{US}h";

/// Example spreadsheet folios with style braces.
pub const EXAMPLE_FOLIOS_STYLE_BRACES: &str = "Sheet1{RS}a{US}b{RS}c{US}d{GS}Sheet2{RS}e{US}f{RS}g{US}h{FS}Sheet1{RS}i{US}j{RS}k{US}l{GS}Sheet2{RS}m{US}n{RS}o{US}p";
