use crate as usv;
use crate::into_usv_string::IntoUSVString;
use usv::from::from_vec4_string_and_usv_style_into_usv_string::*;

impl IntoUSVString for Vec<Vec<Vec<Vec<String>>>> {
    fn into_usv_string(&self) -> String {
        from_vec4_string_and_usv_style_into_usv_string(&self, &usv::style::Style::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as usv;

    #[test]
    fn input() {
        let files =
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
        let actual = files.into_usv_string();
        assert_eq!(actual, usv::examples::EXAMPLE_FILES);
    }

}
