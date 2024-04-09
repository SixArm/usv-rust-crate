use crate as usv;

/// Convert USV files from Vec<Vec<Vec<Vec<String>> into USV String.
pub fn from_vec4_string_and_usv_style_into_usv_string(data: &Vec<Vec<Vec<Vec<String>>>>, style: &usv::style::Style) -> String {
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
            ],
            vec![
                vec![
                    vec![
                        String::from("i"),
                        String::from("j"),
                    ],
                    vec![
                        String::from("k"),
                        String::from("l"),
                    ],
                ],
                vec![
                    vec![
                        String::from("m"),
                        String::from("n"),
                    ],
                    vec![
                        String::from("o"),
                        String::from("p"),
                    ],
                ],
            ],
        ];
        let style = usv::style::Style::default();
        let actual = from_vec4_string_and_usv_style_into_usv_string(&data, &style);
        assert_eq!(actual, usv::examples::EXAMPLE_FILES);
    }

}
