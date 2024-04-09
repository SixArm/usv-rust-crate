use crate as usv;

/// Convert USV groups from Vec<Vec<Vec<&str>>> into USV String.
pub fn from_vec3_str_and_usv_style_into_usv_string(data: &Vec<Vec<Vec<&str>>>, style: &usv::style::Style) -> String {
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
                    "a",
                    "b",
                ],
                vec![
                    "c",
                    "d",
                ],
            ],
            vec![
                vec![
                    "e",
                    "f",
                ],
                vec![
                    "g",
                    "h",
                ],
            ],
        ];
        let style = usv::style::Style::default();
        let actual = from_vec3_str_and_usv_style_into_usv_string(&data, &style);
        assert_eq!(actual, usv::examples::EXAMPLE_GROUPS);
    }

}
