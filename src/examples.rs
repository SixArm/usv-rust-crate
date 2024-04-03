//! Examples of USV strings with styles and units. These can be useful for demos and tests.

//// General purpose examples, without any specific style, without any specific layout.

/// Example USV unit.
pub const EXAMPLE_UNIT: &str = "a␟";

/// Example USV units.
pub const EXAMPLE_UNITS: &str = "a␟b␟";

/// Example USV record.
pub const EXAMPLE_RECORD: &str = "a␟b␟␞";

/// Example USV records.
pub const EXAMPLE_RECORDS: &str = "a␟b␟␞c␟d␟␞";

/// Example USV group.
pub const EXAMPLE_GROUP: &str = "a␟b␟␞c␟d␟␞␝";

/// Example USV groups.
pub const EXAMPLE_GROUPS: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";

/// Example USV file.
pub const EXAMPLE_FILE: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜";

/// Example USV files.
pub const EXAMPLE_FILES: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";

//// With style symbols

/// Example USV unit with style symbols.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS: &str = "a␟";

/// Example USV units with style symbols.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS: &str = "a␟b␟";

/// Example USV record with style symbols.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS: &str = "a␟b␟␞";

/// Example USV records with style symbols.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS: &str = "a␟b␟␞c␟d␟␞";

/// Example USV group with style symbols.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS: &str = "a␟b␟␞c␟d␟␞␝";

/// Example USV groups with style symbols.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";

/// Example USV file with style symbols.
pub const EXAMPLE_FILE_STYLE_SYMBOLS: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜";

/// Example USV files with style symbols.
pub const EXAMPLE_FILES_STYLE_SYMBOLS: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";

//// With style controls

/// Example USV unit with style controls.
pub const EXAMPLE_UNIT_STYLE_CONTROLS: &str = "a\u{001F}";

/// Example USV units with style controls.
pub const EXAMPLE_UNITS_STYLE_CONTROLS: &str = "a\u{001F}b\u{001F}";

/// Example USV record with style controls.
pub const EXAMPLE_RECORD_STYLE_CONTROLS: &str = "a\u{001F}b\u{001F}\u{001E}";

/// Example USV records with style controls.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}";

/// Example USV group with style controls.
pub const EXAMPLE_GROUP_STYLE_CONTROLS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}";

/// Example USV groups with style controls.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}";

/// Example USV file with style controls.
pub const EXAMPLE_FILE_STYLE_CONTROLS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}";

/// Example USV files with style controls.
pub const EXAMPLE_FILES_STYLE_CONTROLS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}i\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}";

//// With style braces

/// Example USV unit with style braces.
pub const EXAMPLE_UNIT_STYLE_BRACES: &str = "a{US}";

/// Example USV units with style braces.
pub const EXAMPLE_UNITS_STYLE_BRACES: &str = "a{US}b{US}";

/// Example USV record with style braces.
pub const EXAMPLE_RECORD_STYLE_BRACES: &str = "a{US}b{US}{RS}";

/// Example USV records with style braces.
pub const EXAMPLE_RECORDS_STYLE_BRACES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}";

/// Example USV group with style braces.
pub const EXAMPLE_GROUP_STYLE_BRACES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}";

/// Example USV groups with style braces.
pub const EXAMPLE_GROUPS_STYLE_BRACES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}";

/// Example USV file with style braces.
pub const EXAMPLE_FILE_STYLE_BRACES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}";

/// Example USV files with style braces.
pub const EXAMPLE_FILES_STYLE_BRACES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}i{US}j{US}{RS}k{US}l{US}{RS}{GS}m{US}n{US}{RS}o{US}p{US}{RS}{GS}{FS}";

//// With style symbols and layout 0

/// Example USV unit with style symbols and layout 0.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟";

/// Example USV units with style symbols and layout 0.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␟";

/// Example USV record with style symbols and layout 0.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␟␞";

/// Example USV records with style symbols and layout 0.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␟␞c␟d␟␞";

/// Example USV group with style symbols and layout 0.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␟␞c␟d␟␞␝";

/// Example USV groups with style symbols and layout 0.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";

/// Example USV file with style symbols and layout 0.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜";

/// Example USV files with style symbols and layout 0.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_0: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";

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
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_1: &str =  "a\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\ne\n␟\nf\n␟\n\n␞\ng\n␟\nh\n␟\n\n␞\n\n␝\n\n␜\n";

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

/// Example USV unit with style symbols and layout units.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\n";

/// Example USV units with style symbols and layout units.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n";

/// Example USV record with style symbols and layout units.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\n";

/// Example USV records with style symbols and layout units.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n";

/// Example USV group with style symbols and layout units.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n␝\n";

/// Example USV groups with style symbols and layout units.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n␝\ne␟\nf␟\n␞\ng␟\nh␟\n␞\n␝\n";

/// Example USV file with style symbols and layout units.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n␝\ne␟\nf␟\n␞\ng␟\nh␟\n␞\n␝\n␜\n";

/// Example USV files with style symbols and layout units.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_UNITS: &str = "a␟\nb␟\n␞\nc␟\nd␟\n␞\n␝\ne␟\nf␟\n␞\ng␟\nh␟\n␞\n␝\n␜\ni␟\nj␟\n␞\nk␟\nl␟\n␞\n␝\nm␟\nn␟\n␞\no␟\np␟\n␞\n␝\n␜\n";

//// With style symbols and layout records

/// Example USV unit with style symbols and layout records.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟";

/// Example USV units with style symbols and layout records.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␟";

/// Example USV record with style symbols and layout records.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␟␞\n";

/// Example USV records with style symbols and layout records.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␟␞\nc␟d␟␞\n";

/// Example USV group with style symbols and layout records.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␟␞\nc␟d␟␞\n␝\n";

/// Example USV groups with style symbols and layout records.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␟␞\nc␟d␟␞\n␝\ne␟f␟␞\ng␟h␟␞\n␝\n";

/// Example USV file with style symbols and layout records.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␟␞\nc␟d␟␞\n␝\ne␟f␟␞\ng␟h␟␞\n␝\n␜\n";

