/// svec! makes a string vector from an array of &str.
///
/// Example:
///
/// ```
/// use usv::svec;
/// let items = svec!["a", "b", "c"];
/// assert_eq!(
///     items,
///     vec![
///         String::from("a"),
///         String::from("b"),
///         String::from("c"),
///     ]
/// );
/// ```
///
#[macro_export]
macro_rules! svec[
    ($($x:expr),*) => (
        vec![$($x),*].into_iter()
                     .map(|s: &'static str| s.to_string())
                     .collect::<Vec<String>>()
    );
    ($($x:expr,)*) => (svec![$($x),*]);
];
