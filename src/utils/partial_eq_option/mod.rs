///Checks if an option of a type is partial_eq but return false if one of them is None
/// assert!(!partial_eq_option(None,None))
/// assert!(partial_eq_option(Some("test),Some("test")))
pub fn partial_eq_option<T: PartialEq>(left: &Option<T>, right: &Option<T>) -> bool {
    match left {
        None => false,
        Some(left) => if let Some(right) = right { left == right } else { false }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::partial_eq_option::partial_eq_option;

    #[test]
    fn test_partial_eq_option() {
        let none: Option<&str> = None;
        assert!(!partial_eq_option(&none, &none));
        assert!(partial_eq_option(&Some(""), &Some("")));
        assert!(partial_eq_option(&Some("test"), &Some("test")));

        assert!(!partial_eq_option(&none, &Some("")));
        assert!(!partial_eq_option(&none, &Some("test")));

        assert!(!partial_eq_option(&Some(""), &none));
        assert!(!partial_eq_option(&Some("test"), &none));

        assert!(!partial_eq_option(&Some("test"), &Some("other")));
        assert!(!partial_eq_option(&Some("test"), &Some("")));
        assert!(!partial_eq_option(&Some(""), &Some("other")));
    }
}