use crate as usv;
use crate::into_usv_string::IntoUSVString;
use usv::from::from_vec3_string_and_usv_style_into_usv_string::*;

impl IntoUSVString for Vec<Vec<Vec<String>>> {
    fn into_usv_string(&self) -> String {
        from_vec3_string_and_usv_style_into_usv_string(&self, &usv::style::Style::default())
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
        let actual = data.into_usv_string();
        assert_eq!(actual, usv::examples::EXAMPLE_GROUPS);
    }

}
