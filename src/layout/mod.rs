//! USV layout is the terminology for how items are displayed.
//!
//! The layout helps make USV more useful for more people.
//!
//! The layout is not part of the USV specification.
//!
//! Currently these are the layouts:
//!
//! * layout-0: Show each item with no line around it.
//!   This is no layout, in other words one long line.
//!
//! * layout-1: Show each item with one line around it.
//!   This is like single-space lines for long form text.
//!
//! * layout-2: Show each item with two lines around it.
//!   This is like double-space lines for long form text.
//!
//! * layout-units: Show each unit on one line.
//!   This can be helpful for line-oriented tools.
//!
//! * layout-records: Show each record on one line.
//!   This is like a typical spreadsheet sheet export.
//!
//! * layout-groups: Show each group on one line.
//!   This can be helpful for folio-oriented tools.
//!
//! * layout-files: Show one file on one line.
//!   This can be helpful for archive-oriented tools.
//!
//! You can create your own layouts that are equivalent to the above.
//!
//! This USV Rust crate provides layout capabilities:
//!
//! * LayoutTrait provides the layout() function that adjusts a style.

use crate::style::Style;

pub mod layout_0;
pub mod layout_1;
pub mod layout_2;
pub mod layout_units;
pub mod layout_records;
pub mod layout_groups;
pub mod layout_files;
