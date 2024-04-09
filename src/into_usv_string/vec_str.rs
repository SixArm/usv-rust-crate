use crate as usv;
use usv::into_usv_string::IntoUSVString;
use usv::from::from_vec_str_and_usv_style_into_usv_string::*;

impl IntoUSVString for Vec<&str> {
    fn into_usv_string(&self) -> String {
        from_vec_str_and_usv_style_into_usv_string(&self, &usv::style::Style::default())
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
            "a",
            "b",
        ];
        let actual = data.into_usv_string();
        assert_eq!(actual, usv::examples::EXAMPLE_UNITS);
    }
}
