use crate::style::Style;

pub trait LayoutTrait {
    fn layout(style: &Style) -> Style;
}

pub enum LayoutEnum {
    Layout0,
    Layout1,
    Layout2,
    LayoutUnits,
    LayoutRecords,
    LayoutGroups,
    LayoutFiles,
}

pub mod layout_0;
pub mod layout_1;
pub mod layout_2;
pub mod layout_units;
pub mod layout_records;
pub mod layout_groups;
pub mod layout_files;
