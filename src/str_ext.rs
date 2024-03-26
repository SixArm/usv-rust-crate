use crate as usv;

pub trait StrExt {
    fn tokens(&self) -> usv::iter::Tokens;
    fn units(&self) -> usv::iter::Units;
    fn records(&self) -> usv::iter::Records;
    fn groups(&self) -> usv::iter::Groups;
    fn files(&self) -> usv::iter::Files;
}

impl StrExt for str {

    fn tokens(&self) -> usv::iter::Tokens {
        self.chars().into()
    }

    fn units(&self) -> usv::iter::Units {
        self.chars().into()
    }

    fn records(&self) -> usv::iter::Records {
        self.chars().into()
    }

    fn groups(&self) -> usv::iter::Groups {
        self.chars().into()
    }

    fn files(&self) -> usv::iter::Files {
        self.chars().into()
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate as usv;
    use crate::examples::*;

    #[test]
    fn tokens_test() {
        let _actual: usv::iter::Tokens = EXAMPLE_FILES_STYLE_SYMBOLS.tokens();
    }

    #[test]
    fn units_test() {
        let _actual: usv::iter::Units = EXAMPLE_UNITS_STYLE_SYMBOLS.units();
    }

    #[test]
    fn records_test() {
        let _actual: usv::iter::Records = EXAMPLE_RECORDS_STYLE_SYMBOLS.records();
    }

    #[test]
    fn groups_test() {
        let _actual: usv::iter::Groups = EXAMPLE_GROUPS_STYLE_SYMBOLS.groups();
    }

    #[test]
    fn files_test() {
        let _actual: usv::iter::Files = EXAMPLE_FILES_STYLE_SYMBOLS.files();
    }

}