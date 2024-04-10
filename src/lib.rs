//! # Unicode Separated Values (USV) ™
//!
//! Unicode Separated Values (USV) ™ is a data format that uses Unicode characters for markup.
//!
//! This USV crate implements the USV specification: <https://github.com/sixarm/usv>.
//!
//! This USV crate and aims to help developers build new USV applications, tools, and workflows.
//!
//! ## USV characters
//!
//! Separators:
//!
//! * File Separator (FS) is U+001C or U+241C ␜
//!
//! * Group Separator (GS) is U+001D or U+241D ␝
//!
//! * Record Separator (RS) is U+001E or U+241E ␞
//!
//! * Unit Separator (US) is U+001F or U+241F ␟
//!
//! Modifiers:
//!
//! * Escape (ESC) is U+001B or U+241B ␛
//!
//! * End of Transmission (EOT) is U+0004 or U+2404 ␄
//!
//! ## Units
//!
//! ```rust
//! use usv::*;
//! let str = "a␟b␟";
//! let units: Units = str.units().collect();
//! assert_eq!(units, ["a", "b"]);
//! assert_eq!(units.into_usv_string(), str);
//!
//! ```
//!
//! ## Records
//!
//! ```rust
//! use usv::*;
//! let str = "a␟b␟␞c␟d␟␞";
//! let records: Records = str.records().collect();
//! assert_eq!(records, [["a", "b"],["c", "d"]]);
//! assert_eq!(records.into_usv_string(), str);
//! ```
//!
//! ## Groups
//!
//! ```rust
//! use usv::*;
//! let str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";
//! let groups: Groups = str.groups().collect();
//! assert_eq!(groups, [[["a", "b"],["c", "d"]],[["e", "f"],["g", "h"]]]);
//! assert_eq!(groups.into_usv_string(), str);
//! ```
//!
//! ## Files
//!
//! ```rust
//! use usv::*;
//! let str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
//! let files: Files = str.files().collect();
//! assert_eq!(files, [[[["a", "b"],["c", "d"]],[["e", "f"],["g", "h"]]],[[["i", "j"],["k", "l"]],[["m", "n"],["o", "p"]]]]);
//! assert_eq!(files.into_usv_string(), str);
//! ```
//!
//! ## Architecture
//!
//! The architecture of this crate looks like this, in order of importance:
//!
//! * `lib.rs`: the library entry point.
//!
//! * `constants.rs`: constants for USV characters.
//!
//! * `token.rs`: the USV Token enumerator for returning parser results.
//!
//! * `iter/`: iterators for units, records, groups, files, tokens.
//!
//! * `style/`: style sets of characters for symbols, controls, braces.
//!
//! * `layout/`: layout formats for lines, visual displays, and editors.
//!
//! * `from/`: convert from one thing into another thing.
//!
//! * `into_usv_string`: trait and impl to convert from data into a usv string.
//!
//! * `examples.rs`: data strings suitable for demos and tests.
//!
//! * `str_ext.rs`: string extension traits for parsing USV.
//!
//! * `svec.rs`: a simple macro for creating string vectors.
//!
//! * `bench/`: benchmark tests; this is work in progress.
//!
//! * `tests/`: integration tests placeholder; not needed yet.
//!
//! ## Token
//!
//! A token is the underlying USV enumeration for parsing a string to output:
//!
//! ```no_run
//! pub enum Token {
//!     Unit(String),
//!     UnitSeparator,
//!     RecordSeparator,
//!     GroupSeparator,
//!     FileSeparator,
//!     EndOfTransmission,
//! }
//! ```
//!
//! ## Type aliases
//!
//! * Token = described above
//!
//! * Tokens = Vec<Token>
//!
//! * Unit = String
//!
//! * Units = Vec<Unit>
//!
//! * Record = Units
//!
//! * Records = Vec<Record>
//!
//! * Group = Records
//!
//! * Groups = Vec<Records>
//!
//! * File = Groups
//!
//! * Files = Vec<File>
//!
//! ## Legal protection for standardization
//!
//! The USV project aims to become a free open source IETF standard and IANA standard, much like the standards for CSV and TDF.
//!
//! Until the standardization happens, the terms "Unicode Separated Values" and "USV" are both trademarks of this project. This repository is copyright 2022-2024. The trademarks and copyrights are by Joel Parker Henderson, me, an individual, not a company.
//!
//! When IETF and IANA approve the submissions as a standard, then the trademarks and copyright will go to a free libre open source software advocacy foundation. We welcome advice about how to do this well.
//!
//! ## Conclusion
//!
//! USV is helping us with data projects. We hope USV may help you too.
//!
//! We welcome constructive feedback about USV, as well as git issues, pull requests, and standardization help.

// Constants for each USV character as a control character and symbol character.
pub mod constants; pub use constants::*;

// Token enum that holds content data or a special character value.
pub mod token; pub use token::Token;

// Examples for demos and tests.
pub mod examples; pub use examples::*;

// Type aliases for USV naming
#[allow(dead_code)] pub type Tokens = std::vec::Vec<Token>;
#[allow(dead_code)] pub type Unit = String;
#[allow(dead_code)] pub type Units = std::vec::Vec<Unit>;
#[allow(dead_code)] pub type Record = Units;
#[allow(dead_code)] pub type Records = std::vec::Vec<Record>;
#[allow(dead_code)] pub type Group = Records;
#[allow(dead_code)] pub type Groups = std::vec::Vec<Records>;
#[allow(dead_code)] pub type File = Groups;
#[allow(dead_code)] pub type Files = std::vec::Vec<File>;

// Iterator for token, unit, record, group, file.
pub mod iter;

// &str extensions such as iterators for units, records, groups, files.
pub mod str_ext; pub use str_ext::StrExt;

// Convert such as from vectors of strings to a USV string.
pub mod from; pub use from::*;

// String extensions such as from functions for conversions.
pub mod into_usv_string; pub use into_usv_string::*;

// Style provides rendering configuration for separators etc.
pub mod style; pub use style::Style;

// Layout provides style decorations for rendering liners.
pub mod layout; pub use layout::*;

// svec! macro that makes string vectors.
pub mod svec; pub use svec::*;