/// Example USV files with style symbols and layout records.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_RECORDS: &str = "a␟b␟␞\nc␟d␟␞\n␝\ne␟f␟␞\ng␟h␟␞\n␝\n␜\ni␟j␟␞\nk␟l␟␞\n␝\nm␟n␟␞\no␟p␟␞\n␝\n␜\n";

//// With style symbols and layout groups

/// Example USV unit with style symbols and layout groups.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟";

/// Example USV units with style symbols and layout groups.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␟";

/// Example USV record with style symbols and layout groups.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␟␞";

/// Example USV records with style symbols and layout groups.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␟␞c␟d␟␞";

/// Example USV group with style symbols and layout groups.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␟␞c␟d␟␞␝\n";

/// Example USV groups with style symbols and layout groups.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␟␞c␟d␟␞␝\ne␟f␟␞g␟h␟␞␝\n";

/// Example USV file with style symbols and layout groups.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␟␞c␟d␟␞␝\ne␟f␟␞g␟h␟␞␝\n␜\n";

/// Example USV files with style symbols and layout groups.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_GROUPS: &str = "a␟b␟␞c␟d␟␞␝\ne␟f␟␞g␟h␟␞␝\n␜\ni␟j␟␞k␟l␟␞␝\nm␟n␟␞o␟p␟␞␝\n␜\n";

//// With style symbols and layout files

/// Example USV unit with style symbols and layout files.
pub const EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟";

/// Example USV units with style symbols and layout files.
pub const EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␟";

/// Example USV record with style symbols and layout files.
pub const EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␟␞";

/// Example USV records with style symbols and layout files.
pub const EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␟␞c␟d␟␞";

/// Example USV group with style symbols and layout files.
pub const EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␟␞c␟d␟␞␝";

/// Example USV groups with style symbols and layout files.
pub const EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";

/// Example USV file with style symbols and layout files.
pub const EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜\n";

/// Example USV files with style symbols and layout files.
pub const EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_FILES: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜\ni␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜\n";

//// With style controls and layout 0

/// Example USV unit with style controls and layout 0.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}";

/// Example USV units with style controls and layout 0.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001F}";

/// Example USV record with style controls and layout 0.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001F}\u{001E}";

/// Example USV records with style controls and layout 0.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}";

/// Example USV group with style controls and layout 0.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}";

/// Example USV groups with style controls and layout 0.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}";

/// Example USV file with style controls and layout 0.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}";

/// Example USV files with style controls and layout 0.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_0: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}i\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}";

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
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_1: &str =  "a\n\u{001F}\nb\n\u{001F}\n\n\u{001E}\nc\n\u{001F}\nd\n\u{001F}\n\n\u{001E}\n\n\u{001D}\n";

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
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\n";

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
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n\u{001E}\nc\u{001F}\nd\u{001F}\n\u{001E}\n\u{001D}\ne\u{001F}\nf\u{001F}\n\u{001E}\ng\u{001F}\nh\u{001F}\n\u{001E}\n\u{001D}\n";

/// Example USV files with style controls and layout records.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_UNITS: &str = "a\u{001F}\nb\u{001F}\n\u{001E}\nc\u{001F}\nd\u{001F}\n\u{001E}\n\u{001D}\ne\u{001F}\nf\u{001F}\n\u{001E}\ng\u{001F}\nh\u{001F}\n\u{001E}\n\u{001D}\ni\u{001F}\nj\u{001F}\n\u{001E}\nk\u{001F}\nl\u{001F}\n\u{001E}\n\u{001D}\nm\u{001F}\nn\u{001F}\n\u{001E}\no\u{001F}\np\u{001F}\n\u{001E}\n\u{001D}\n";

//// With style controls and layout records

/// Example USV unit with style controls and layout records.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}";

/// Example USV units with style controls and layout records.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001F}";

/// Example USV record with style controls and layout records.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001F}\u{001E}\n";

/// Example USV records with style controls and layout records.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001F}\u{001E}\nc\u{001F}d\u{001F}\u{001E}\n";

/// Example USV group with style controls and layout records.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001F}\u{001E}\nc\u{001F}d\u{001F}\u{001E}\n\u{001D}\n";

/// Example USV groups with style controls and layout records.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001F}\u{001E}\nc\u{001F}d\u{001F}\u{001E}\n\u{001D}\ne\u{001F}f\u{001F}\u{001E}\ng\u{001F}h\u{001F}\u{001E}\n\u{001D}\n";

/// Example USV file with style controls and layout records.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001F}\u{001E}\nc\u{001F}d\u{001F}\u{001E}\n\u{001D}\ne\u{001F}f\u{001F}\u{001E}\ng\u{001F}h\u{001F}\u{001E}\n\u{001D}\n\u{001C}\n";

/// Example USV files with style controls and layout records.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_RECORDS: &str = "a\u{001F}b\u{001F}\u{001E}\nc\u{001F}d\u{001F}\u{001E}\n\u{001D}\ne\u{001F}f\u{001F}\u{001E}\ng\u{001F}h\u{001F}\u{001E}\n\u{001D}\n\u{001C}\ni\u{001F}j\u{001F}\u{001E}\nk\u{001F}l\u{001F}\u{001E}\n\u{001D}\nm\u{001F}n\u{001F}\u{001E}\no\u{001F}p\u{001F}\u{001E}\n\u{001D}\n\u{001C}\n";

//// With style controls and layout groups

/// Example USV unit with style controls and layout records.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}";

/// Example USV units with style controls and layout records.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001F}";

/// Example USV record with style controls and layout records.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001F}\u{001E}";

/// Example USV records with style controls and layout records.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}";

/// Example USV group with style controls and layout records.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}\n";

/// Example USV groups with style controls and layout records.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}\ne\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\n";

/// Example USV file with style controls and layout records.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}\ne\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\n\u{001C}\n";

/// Example USV files with style controls and layout records.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_GROUPS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}\ne\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\n\u{001C}\ni\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}\nm\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\n\u{001C}\n";

//// With style controls and layout files

