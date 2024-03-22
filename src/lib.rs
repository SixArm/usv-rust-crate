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
//! let input = "a␟b␟";
//! let units: Vec<String> = input.units().collect();
//! assert_eq!(
//!     units,
//!     vec![
//!         String::from("a"),
//!         String::from("b"),
//!     ]
//! );
//! ```
//!
//! ## Records
//!
//! ```rust
//! use usv::*;
//! let input = "a␟b␟␞c␟d␟␞";
//! let records: Vec<String> = input.records().collect();
//! assert_eq!(
//!     records,
//!     vec![
//!         String::from("a␟b␟"),
//!         String::from("c␟d␟"),
//!     ]
//! );
//! ```
//!
//! ## Groups
//!
//! ```rust
//! use usv::*;
//! let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";
//! let groups: Vec<String> = input.groups().collect();
//! assert_eq!(
//!     groups,
//!     vec![
//!         String::from("a␟b␟␞c␟d␟␞"),
//!         String::from("e␟f␟␞g␟h␟␞"),
//!     ]
//! );
//! ```
//!
//! ## Files
//!
//! ```rust
//! use usv::*;
//! let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
//! let files: Vec<String> = input.files().collect();
//! assert_eq!(
//!     files,
//!     vec![
//!         String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝"),
//!         String::from("i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝"),
//!     ]
//! );
//! ```
//!
//! ## Loops
//!
//! ```rust
//! use usv::*;
//! let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
//! let mut string = String::new();
//! for file in input.files() {
//!     for group in file.groups() {
//!         for record in group.records() {
//!             for unit in record.units() {
//!                 string += &unit;
//!             }
//!         }
//!     }
//! }
//! assert_eq!(
//!     string,
//!     String::from("abcdefghijklmnop"),
//! );
//! ```

// Constants for each USV character as a control character and symbol character.
pub mod constants; pub use constants::*;

// Examples for demos and tests.
pub mod examples; pub use examples::*;

// Iterator for unit, record, group, file.
pub mod string_iterator; pub use string_iterator::StringIterator;

// Iterator extensions for units, records, groups, files.
pub mod str_ext; pub use str_ext::StrExt;

// Style provides rendering configuration for separators etc.
pub mod style; pub use style::Style;

// Layout provides style decorations for rendering liners.
pub mod layout; pub use layout::*;

/// svec! makes a string vector from an array of &str.
///
/// Example:
///
/// ```
/// use usv::svec;
/// let items = svec!["a", "b", "c"];
/// assert_eq!(
///     items,
///     vec![
///         String::from("a"),
///         String::from("b"),
///         String::from("c"),
///     ]
/// );
/// ```
///
#[macro_export]
macro_rules! svec[
    ($($x:expr),*) => (
        vec![$($x),*].into_iter()
                     .map(|s: &'static str| s.to_string())
                     .collect::<Vec<String>>()
    );
    ($($x:expr,)*) => (svec![$($x),*]);
];
