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
//! * ␟ U+241F Symbol for Unit Separator (US).
//!
//! * ␞ U+241E Symbol for Record Separator (RS).
//!
//! * ␝ U+241D Symbol for Group Separator (GS).
//!
//! * ␜ U+241C Symbol for File Separator (FS).
//!
//! Modifiers:
//!
//! * ␛ U+241B Symbol for Escape (ESC).
//!
//! * ␗ U+2417 Symbol for End of Transmission Block (ETB).
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
//! ## All together
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

// Iterator for unit, record, group, file.
pub mod string_iterator; pub use string_iterator::StringIterator;

// Iterator extensions for units, records, groups, files.
pub mod str_ext; pub use str_ext::StrExt;
pub mod string_ext; pub use string_ext::StringExt;

#[macro_export]
macro_rules! svec[
    ($($x:expr),*) => (
        vec![$($x),*].into_iter()
                     .map(|s: &'static str| s.to_string())
                     .collect::<Vec<String>>()
    );
    ($($x:expr,)*) => (svec![$($x),*]);
];