/// Example USV unit with style controls and layout records.
pub const EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}";

/// Example USV units with style controls and layout records.
pub const EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001F}";

/// Example USV record with style controls and layout records.
pub const EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001F}\u{001E}";

/// Example USV records with style controls and layout records.
pub const EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}";

/// Example USV group with style controls and layout records.
pub const EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}";

/// Example USV groups with style controls and layout records.
pub const EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}";

/// Example USV file with style controls and layout records.
pub const EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}\n";

/// Example USV files with style controls and layout records.
pub const EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_FILES: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}\ni\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}\n";

//// With style braces and layout 0

/// Example USV unit with style braces and layout 0.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_0: &str = "a{US}";

/// Example USV units with style braces and layout 0.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{US}";

/// Example USV record with style braces and layout 0.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{US}{RS}";

/// Example USV records with style braces and layout 0.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{US}{RS}c{US}d{US}{RS}";

/// Example USV group with style braces and layout 0.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}";

/// Example USV groups with style braces and layout 0.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}";

/// Example USV file with style braces and layout 0.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}";

/// Example USV files with style braces and layout 0.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_0: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}i{US}j{US}{RS}k{US}l{US}{RS}{GS}m{US}n{US}{RS}o{US}p{US}{RS}{GS}{FS}";

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
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_UNITS: &str = "a{US}\n";

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
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}";

/// Example USV units with style braces and layout records.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{US}";

/// Example USV record with style braces and layout records.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{US}{RS}\n";

/// Example USV records with style braces and layout records.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{US}{RS}\nc{US}d{US}{RS}\n";

/// Example USV group with style braces and layout records.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{US}{RS}\nc{US}d{US}{RS}\n{GS}\n";

/// Example USV groups with style braces and layout records.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{US}{RS}\nc{US}d{US}{RS}\n{GS}\ne{US}f{US}{RS}\ng{US}h{US}{RS}\n{GS}\n";

/// Example USV file with style braces and layout records.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{US}{RS}\nc{US}d{US}{RS}\n{GS}\ne{US}f{US}{RS}\ng{US}h{US}{RS}\n{GS}\n{FS}\n";

/// Example USV files with style braces and layout records.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_RECORDS: &str = "a{US}b{US}{RS}\nc{US}d{US}{RS}\n{GS}\ne{US}f{US}{RS}\ng{US}h{US}{RS}\n{GS}\n{FS}\ni{US}j{US}{RS}\nk{US}l{US}{RS}\n{GS}\nm{US}n{US}{RS}\no{US}p{US}{RS}\n{GS}\n{FS}\n";

//// With style braces and layout groups

/// Example USV unit with style braces and layout groups.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}";

/// Example USV units with style braces and layout groups.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{US}";

/// Example USV record with style braces and layout groups.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{US}{RS}";

/// Example USV records with style braces and layout groups.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{US}{RS}c{US}d{US}{RS}";

/// Example USV group with style braces and layout groups.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}\n";

/// Example USV groups with style braces and layout groups.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}\ne{US}f{US}{RS}g{US}h{US}{RS}{GS}\n";

/// Example USV file with style braces and layout groups.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}\ne{US}f{US}{RS}g{US}h{US}{RS}{GS}\n{FS}\n";

/// Example USV files with style braces and layout groups.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_GROUPS: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}\ne{US}f{US}{RS}g{US}h{US}{RS}{GS}\n{FS}\ni{US}j{US}{RS}k{US}l{US}{RS}{GS}\nm{US}n{US}{RS}o{US}p{US}{RS}{GS}\n{FS}\n";

//// With style braces and layout files

/// Example USV unit with style braces and layout files.
pub const EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}";

/// Example USV units with style braces and layout files.
pub const EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{US}";

/// Example USV record with style braces and layout files.
pub const EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{US}{RS}";

/// Example USV records with style braces and layout files.
pub const EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}";

/// Example USV group with style braces and layout files.
pub const EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}";

/// Example USV groups with style braces and layout files.
pub const EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}";

/// Example USV file with style braces and layout files.
pub const EXAMPLE_FILE_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}\n";

/// Example USV files with style braces and layout files.
pub const EXAMPLE_FILES_STYLE_BRACES_LAYOUT_FILES: &str = "a{US}b{US}{RS}c{US}d{US}{RS}{GS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}\ni{US}j{US}{RS}k{US}l{US}{RS}{GS}m{US}n{US}{RS}o{US}p{US}{RS}{GS}{FS}\n";

//// Arrays

/// Example array with USV units
pub const EXAMPLE_ARRAY_UNITS: [&str;2] = ["a","b"];

/// Example array with USV units
pub const EXAMPLE_ARRAY_RECORDS: [[&str;2];2] = [["a","b"],["c","d"]];

/// Example array with USV files
pub const EXAMPLE_ARRAY_GROUPS: [[[&str;2];2];2] = [[["a","b"],["c","d"]],[["e","f"],["g","h"]]];

/// Example array with USV files
pub const EXAMPLE_ARRAY_FILES: [[[[&str;2];2];2];2] = [[[["a","b"],["c","d"]],[["e","f"],["g","h"]]],[[["i","j"],["k","l"]],[["m","n"],["o","p"]]]];


#[cfg(test)]
mod tests {
    use super::*;

    fn assert_content(a: &str, b: &str) {
        let s = a
        .replace("\n", "")
        .replace("␟", "")
        .replace("␞", "")
        .replace("␝", "")
        .replace("␜", "")
        .replace("\u{001F}", "")
        .replace("\u{001E}", "")
        .replace("\u{001D}", "")
        .replace("\u{001C}", "")
        .replace("{US}", "")
        .replace("{RS}", "")
        .replace("{GS}", "")
        .replace("{FS}", "");
        assert_eq!(s, b);
    }

    fn assert_starts(a: &str, b: &str) {
        assert!(a.starts_with(b), "assert_starts a.starts_with(b)\n left: {}\nright: {}\n", a, b);
    }

