use crate as usv;
use crate::into_usv_string::IntoUSVString;
use usv::from::from_vec_string_and_usv_style_into_usv_string::*;

impl IntoUSVString for Vec<String> {
    fn into_usv_string(&self) -> String {
        from_vec_string_and_usv_style_into_usv_string(&self, &usv::style::Style::default())
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
            String::from("a"),
            String::from("b"),
        ];
        let actual = data.into_usv_string();
        assert_eq!(actual, usv::examples::EXAMPLE_UNITS);
    }

}
