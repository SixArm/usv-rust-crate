use crate as usv;
use crate::into_usv_string::IntoUSVString;
use usv::from::from_vec4_str_and_usv_style_into_usv_string::*;

impl IntoUSVString for Vec<Vec<Vec<Vec<&str>>>> {
    fn into_usv_string(&self) -> String {
        from_vec4_str_and_usv_style_into_usv_string(&self, &usv::style::Style::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as usv;

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
        let actual = data.into_usv_string();
        assert_eq!(actual, usv::examples::EXAMPLE_FILES);
    }

}
