use crate as usv;

/// Convert USV groups from Vec<Vec<Vec<String>>> into USV String.
pub fn from_vec3_string_and_usv_style_into_usv_string(data: &Vec<Vec<Vec<String>>>, style: &usv::style::Style) -> String {
    let mut s = String::new();
    for group in data {
        for record in group {
            for unit in record {
                s += &unit;
                s += &style.unit_separator;
                }
            s += &style.record_separator;
        }
        s += &style.group_separator;
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
                vec![
                    String::from("a"),
                    String::from("b"),
                ],
                vec![
                    String::from("c"),
                    String::from("d"),
                ],
            ],
            vec![
                vec![
                    String::from("e"),
                    String::from("f"),
                ],
                vec![
                    String::from("g"),
                    String::from("h"),
                ],
            ],
        ];
        let style = usv::style::Style::default();
        let actual = from_vec3_string_and_usv_style_into_usv_string(&data, &style);
        assert_eq!(actual, usv::examples::EXAMPLE_GROUPS);
    }

}
