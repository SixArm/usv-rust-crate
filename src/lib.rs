//! # Unicode Separated Values (USV)
//!
//! Unicode separated values (USV) is a data format that uses Unicode symbol characters between data parts. USV competes with comma separated values (CSV), tab separated values (TSV), ASCII separated values (ASV), and similar systems. USV offers more capabilities and standards-track syntax.
//!
//! The USV repo is <https://github.com/sixarm/usv>.
//!
//! The USV file name extension is "usv".
//!
//! The USV media type is "text/usv". We have applied for IANA registration.
//!
//! [Frequently asked questions](https://github.com/SixArm/usv/blob/main/doc/faq/)
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
//! * ␖ U+2416 Symbol For Synchronous Idle (SYN).
//!
//! ## Hello World
//!
//! This kind of data …
//!
//! ```txt
//! hello, world
//! ```
//!
//! … is represented in USV as two units:
//!
//! ```usv
//! hello␟world␟
//! ```
//!
//! Optional: if you prefer to see one unit per line, then end each line with a USV escape:
//!
//! ```usv
//! hello␟␛
//! world␟␛
//! ```
//!
//! Example source code:
//!
//! ```rust
//! use usv::*;
//! let input = "hello␟world␟";
//! let output: Units = input.units().collect();
//! assert_eq!(
//!     output,
//!     [
//!         Unit::from("hello"),
//!         Unit::from("world"),
//!     ]
//! );
//! ```
//!
//! ## Hello World Goodnight Moon
//!
//! This kind of data …
//!
//! ```txt
//! [ hello, world ], [ goodnight, moon ]
//! ```
//!
//! … is represented in USV as two records, each with two units:
//!
//! ```usv
//! hello␟world␟␞goodnight␟␟moon␞
//! ```
//!
//! Optional: if you prefer to see one record per line, then end each line with a USV escape:
//!
//! ```usv
//! hello␟world␟␞␛
//! goodnight␟moon␟␞␛
//! ```
//!
//! Example source code:
//!
//! ```rust
//! use usv::*;
//! let input = "hello␟world␟␞goodnight␟moon␟␞";
//! let output: Records = input.records().collect();
//! assert_eq!(
//!     output,
//!     [
//!         vec![
//!             Unit::from("hello"),
//!             Unit::from("world"),
//!         ],
//!         vec![
//!             Unit::from("goodnight"),
//!             Unit::from("moon"),
//!         ],
//!     ]
//! );
//! ```
//!
//! ## Documentation
//!
//! Documentation links:
//!
//! * [Frequently asked questions](https://github.com/sixarm/usv/blob/main/doc/faq/)
//!
//! * [BNF: Backus-Naur form for standardization](https://github.com/sixarm/usv/blob/main/doc/bnf/)
//!
//! * [TODO list](https://github.com/sixarm/usv/blob/main/doc/todo/)
//!
//! Context help:
//! 
//! * [Comparisons with CSV, TSV, TDF, ASV, DEL](https://github.com/sixarm/usv/blob/main/doc/comparisons/)
//!
//! * [How to type Unicode characters](https://github.com/sixarm/usv/blob/main/doc/how-to-type-unicode-characters/)
//!
//! * [History of ASCII separated values (ASV)](https://github.com/sixarm/usv/blob/main/history-of-ascii-separated-values/)
//!
//! Commands to convert between formats:
//!
//! * [usv-to-csv](https://crates.io/crates/usv-to-csv)
//!
//! * [csv-to-usv](https://crates.io/crates/csv-to-usv)
//!
//! Example files:
//!
//! * [hello-world.usv](https://github.com/sixarm/usv/blob/main/examples/hello-world.usv) versus [hello-world.csv](https://github.com/sixarm/usv/blob/main/examples/hello-world.csv)
//!
//! * [zen-koans.usv](https://github.com/sixarm/usv/blob/main/examples/zen-koans.usv) versus [zen-koans.csv](https://github.com/sixarm/usv/blob/main/examples/zen-koans.csv)
//!
//! * [blog-posts.usv](https://github.com/sixarm/usv/blob/main/examples/blog-posts.usv) versus [blog-posts.csv](https://github.com/sixarm/usv/blob/main/examples/blog-posts.csv)
//!
//! * [end-of-transmission-block.usv](https://github.com/sixarm/usv/blob/main/examples/end-of-transmission-block.usv)
//!
//!
//! ## Examples
//!
//! USV with 2 units by 2 records by 2 groups by 2 files:
//!
//! ```usv
//! a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜
//! ```
//!
//! Optional: if you prefer to see one record per line, then end each line with a USV escape:
//!
//! ```usv
//! a␟b␟␞␛
//! c␟d␟␞␛
//! ␝␛
//! e␟f␟␞␛
//! g␟h␟␞␛
//! ␝␛
//! ␜␛
//! i␟j␟␞␛
//! k␟l␟␞␛
//! ␝␛
//! m␟n␟␞␛
//! o␟p␟␞␛
//! ␝␛
//! ␜␛
//! ```
//!
//! Example source code:
//!
//! ```rust
//! use usv::*;
//! let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
//! let output: Files = input.files().collect();
//! assert_eq!(
//!     output,
//!     [
//!         vec![
//!             vec![
//!                 vec![
//!                     Unit::from("a"),
//!                     Unit::from("b"),
//!                 ],
//!                 vec![
//!                     Unit::from("c"),
//!                     Unit::from("d"),
//!                 ],
//!             ],
//!             vec![
//!                 vec![
//!                     Unit::from("e"),
//!                     Unit::from("f"),
//!                 ],
//!                 vec![
//!                     Unit::from("g"),
//!                     Unit::from("h"),
//!                 ],
//!              ],
//!         ],
//!         vec![
//!             vec![
//!                 vec![
//!                     Unit::from("i"),
//!                     Unit::from("j"),
//!                 ],
//!                 vec![
//!                     Unit::from("k"),
//!                     Unit::from("l"),
//!                 ],
//!             ],
//!             vec![
//!                 vec![
//!                     Unit::from("m"),
//!                     Unit::from("n"),
//!                 ],
//!                 vec![
//!                     Unit::from("o"),
//!                     Unit::from("p"),
//!                 ],
//!             ],
//!         ],
//!     ]
//! );
//! ```
//!
//! ## Escape
//!
//! The Escape symbol flips the purpose of the subsequent character:
//!
//! * Escape + USV special character: the character is treated as content.
//!
//! * Escape + USV typical character: the character is ignored.
//!
//! USV with a unit that contains an Escape + End of Transmission Block, which is treated as content:
//!
//! ```usv
//! a␛␗b␟
//! ```
//!
//! Escape + newline can be helpful for typical text editor line continuations:
//!
//! ```usv
//! a␟b␞␛
//! c␟d␞␛
//! ```
//!
//! ## Synchronous Idle
//!
//! The Synchronous Idle is a heartbeat, and is especially useful for streaming data, such as to keep a connection alive.
//!
//! * It tells the data reader that data streaming is still in progress.
//!
//! * It has no effect on the output content.
//!
//! Example of a unit that contains a Synchronous Idle:
//!
//! ```usv
//! a␖b␞
//! ```
//!
//! ## USV is easy and friendly
//!
//! USV is intended to be easy to use and friendly to try:
//!
//! USV works with many kinds of data. Any data can contain any characters except the five USV characters.
//!
//! USV works with many kinds of editors. Any editor that can render the USV characters will work. We use vi, emacs, Coda, Notepad++, TextMate, Sublime, VS Code, etc.
//!
//! USV works with many kinds of tools. Any tool that can parse the USV characters will work. We use awk, sed, grep, rg, miller, etc.
//!
//! USV works with many kinds of languages. Any language that can handle UTF-8 character encoding and rendering should work. We use C, C++, C#, Elixir, Erlang, Go, Java, JavaScript, Julia, Kotlin, Perl, PHP, Python, R, Ruby, Rust, Swift, TypeScript, etc.
//!
//!
//! ## Why use USV?
//!
//! USV can handle data that contains commas, semicolons, quotes, tabs, newlines, and other special characters, all without escaping.
//!
//! USV can format units/columns/cells and records/rows/lines (similar to CSV) and groups/tables/grids and files/schemas/folios (similar to ASV).
//!
//! USV aims to be an international standard.
//!
//! USV uses Unicode characters that are semantically meaningful.
//!
//! USV works well with any typical modern editor, font, terminal, shell, search, and language.
//!
//! USV uses visible letter-width characters, and these are easy to view, select, copy, paste, search.
//!
//!
//! ## USV source code
//!
//! This repository includes example USV scripts with character parsing. The scripts are a bash shell scripts, and should run on any current Unix system or current Bash shell.
//!
//! * [usv-to-display.bash](bin/usv-to-display.bash)
//!
//! * [usv-to-debug.bash](bin/usv-to-debug.bash)
//!
//! * [usv-to-csv.bash](bin/usv-to-csv.bash)
//!
//! USV is available as a Rust crate:
//!
//! * `cargo install usv`
//!
//! * [https://crates.io/crate/usv](https://crates.io/crate/usv)
//!
//! * [https://github.com/sixarm/usv-rust-crate](https://github.com/sixarm/usv-rust-crate) (GitHub repository)
//!
//!
//! ## Legal protection for standardization
//!
//! The USV project aims to become a free open source IANA standard, much like the IANA standard for CSV.
//!
//! Until the standardization happens, the terms "USV" and "Unicode Separated Values" are trademarks of this project, and this repository is copyright 2022-2024. When IANA approves the standard, then the trademarks and copyrights become public domain.
//!
//!
//! ## Conclusion
//!
//! USV is helping us with data projects. We hope USV may help you too.
//!
//! We welcome constructive feedback about USV, as well as git issues, pull requests, and standardization help.
//!
//! <https://github.com/sixarm/usv>