    fn assert_comprises(a: &str, b: &str) {
        assert_eq!(a.len(), b.len() * 2, "assert_comprises a.len() == b.len() * 2\na.len(): {}\nb.len(): {}\na: {}\nb: {}", a.len(), b.len(), a, b);
        assert!(a.starts_with(b), "assert_comprises a.starts_with(b)\n left: {}\nright: {}\n", a, b);
    }

    #[test]
    fn example_unit() {
        assert_content(EXAMPLE_UNIT, "a");
    }

    #[test]
    fn example_unit_style_symbols() {
        assert_content(EXAMPLE_UNIT_STYLE_SYMBOLS, "a");
    }
    
    #[test]
    fn example_unit_style_symbols_layout_0() {
        assert_content(EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_0, "a");
    }
    
    #[test]
    fn example_unit_style_symbols_layout_1() {
        assert_content(EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_1, "a");
    }
    
    #[test]
    fn example_unit_style_symbols_layout_2() {
        assert_content(EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_2, "a");
    }
    
    #[test]
    fn example_unit_style_symbols_layout_unit() {
        assert_content(EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_UNITS, "a");
    }
    
    #[test]
    fn example_unit_style_symbols_layout_records() {
        assert_content(EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_RECORDS, "a");
    }
    
    #[test]
    fn example_unit_style_symbols_layout_groups() {
        assert_content(EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_GROUPS, "a");
    }
    
    #[test]
    fn example_unit_style_symbols_layout_files() {
        assert_content(EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_FILES, "a");
    }
    
    #[test]
    fn example_unit_style_controls() {
        assert_content(EXAMPLE_UNIT_STYLE_CONTROLS, "a");
    }
    
    #[test]
    fn example_unit_style_controls_layout_0() {
        assert_content(EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_0, "a");
    }
    
    #[test]
    fn example_unit_style_controls_layout_1() {
        assert_content(EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_1, "a");
    }
    
    #[test]
    fn example_unit_style_controls_layout_2() {
        assert_content(EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_2, "a");
    }
    
    #[test]
    fn example_unit_style_controls_layout_unit() {
        assert_content(EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_UNITS, "a");
    }
    
    #[test]
    fn example_unit_style_controls_layout_records() {
        assert_content(EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_RECORDS, "a");
    }
    
    #[test]
    fn example_unit_style_controls_layout_groups() {
        assert_content(EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_GROUPS, "a");
    }
    
    #[test]
    fn example_unit_style_controls_layout_files() {
        assert_content(EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_FILES, "a");
    }
    
    #[test]
    fn example_unit_style_braces() {
        assert_content(EXAMPLE_UNIT_STYLE_BRACES, "a");
    }
    
    #[test]
    fn example_unit_style_braces_layout_0() {
        assert_content(EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_0, "a");
    }
    
    #[test]
    fn example_unit_style_braces_layout_1() {
        assert_content(EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_1, "a");
    }
    
    #[test]
    fn example_unit_style_braces_layout_2() {
        assert_content(EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_2, "a");
    }
    
    #[test]
    fn example_unit_style_braces_layout_unit() {
        assert_content(EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_UNITS, "a");
    }
    
    #[test]
    fn example_unit_style_braces_layout_records() {
        assert_content(EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_RECORDS, "a");
    }
    
    #[test]
    fn example_unit_style_braces_layout_groups() {
        assert_content(EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_GROUPS, "a");
    }
    
    #[test]
    fn example_unit_style_braces_layout_files() {
        assert_content(EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_FILES, "a");
    }

    #[test]
    fn example_units() {
        assert_content(EXAMPLE_UNITS, "ab");
        assert_comprises(EXAMPLE_UNITS, EXAMPLE_UNIT);
    }

    #[test]
    fn example_units_style_symbols() {
        assert_content(EXAMPLE_UNITS_STYLE_SYMBOLS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_SYMBOLS, EXAMPLE_UNIT_STYLE_SYMBOLS);
    }
    
    #[test]
    fn example_units_style_symbols_layout_0() {
        assert_content(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_0, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_0, EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_0);
    }
    
    #[test]
    fn example_units_style_symbols_layout_1() {
        assert_content(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_1, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_1, EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_1);
    }
    
    #[test]
    fn example_units_style_symbols_layout_2() {
        assert_content(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_2, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_2, EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_2);
    }
    
    #[test]
    fn example_units_style_symbols_layout_units() {
        assert_content(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_UNITS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_UNITS, EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_UNITS);
    }
    
    #[test]
    fn example_units_style_symbols_layout_records() {
        assert_content(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_RECORDS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_RECORDS, EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_RECORDS);
    }
    
    #[test]
    fn example_units_style_symbols_layout_groups() {
        assert_content(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_GROUPS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_GROUPS, EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_GROUPS);
    }
    
    #[test]
    fn example_units_style_symbols_layout_files() {
        assert_content(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_FILES, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_FILES, EXAMPLE_UNIT_STYLE_SYMBOLS_LAYOUT_FILES);
    }
    
    #[test]
    fn example_units_style_controls() {
        assert_content(EXAMPLE_UNITS_STYLE_CONTROLS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_CONTROLS, EXAMPLE_UNIT_STYLE_CONTROLS);
    }
    
    #[test]
    fn example_units_style_controls_layout_0() {
        assert_content(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_0, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_0, EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_0);
    }
    
    #[test]
    fn example_units_style_controls_layout_1() {
        assert_content(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_1, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_1, EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_1);
    }
    
    #[test]
    fn example_units_style_controls_layout_2() {
        assert_content(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_2, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_2, EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_2);
    }
    
    #[test]
    fn example_units_style_controls_layout_units() {
        assert_content(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_UNITS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_UNITS, EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_UNITS);
    }
    
    #[test]
    fn example_units_style_controls_layout_records() {
        assert_content(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_RECORDS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_RECORDS, EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_RECORDS);
    }
    
    #[test]
    fn example_units_style_controls_layout_groups() {
        assert_content(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_GROUPS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_GROUPS, EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_GROUPS);
    }
    
