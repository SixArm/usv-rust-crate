use crate as usv;

/// Convert USV files from Vec<Vec<Vec<Vec<&str>>>> into USV String.
pub fn from_vec4_str_and_usv_style_into_usv_string(data: &Vec<Vec<Vec<Vec<&str>>>>, style: &usv::style::Style) -> String {
    let mut s = String::new();
    for file in data {
        for group in file {
            for record in group {
                for unit in record {
                    s += &unit;
                    s += &style.unit_separator;
                    }
                s += &style.record_separator;
            }
            s += &style.group_separator;
        }
        s += &style.file_separator;
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
            ],
            vec![
                vec![
                    vec![
                        "i",
                        "j",
                    ],
                    vec![
                        "k",
                        "l",
                    ],
                ],
                vec![
                    vec![
                        "m",
                        "n",
                    ],
                    vec![
                        "o",
                        "p",
                    ],
                ],
            ],
        ];
        let style = usv::style::Style::default();
        let actual = from_vec4_str_and_usv_style_into_usv_string(&data, &style);
        assert_eq!(actual, usv::examples::EXAMPLE_FILES);
    }

}