// Type aliases
#[allow(dead_code)] pub type Tokens = Vec<Token>;

// Type aliases for USV naming
#[allow(dead_code)] pub type Unit = String;
#[allow(dead_code)] pub type Units = Vec<Unit>;
#[allow(dead_code)] pub type Record = Units;
#[allow(dead_code)] pub type Records = Vec<Record>;
#[allow(dead_code)] pub type Group = Records;
#[allow(dead_code)] pub type Groups = Vec<Records>;
#[allow(dead_code)] pub type File = Groups;
#[allow(dead_code)] pub type Files = Vec<File>;

// Type aliases for spreadsheet naming
#[allow(dead_code)] pub type SpreadsheetCell = String;
#[allow(dead_code)] pub type SpreadsheetCells = Vec<SpreadsheetCell>;
#[allow(dead_code)] pub type SpreadsheetRow = SpreadsheetCells;
#[allow(dead_code)] pub type SpreadsheetRows = Vec<SpreadsheetRow>;
#[allow(dead_code)] pub type SpreadsheetSheet = SpreadsheetRows;
#[allow(dead_code)] pub type SpreadsheetSheets = Vec<SpreadsheetSheet>;
#[allow(dead_code)] pub type SpreadsheetFolio = SpreadsheetSheets;
#[allow(dead_code)] pub type SpreadsheetFolios = Vec<SpreadsheetFolio>;

// Type aliases for database naming
#[allow(dead_code)] pub type DatabaseField = String;
#[allow(dead_code)] pub type DatabaseFields = Vec<DatabaseField>;
#[allow(dead_code)] pub type DatabaseRow = DatabaseFields;
#[allow(dead_code)] pub type DatabaseRows = Vec<DatabaseRow>;
#[allow(dead_code)] pub type DatabaseTable = DatabaseRows;
#[allow(dead_code)] pub type DatabaseTables = Vec<DatabaseTable>;
#[allow(dead_code)] pub type DatabaseSchema = DatabaseTables;
#[allow(dead_code)] pub type DatabaseSchemas = Vec<DatabaseSchema>;

// Token enum that holds content data or a special character value.
pub mod token; pub use token::Token;

// Iterator structs for token, unit, record, group, file.
pub mod token_iterator; pub use token_iterator::TokenIterator;
pub mod unit_iterator; pub use unit_iterator::UnitIterator;
pub mod record_iterator; pub use record_iterator::RecordIterator;
pub mod group_iterator; pub use group_iterator::GroupIterator;
pub mod file_iterator; pub use file_iterator::FileIterator;

// Iterator extensions for tokens, units, records, groups, files.
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