    #[test]
    fn example_units_style_controls_layout_files() {
        assert_content(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_FILES, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_FILES, EXAMPLE_UNIT_STYLE_CONTROLS_LAYOUT_FILES);
    }
    
    #[test]
    fn example_units_style_braces() {
        assert_content(EXAMPLE_UNITS_STYLE_BRACES, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_BRACES, EXAMPLE_UNIT_STYLE_BRACES);
    }
    
    #[test]
    fn example_units_style_braces_layout_0() {
        assert_content(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_0, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_0, EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_0);
    }
    
    #[test]
    fn example_units_style_braces_layout_1() {
        assert_content(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_1, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_1, EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_1);
    }
    
    #[test]
    fn example_units_style_braces_layout_2() {
        assert_content(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_2, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_2, EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_2);
    }
    
    #[test]
    fn example_units_style_braces_layout_units() {
        assert_content(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_UNITS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_UNITS, EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_UNITS);
    }
    
    #[test]
    fn example_units_style_braces_layout_records() {
        assert_content(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_RECORDS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_RECORDS, EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_RECORDS);
    }
    
    #[test]
    fn example_units_style_braces_layout_groups() {
        assert_content(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_GROUPS, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_GROUPS, EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_GROUPS);
    }
    
    #[test]
    fn example_units_style_braces_layout_files() {
        assert_content(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_FILES, "ab"); 
        assert_comprises(EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_FILES, EXAMPLE_UNIT_STYLE_BRACES_LAYOUT_FILES);
    }
    
    #[test]
    fn example_record() {
        assert_content(EXAMPLE_RECORD, "ab"); 
        assert_starts(EXAMPLE_RECORD, EXAMPLE_UNITS);
    }

    #[test]
    fn example_record_style_symbols() {
        assert_content(EXAMPLE_RECORD_STYLE_SYMBOLS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_SYMBOLS, EXAMPLE_UNITS_STYLE_SYMBOLS);
    }

    #[test]
    fn example_record_style_symbols_layout_0() {
        assert_content(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_0, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_0, EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_0);
    }

    #[test]
    fn example_record_style_symbols_layout_1() {
        assert_content(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_1, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_1, EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_1);
    }

    #[test]
    fn example_record_style_symbols_layout_2() {
        assert_content(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_2, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_2, EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_2);
    }

