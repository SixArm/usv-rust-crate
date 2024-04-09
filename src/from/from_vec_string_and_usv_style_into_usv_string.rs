use crate as usv;

/// Convert USV units from Vec<String> into USV String.
pub fn from_vec_string_and_usv_style_into_usv_string(data: &Vec<String>, style: &usv::style::Style) -> String {
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
            String::from("a"),
            String::from("b"),
        ];
        let style = usv::style::Style::default();
        let actual = from_vec_string_and_usv_style_into_usv_string(&data, &style);
        assert_eq!(actual, usv::examples::EXAMPLE_UNITS);
    }

}
