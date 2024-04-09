use crate as usv;

/// Convert USV records from Vec<Vec<&str>> into USV String.
pub fn from_vec2_str_and_usv_style_into_usv_string(data: &Vec<Vec<&str>>, style: &usv::style::Style) -> String {
    let mut s = String::new();
    for record in data {
        for unit in record {
            s += &unit;
            s += &style.unit_separator;
            }
        s += &style.record_separator;
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
            vec![
                "a",
                "b",
            ],
            vec![
                "c",
                "d",
            ],
        ];
        let style = usv::style::Style::default();
        let actual = from_vec2_str_and_usv_style_into_usv_string(&data, &style);
        assert_eq!(actual, usv::examples::EXAMPLE_RECORDS);
    }

}