    #[test]
    fn example_record_style_symbols_layout_units() {
        assert_content(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_UNITS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_UNITS, EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_record_style_symbols_layout_records() {
        assert_content(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_RECORDS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_RECORDS, EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_record_style_symbols_layout_groups() {
        assert_content(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_GROUPS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_GROUPS, EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_record_style_symbols_layout_files() {
        assert_content(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_FILES, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_FILES, EXAMPLE_UNITS_STYLE_SYMBOLS_LAYOUT_FILES);
    }

    #[test]
    fn example_record_style_controls() {
        assert_content(EXAMPLE_RECORD_STYLE_CONTROLS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_CONTROLS, EXAMPLE_UNITS_STYLE_CONTROLS);
    }

    #[test]
    fn example_record_style_controls_layout_0() {
        assert_content(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_0, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_0, EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_0);
    }

    #[test]
    fn example_record_style_controls_layout_1() {
        assert_content(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_1, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_1, EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_1);
    }

    #[test]
    fn example_record_style_controls_layout_2() {
        assert_content(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_2, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_2, EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_2);
    }

    #[test]
    fn example_record_style_controls_layout_units() {
        assert_content(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_UNITS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_UNITS, EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_record_style_controls_layout_records() {
        assert_content(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_RECORDS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_RECORDS, EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_record_style_controls_layout_groups() {
        assert_content(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_GROUPS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_GROUPS, EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_record_style_controls_layout_files() {
        assert_content(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_FILES, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_FILES, EXAMPLE_UNITS_STYLE_CONTROLS_LAYOUT_FILES);
    }

    #[test]
    fn example_record_style_braces() {
        assert_content(EXAMPLE_RECORD_STYLE_BRACES, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_BRACES, EXAMPLE_UNITS_STYLE_BRACES);
    }

    #[test]
    fn example_record_style_braces_layout_0() {
        assert_content(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_0, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_0, EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_0);
    }

    #[test]
    fn example_record_style_braces_layout_1() {
        assert_content(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_1, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_1, EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_1);
    }

    #[test]
    fn example_record_style_braces_layout_2() {
        assert_content(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_2, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_2, EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_2);
    }

    #[test]
    fn example_record_style_braces_layout_units() {
        assert_content(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_UNITS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_UNITS, EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_UNITS);
    }

    #[test]
    fn example_record_style_braces_layout_records() {
        assert_content(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_RECORDS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_RECORDS, EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_RECORDS);
    }

    #[test]
    fn example_record_style_braces_layout_groups() {
        assert_content(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_GROUPS, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_GROUPS, EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_GROUPS);
    }

    #[test]
    fn example_record_style_braces_layout_files() {
        assert_content(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_FILES, "ab"); 
        assert_starts(EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_FILES, EXAMPLE_UNITS_STYLE_BRACES_LAYOUT_FILES);
    }

    #[test]
    fn example_records() {
        assert_content(EXAMPLE_RECORDS, "abcd"); 
        assert_starts(EXAMPLE_RECORDS, EXAMPLE_RECORD);
    }

    #[test]
    fn example_records_style_symbols() {
        assert_content(EXAMPLE_RECORDS_STYLE_SYMBOLS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_SYMBOLS, EXAMPLE_RECORD_STYLE_SYMBOLS);
    }

    #[test]
    fn example_records_style_symbols_layout_0() {
        assert_content(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_0, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_0, EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_0);
    }

    #[test]
    fn example_records_style_symbols_layout_1() {
        assert_content(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_1, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_1, EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_1);
    }

    #[test]
    fn example_records_style_symbols_layout_2() {
        assert_content(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_2, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_2, EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_2);
    }

    #[test]
    fn example_records_style_symbols_layout_units() {
        assert_content(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_UNITS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_UNITS, EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_records_style_symbols_layout_records() {
        assert_content(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_RECORDS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_RECORDS, EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_records_style_symbols_layout_groups() {
        assert_content(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_GROUPS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_GROUPS, EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_records_style_symbols_layout_files() {
        assert_content(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_FILES, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_FILES, EXAMPLE_RECORD_STYLE_SYMBOLS_LAYOUT_FILES);
    }

    #[test]
    fn example_records_style_controls() {
        assert_content(EXAMPLE_RECORDS_STYLE_CONTROLS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_CONTROLS, EXAMPLE_RECORD_STYLE_CONTROLS);
    }

    #[test]
    fn example_records_style_controls_layout_0() {
        assert_content(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_0, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_0, EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_0);
    }

    #[test]
    fn example_records_style_controls_layout_1() {
        assert_content(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_1, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_1, EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_1);
    }

    #[test]
    fn example_records_style_controls_layout_2() {
        assert_content(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_2, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_2, EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_2);
    }

    #[test]
    fn example_records_style_controls_layout_units() {
        assert_content(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_UNITS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_UNITS, EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_records_style_controls_layout_records() {
        assert_content(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_RECORDS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_RECORDS, EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_records_style_controls_layout_groups() {
        assert_content(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_GROUPS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_GROUPS, EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_records_style_controls_layout_files() {
        assert_content(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_FILES, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_FILES, EXAMPLE_RECORD_STYLE_CONTROLS_LAYOUT_FILES);
    }

    #[test]
    fn example_records_style_braces() {
        assert_content(EXAMPLE_RECORDS_STYLE_BRACES, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_BRACES, EXAMPLE_RECORD_STYLE_BRACES);
    }

    #[test]
    fn example_records_style_braces_layout_0() {
        assert_content(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_0, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_0, EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_0);
    }

    #[test]
    fn example_records_style_braces_layout_1() {
        assert_content(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_1, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_1, EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_1);
    }

    #[test]
    fn example_records_style_braces_layout_2() {
        assert_content(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_2, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_2, EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_2);
    }

    #[test]
    fn example_records_style_braces_layout_units() {
        assert_content(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_UNITS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_UNITS, EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_UNITS);
    }

    #[test]
    fn example_records_style_braces_layout_records() {
        assert_content(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_RECORDS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_RECORDS, EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_RECORDS);
    }

    #[test]
    fn example_records_style_braces_layout_groups() {
        assert_content(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_GROUPS, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_GROUPS, EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_GROUPS);
    }

    #[test]
    fn example_records_style_braces_layout_files() {
        assert_content(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_FILES, "abcd"); 
        assert_comprises(EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_FILES, EXAMPLE_RECORD_STYLE_BRACES_LAYOUT_FILES);
    }

    #[test]
    fn example_group() {
        assert_content(EXAMPLE_GROUP, "abcd"); 
        assert_starts(EXAMPLE_GROUP, EXAMPLE_RECORDS);
    }

    #[test]
    fn example_group_style_symbols() {
        assert_content(EXAMPLE_GROUP_STYLE_SYMBOLS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_SYMBOLS, EXAMPLE_RECORDS_STYLE_SYMBOLS);
    }

    #[test]
    fn example_group_style_symbols_layout_0() {
        assert_content(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_0, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_0, EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_0);
    }

    #[test]
    fn example_group_style_symbols_layout_1() {
        assert_content(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_1, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_1, EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_1);
    }

    #[test]
    fn example_group_style_symbols_layout_2() {
        assert_content(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_2, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_2, EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_2);
    }

    #[test]
    fn example_group_style_symbols_layout_units() {
        assert_content(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_UNITS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_UNITS, EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_group_style_symbols_layout_records() {
        assert_content(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_RECORDS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_RECORDS, EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_group_style_symbols_layout_groups() {
        assert_content(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_GROUPS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_GROUPS, EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_group_style_symbols_layout_files() {
        assert_content(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_FILES, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_FILES, EXAMPLE_RECORDS_STYLE_SYMBOLS_LAYOUT_FILES);
    }

    #[test]
    fn example_group_style_controls() {
        assert_content(EXAMPLE_GROUP_STYLE_CONTROLS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_CONTROLS, EXAMPLE_RECORDS_STYLE_CONTROLS);
    }

    #[test]
    fn example_group_style_controls_layout_0() {
        assert_content(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_0, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_0, EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_0);
    }

    #[test]
    fn example_group_style_controls_layout_1() {
        assert_content(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_1, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_1, EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_1);
    }

    #[test]
    fn example_group_style_controls_layout_2() {
        assert_content(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_2, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_2, EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_2);
    }

    #[test]
    fn example_group_style_controls_layout_units() {
        assert_content(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_UNITS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_UNITS, EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_group_style_controls_layout_records() {
        assert_content(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_RECORDS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_RECORDS, EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_group_style_controls_layout_groups() {
        assert_content(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_GROUPS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_GROUPS, EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_group_style_controls_layout_files() {
        assert_content(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_FILES, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_FILES, EXAMPLE_RECORDS_STYLE_CONTROLS_LAYOUT_FILES);
    }

    #[test]
    fn example_group_style_braces() {
        assert_content(EXAMPLE_GROUP_STYLE_BRACES, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_BRACES, EXAMPLE_RECORDS_STYLE_BRACES);
    }

    #[test]
    fn example_group_style_braces_layout_0() {
        assert_content(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_0, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_0, EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_0);
    }

    #[test]
    fn example_group_style_braces_layout_1() {
        assert_content(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_1, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_1, EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_1);
    }

    #[test]
    fn example_group_style_braces_layout_2() {
        assert_content(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_2, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_2, EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_2);
    }

    #[test]
    fn example_group_style_braces_layout_units() {
        assert_content(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_UNITS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_UNITS, EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_UNITS);
    }

    #[test]
    fn example_group_style_braces_layout_records() {
        assert_content(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_RECORDS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_RECORDS, EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_RECORDS);
    }

    #[test]
    fn example_group_style_braces_layout_groups() {
        assert_content(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_GROUPS, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_GROUPS, EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_GROUPS);
    }

    #[test]
    fn example_group_style_braces_layout_files() {
        assert_content(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_FILES, "abcd"); 
        assert_starts(EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_FILES, EXAMPLE_RECORDS_STYLE_BRACES_LAYOUT_FILES);
    }

    #[test]
    fn example_groups() {
        assert_content(EXAMPLE_GROUPS, "abcdefgh"); 
        assert_starts(EXAMPLE_GROUPS, EXAMPLE_GROUP);
    }

    #[test]
    fn example_groups_style_symbols() {
        assert_content(EXAMPLE_GROUPS_STYLE_SYMBOLS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_SYMBOLS, EXAMPLE_GROUP_STYLE_SYMBOLS);
    }

    #[test]
    fn example_groups_style_symbols_layout_0() {
        assert_content(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_0, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_0, EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_0);
    }

    #[test]
    fn example_groups_style_symbols_layout_1() {
        assert_content(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_1, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_1, EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_1);
    }

    #[test]
    fn example_groups_style_symbols_layout_2() {
        assert_content(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_2, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_2, EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_2);
    }

    #[test]
    fn example_groups_style_symbols_layout_units() {
        assert_content(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_UNITS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_UNITS, EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_groups_style_symbols_layout_records() {
        assert_content(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_RECORDS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_RECORDS, EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_groups_style_symbols_layout_groups() {
        assert_content(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_GROUPS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_GROUPS, EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_groups_style_symbols_layout_files() {
        assert_content(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_FILES, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_FILES, EXAMPLE_GROUP_STYLE_SYMBOLS_LAYOUT_FILES);
    }

    #[test]
    fn example_groups_style_controls() {
        assert_content(EXAMPLE_GROUPS_STYLE_CONTROLS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_CONTROLS, EXAMPLE_GROUP_STYLE_CONTROLS);
    }

    #[test]
    fn example_groups_style_controls_layout_0() {
        assert_content(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_0, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_0, EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_0);
    }

    #[test]
    fn example_groups_style_controls_layout_1() {
        assert_content(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_1, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_1, EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_1);
    }

    #[test]
    fn example_groups_style_controls_layout_2() {
        assert_content(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_2, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_2, EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_2);
    }

    #[test]
    fn example_groups_style_controls_layout_units() {
        assert_content(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_UNITS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_UNITS, EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_groups_style_controls_layout_records() {
        assert_content(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_RECORDS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_RECORDS, EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_groups_style_controls_layout_groups() {
        assert_content(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_GROUPS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_GROUPS, EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_groups_style_controls_layout_files() {
        assert_content(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_FILES, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_FILES, EXAMPLE_GROUP_STYLE_CONTROLS_LAYOUT_FILES);
    }

    #[test]
    fn example_groups_style_braces() {
        assert_content(EXAMPLE_GROUPS_STYLE_BRACES, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_BRACES, EXAMPLE_GROUP_STYLE_BRACES);
    }

    #[test]
    fn example_groups_style_braces_layout_0() {
        assert_content(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_0, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_0, EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_0);
    }

    #[test]
    fn example_groups_style_braces_layout_1() {
        assert_content(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_1, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_1, EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_1);
    }

    #[test]
    fn example_groups_style_braces_layout_2() {
        assert_content(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_2, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_2, EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_2);
    }

    #[test]
    fn example_groups_style_braces_layout_units() {
        assert_content(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_UNITS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_UNITS, EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_UNITS);
    }

    #[test]
    fn example_groups_style_braces_layout_records() {
        assert_content(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_RECORDS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_RECORDS, EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_RECORDS);
    }

    #[test]
    fn example_groups_style_braces_layout_groups() {
        assert_content(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_GROUPS, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_GROUPS, EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_GROUPS);
    }

    #[test]
    fn example_groups_style_braces_layout_files() {
        assert_content(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_FILES, "abcdefgh"); 
        assert_comprises(EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_FILES, EXAMPLE_GROUP_STYLE_BRACES_LAYOUT_FILES);
    }

    #[test]
    fn example_file() {
        assert_content(EXAMPLE_FILE, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE, EXAMPLE_GROUPS);
    }

    #[test]
    fn example_file_style_symbols() {
        assert_content(EXAMPLE_FILE_STYLE_SYMBOLS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_SYMBOLS, EXAMPLE_GROUPS_STYLE_SYMBOLS);
    }

    #[test]
    fn example_file_style_symbols_layout_0() {
        assert_content(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_0, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_0, EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_0);
    }

    #[test]
    fn example_file_style_symbols_layout_1() {
        assert_content(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_1, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_1, EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_1);
    }

    #[test]
    fn example_file_style_symbols_layout_2() {
        assert_content(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_2, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_2, EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_2);
    }

    #[test]
    fn example_file_style_symbols_layout_units() {
        assert_content(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_UNITS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_UNITS, EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_file_style_symbols_layout_records() {
        assert_content(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_RECORDS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_RECORDS, EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_file_style_symbols_layout_groups() {
        assert_content(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_GROUPS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_GROUPS, EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_file_style_symbols_layout_files() {
        assert_content(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_FILES, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_FILES, EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_FILES);
    }

    #[test]
    fn example_file_style_controls() {
        assert_content(EXAMPLE_FILE_STYLE_CONTROLS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_CONTROLS, EXAMPLE_GROUPS_STYLE_CONTROLS);
    }

    #[test]
    fn example_file_style_controls_layout_0() {
        assert_content(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_0, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_0, EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_0);
    }

    #[test]
    fn example_file_style_controls_layout_1() {
        assert_content(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_1, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_1, EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_1);
    }

    #[test]
    fn example_file_style_controls_layout_2() {
        assert_content(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_2, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_2, EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_2);
    }

    #[test]
    fn example_file_style_controls_layout_units() {
        assert_content(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_UNITS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_UNITS, EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_file_style_controls_layout_records() {
        assert_content(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_RECORDS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_RECORDS, EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_file_style_controls_layout_groups() {
        assert_content(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_GROUPS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_GROUPS, EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_file_style_controls_layout_files() {
        assert_content(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_FILES, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_FILES, EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_FILES);
    }

    #[test]
    fn example_file_style_braces() {
        assert_content(EXAMPLE_FILE_STYLE_BRACES, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_BRACES, EXAMPLE_GROUPS_STYLE_BRACES);
    }

    #[test]
    fn example_file_style_braces_layout_0() {
        assert_content(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_0, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_0, EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_0);
    }

    #[test]
    fn example_file_style_braces_layout_1() {
        assert_content(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_1, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_1, EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_1);
    }

    #[test]
    fn example_file_style_braces_layout_2() {
        assert_content(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_2, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_2, EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_2);
    }

    #[test]
    fn example_file_style_braces_layout_units() {
        assert_content(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_UNITS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_UNITS, EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_UNITS);
    }

    #[test]
    fn example_file_style_braces_layout_records() {
        assert_content(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_RECORDS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_RECORDS, EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_RECORDS);
    }

    #[test]
    fn example_file_style_braces_layout_groups() {
        assert_content(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_GROUPS, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_GROUPS, EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_GROUPS);
    }

    #[test]
    fn example_file_style_braces_layout_files() {
        assert_content(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_FILES, "abcdefgh"); 
        assert_starts(EXAMPLE_FILE_STYLE_BRACES_LAYOUT_FILES, EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_FILES);
    }

    #[test]
    fn example_files() {
        assert_content(EXAMPLE_FILES, "abcdefghijklmnop"); 
        assert_starts(EXAMPLE_FILES, EXAMPLE_FILE);
    }

    #[test]
    fn example_files_style_symbols() {
        assert_content(EXAMPLE_FILES_STYLE_SYMBOLS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_SYMBOLS, EXAMPLE_FILE_STYLE_SYMBOLS);
    }

    #[test]
    fn example_files_style_symbols_layout_0() {
        assert_content(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_0, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_0, EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_0);
    }

    #[test]
    fn example_files_style_symbols_layout_1() {
        assert_content(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_1, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_1, EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_1);
    }

    #[test]
    fn example_files_style_symbols_layout_2() {
        assert_content(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_2, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_2, EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_2);
    }

    #[test]
    fn example_files_style_symbols_layout_units() {
        assert_content(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_UNITS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_UNITS, EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_files_style_symbols_layout_records() {
        assert_content(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_RECORDS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_RECORDS, EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_files_style_symbols_layout_groups() {
        assert_content(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_GROUPS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_GROUPS, EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_files_style_symbols_layout_files() {
        assert_content(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_FILES, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_FILES, EXAMPLE_FILE_STYLE_SYMBOLS_LAYOUT_FILES);
    }

    #[test]
    fn example_files_style_controls() {
        assert_content(EXAMPLE_FILES_STYLE_CONTROLS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_CONTROLS, EXAMPLE_FILE_STYLE_CONTROLS);
    }

    #[test]
    fn example_files_style_controls_layout_0() {
        assert_content(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_0, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_0, EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_0);
    }

    #[test]
    fn example_files_style_controls_layout_1() {
        assert_content(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_1, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_1, EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_1);
    }

    #[test]
    fn example_files_style_controls_layout_2() {
        assert_content(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_2, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_2, EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_2);
    }

    #[test]
    fn example_files_style_controls_layout_units() {
        assert_content(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_UNITS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_UNITS, EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_UNITS);
    }

    #[test]
    fn example_files_style_controls_layout_records() {
        assert_content(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_RECORDS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_RECORDS, EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_RECORDS);
    }

    #[test]
    fn example_files_style_controls_layout_groups() {
        assert_content(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_GROUPS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_GROUPS, EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_GROUPS);
    }

    #[test]
    fn example_files_style_controls_layout_files() {
        assert_content(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_FILES, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_FILES, EXAMPLE_FILE_STYLE_CONTROLS_LAYOUT_FILES);
    }

    #[test]
    fn example_files_style_braces() {
        assert_content(EXAMPLE_FILES_STYLE_BRACES, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_BRACES, EXAMPLE_FILE_STYLE_BRACES);
    }

    #[test]
    fn example_files_style_braces_layout_0() {
        assert_content(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_0, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_0, EXAMPLE_FILE_STYLE_BRACES_LAYOUT_0);
    }

    #[test]
    fn example_files_style_braces_layout_1() {
        assert_content(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_1, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_1, EXAMPLE_FILE_STYLE_BRACES_LAYOUT_1);
    }

    #[test]
    fn example_files_style_braces_layout_2() {
        assert_content(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_2, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_2, EXAMPLE_FILE_STYLE_BRACES_LAYOUT_2);
    }

    #[test]
    fn example_files_style_braces_layout_units() {
        assert_content(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_UNITS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_UNITS, EXAMPLE_FILE_STYLE_BRACES_LAYOUT_UNITS);
    }

    #[test]
    fn example_files_style_braces_layout_records() {
        assert_content(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_RECORDS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_RECORDS, EXAMPLE_FILE_STYLE_BRACES_LAYOUT_RECORDS);
    }

    #[test]
    fn example_files_style_braces_layout_groups() {
        assert_content(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_GROUPS, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_GROUPS, EXAMPLE_FILE_STYLE_BRACES_LAYOUT_GROUPS);
    }

    #[test]
    fn example_files_style_braces_layout_files() {
        assert_content(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_FILES, "abcdefghijklmnop"); 
        assert_comprises(EXAMPLE_FILES_STYLE_BRACES_LAYOUT_FILES, EXAMPLE_FILE_STYLE_BRACES_LAYOUT_FILES);
    }

    #[test]
    fn example_array_units() {
        assert_eq!(EXAMPLE_ARRAY_UNITS.len(), 2);
    }

    #[test]
    fn example_array_records() {
        assert_eq!(EXAMPLE_ARRAY_RECORDS.len(), 2);
    }

    #[test]
    fn example_array_groups() {
        assert_eq!(EXAMPLE_ARRAY_GROUPS.len(), 2);
    }

    #[test]
    fn example_array_files() {
        assert_eq!(EXAMPLE_ARRAY_FILES.len(), 2);
    }

}
