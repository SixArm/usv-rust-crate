//! # Unicode Separated Values (USV)
//!
//! Unicode separated values (USV) is a data format that uses Unicode symbol characters between data parts.
//!
//! The USV repo is <https://github.com/sixarm/usv>.
//!
//! ## USV characters
//!
//! Separators:
//!
//! * Unit Separator (US) is U+001F or U+241F ␟
//!
//! * Record Separator (RS) is U+001E or U+241E ␞
//!
//! * Group Separator (GS) is U+001D or U+241D ␝
//!
//! * File Separator (FS) is U+001C or U+241C ␜
//!
//! Modifiers:
//!
//! * Escape (ESC) is U+001B or U+241B ␛
//!
//! * End of Transmission (EOT) is U+0004 or U+2404 ␄
//!
//! Liners:
//!
//! * Carriage Return (CR) is U+000D
//!
//! * Line Feed (LF) is U+000A
//!
//! ## Units
//!
//! ```rust
//! use usv::*;
//! let input = "a␟b";
//! let units: Units = input.units().collect();
//! assert_eq!(units, ["a", "b"]);
//! ```
//!
//! ## Records
//!
//! ```rust
//! use usv::*;
//! let input = "a␟b␞c␟d";
//! let records: Records = input.records().collect();
//! assert_eq!(records, [["a", "b"],["c", "d"]]);
//! ```
//!
//! ## Groups
//!
//! ```rust
//! use usv::*;
//! let input = "a␟b␞c␟d␝e␟f␞g␟h";
//! let groups: Groups = input.groups().collect();
//! assert_eq!(groups, [[["a", "b"],["c", "d"]],[["e", "f"],["g", "h"]]]);
//! ```
//!
//! ## Files
//!
//! ```rust
//! use usv::*;
//! let input = "a␟b␞c␟d␝e␟f␞g␟h␜i␟j␞k␟l␝m␟n␞o␟p";
//! let files: Files = input.files().collect();
//! assert_eq!(files, [[[["a", "b"],["c", "d"]],[["e", "f"],["g", "h"]]],[[["i", "j"],["k", "l"]],[["m", "n"],["o", "p"]]]]);
//! ```
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

// Constants for each USV character as a control character and symbol character.
pub mod constants; pub use constants::*;

// Token enum that holds content data or a special character value.
pub mod token; pub use token::Token;

// Examples for demos and tests.
pub mod examples; pub use examples::*;

// Type aliases for USV naming
#[allow(dead_code)] pub type Tokens = Vec<Token>;
#[allow(dead_code)] pub type Unit = String;
#[allow(dead_code)] pub type Units = Vec<Unit>;
#[allow(dead_code)] pub type Record = Units;
#[allow(dead_code)] pub type Records = Vec<Record>;
#[allow(dead_code)] pub type Group = Records;
#[allow(dead_code)] pub type Groups = Vec<Records>;
#[allow(dead_code)] pub type File = Groups;
#[allow(dead_code)] pub type Files = Vec<File>;

// Iterator for token, unit, record, group, file.
pub mod iter;

// Iterator extensions for units, records, groups, files.
pub mod str_ext; pub use str_ext::StrExt;

// Style provides rendering configuration for separators etc.
pub mod style; pub use style::Style;

// Layout provides style decorations for rendering liners.
pub mod layout; pub use layout::*;

// svec! macro that makes string vectors.
pub mod svec; pub use svec::*;
