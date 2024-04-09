use crate as usv;

/// Convert USV units from Vec<&str> into USV String.
pub fn from_vec_str_and_usv_style_into_usv_string(data: &Vec<&str>, style: &usv::style::Style) -> String {
    let mut s = String::new();
    for unit in data {
        s += &unit;
        s += &style.unit_separator;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data =
        vec![
            "a",
            "b",
        ];
        let style = usv::style::Style::default();
        let actual = from_vec_str_and_usv_style_into_usv_string(&data, &style);
        assert_eq!(actual, usv::examples::EXAMPLE_UNITS);
    }

}
