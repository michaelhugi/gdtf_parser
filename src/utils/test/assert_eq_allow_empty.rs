#![cfg(test)]
//! PartialEq for comparing structs or enums that will behave differently to PartialEq when no content is in the struct or enum

use std::fmt::Debug;

///Will perform an assert_eq! on traits and enums but not panic if both are empty what may be a different behaviour to PartialEq where empty options may return false in any case
pub trait AssertEqAllowEmpty: Debug {
    fn assert_eq_allow_empty(&self, other: &Self) {
        if !self.is_eq_allow_empty(other) {
            panic!("assert_eq_allow_empty paniced! \n left: {:?} \n right: {:?}", self, other)
        }
    }

    fn assert_eq_allow_empty_no_log(&self, other: &Self) {
        if !self.is_eq_allow_empty_no_log(other) {
            panic!("assert_eq_allow_empty paniced! \n left: {:?} \n right: {:?}", self, other)
        }
    }

    fn is_eq_allow_empty(&self, other: &Self) -> bool {
        let out = self.is_eq_allow_empty_no_log(&other);
        if !out {
            println!("Structs were not equal \nleft: {:?} \nright: {:?}", &self, &other);
        }
        out
    }


    fn is_eq_allow_empty_no_log(&self, other: &Self) ->bool;

    fn is_eq_allow_option<T: PartialEq>(left: &Option<T>, right: &Option<T>) -> bool {
        match left {
            None => if let None = right { true } else { false },
            Some(left) => if let Some(right) = right { right == left } else { false }
        }
    }

}

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter};
    use std::fmt;

    use crate::utils::test::assert_eq_allow_empty::AssertEqAllowEmpty;

    #[derive(Debug, PartialEq)]
    struct Teststruct(Option<String>);

    impl Display for Teststruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match &self.0 {
                None => write!(f, "None"),
                Some(val) => write!(f, "Some({})", val)
            }
        }
    }

    impl AssertEqAllowEmpty for Teststruct {
        fn is_eq_allow_empty_no_log(&self, other: &Self) -> bool {
            Self::is_eq_allow_option(&self.0, &other.0)
        }
    }

    #[test]
    fn test_eq() {
        Teststruct::assert_eq_allow_empty(&Teststruct(Some("one".to_string())), &Teststruct(Some("one".to_string())));
        Teststruct::assert_eq_allow_empty(&Teststruct(Some("".to_string())), &Teststruct(Some("".to_string())));
        Teststruct::assert_eq_allow_empty(&Teststruct(None), &Teststruct(None));
    }

    #[test]
    #[should_panic]
    fn test_ne1() {
        Teststruct::assert_eq_allow_empty_no_log(&Teststruct(Some("one".to_string())), &Teststruct(Some("two".to_string())));
    }

    #[test]
    #[should_panic]
    fn test_ne2() {
        Teststruct::assert_eq_allow_empty_no_log(&Teststruct(Some("".to_string())), &Teststruct(Some("some".to_string())));
    }

    #[test]
    #[should_panic]
    fn test_ne3() {
        Teststruct::assert_eq_allow_empty_no_log(&Teststruct(None), &Teststruct(Some("some".to_string())));
    }

    #[test]
    #[should_panic]
    fn test_ne4() {
        Teststruct::assert_eq_allow_empty_no_log(&Teststruct(Some("some".to_string())), &Teststruct(None));
    }

    #[test]
    #[should_panic]
    fn test_ne5() {
        Teststruct::assert_eq_allow_empty_no_log(&Teststruct(None), &Teststruct(Some("None".to_string())));
    }

    #[test]
    #[should_panic]
    fn test_ne6() {
        Teststruct::assert_eq_allow_empty_no_log(&Teststruct(Some("None".to_string())), &Teststruct(None));
    }
}