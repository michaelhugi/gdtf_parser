#![cfg(test)]

//! A lot of PartialEq implementations will return false in this crate in case both have no content (either Option or another Enum). This module contains a trait that will handle this cases for testing comparison

use std::fmt::Debug;

use backtrace::Backtrace;

///Checks if two structs or enums are equal and has some additional method for testing. This is in addition to PartialEq because a lot of implementations of PartialEq will return false in case some content is empty (Option or other Enum) because this is more useful when implementing the crate to a project.
pub trait PartialEqAllowEmpty: Debug {
    ///Compares if two structs or enums are the same no matter if they have content or not and panics if they aren't. log defines if underlying PartialEqAllowEmpty impl should log additionally if they are not equal
    fn assert_eq_allow_empty(&self, other: &Self, log: bool) {
        if !self.is_eq_allow_empty(other, log) {
            panic!("assert_eq_allow_empty! \n left: {:?} \n right: {:?}", self, other)
        }
    }

    ///The same as assert_eq_allow_empty but panics if they are equal.
    fn assert_ne_allow_empty(&self, other: &Self) {
        if self.is_eq_allow_empty(other, false) {
            panic!("assert_ne_allow_empty! \n left: {:?} \n right: {:?}", self, other)
        }
    }
    ///Returns true if two objects are equal, else false. If they are not equal it will also log left and right if log == true
    fn is_eq_allow_empty(&self, other: &Self, log: bool) -> bool {
        let out = self.is_eq_allow_empty_impl(&other, log);
        if !out && log {
            let bt = Backtrace::new();
            println!("\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\nStructs were not equal\n----------------------------------------------------------------------------- \n-----------------------------------------------------------------------------\nleft: {:?} \nright: {:?}\n\n{:#?}\n\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\n-----\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\n", &self, &other, bt);
        }
        out
    }
    ///Returns true if two objects are equal, else false. impl, that this must be implemented. If log==true, this value may be passed to other calls of is_eq_allow_empty.
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool;

    ///Compares if two optionsif they the same no matter if they have content or not and panics if they aren't.
    fn assert_eq_allow_option<T: PartialEq + Debug>(left: &Option<T>, right: &Option<T>, log: bool) {
        if !Self::is_eq_allow_option(left, right, log) {
            panic!("assert_eq_allow_empty! \n left: {:?} \n right: {:?}", left, right)
        }
    }

    ///The same as assert_eq_allow_option but panics if they are equal.
    fn assert_ne_allow_option<T: PartialEq + Debug>(left: &Option<T>, right: &Option<T>) {
        if Self::is_eq_allow_option(left, right, false) {
            panic!("assert_ne_allow_empty! \n left: {:?} \n right: {:?}", left, right)
        }
    }

    ///Helper method for easier implementation of the trait if two options are compared. Can be used in is_eq_allow_empty_impl if needed.
    fn is_eq_allow_option<T: PartialEq + Debug>(left: &Option<T>, right: &Option<T>, log: bool) -> bool {
        match left {
            None => if let None = right { true } else {
                if log {
                    let bt = Backtrace::new();
                    println!("\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\nStructs were not equal\n----------------------------------------------------------------------------- \n-----------------------------------------------------------------------------\nleft: None \nright: {:?}\n\n{:#?}\n\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\n-----\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\n", &right, bt);
                }
                false
            },
            Some(left) => if let Some(right) = right {
                let output = right == left;
                if !output && log {
                    let bt = Backtrace::new();
                    println!("\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\nStructs were not equal\n----------------------------------------------------------------------------- \n-----------------------------------------------------------------------------\nleft: {:?} \nright: {:?}\n\n{:#?}\n\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\n-----\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\n", &left, &right, bt);
                }
                output
            } else {
                if log {
                    let bt = Backtrace::new();
                    println!("\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\nStructs were not equal\n----------------------------------------------------------------------------- \n-----------------------------------------------------------------------------\nleft: {:?} \nright: None\n\n{:#?}\n\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\n-----\n-----------------------------------------------------------------------------\n-----------------------------------------------------------------------------\n", &left, bt);
                }
                false
            }
        }
    }

    ///Helper method for easier implementation of the trait if two options are compared. Can be used in is_eq_allow_empty_impl if needed and respects if underlying
    fn is_eq_allow_option_allow_empty<T: PartialEqAllowEmpty>(left: &Option<T>, right: &Option<T>, log: bool) -> bool {
        match left {
            None => if let None = right { true } else { false },
            Some(left) => if let Some(right) = right { right.is_eq_allow_empty(left, log) } else { false }
        }
    }


    ///Says if two vecs have the same items compared with PartialEqAllowEmpty where the order does matter. If log is true, eventual inconsistency will be logged additionally
    fn is_vec_eq_ordered<T: PartialEqAllowEmpty>(one: &Vec<T>, two: &Vec<T>, log: bool) -> bool {
        if one.len() != two.len() {
            println!("Testing {} for vec returned not the same amount of items", std::any::type_name::<T>());
            return false;
        }
        for (index, one) in one.iter().enumerate() {
            let two = two.get(index);
            let test = match two {
                None => false,
                Some(two) => one.is_eq_allow_empty(&two, log)
            };
            if !test {
                return false;
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::units::node::Node;

    #[derive(Debug, PartialEq)]
    struct TeststructOption(Option<String>);

    impl PartialEqAllowEmpty for TeststructOption {
        fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
            Self::is_eq_allow_option(&self.0, &other.0, log)
        }
    }

    #[test]
    fn test_assert_eq_allow_empty() {
        TeststructOption(Some("test".to_string())).assert_eq_allow_empty(&TeststructOption(Some("test".to_string())), true);
        TeststructOption(Some("".to_string())).assert_eq_allow_empty(&TeststructOption(Some("".to_string())), true);
        TeststructOption(None).assert_eq_allow_empty(&TeststructOption(None), true);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_empty_panic_1() {
        TeststructOption(Some("test".to_string())).assert_eq_allow_empty(&TeststructOption(Some("other".to_string())), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_empty_panic_2() {
        TeststructOption(Some("test".to_string())).assert_eq_allow_empty(&TeststructOption(Some("".to_string())), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_empty_panic_3() {
        TeststructOption(Some("".to_string())).assert_eq_allow_empty(&TeststructOption(Some("other".to_string())), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_empty_panic_4() {
        TeststructOption(Some("test".to_string())).assert_eq_allow_empty(&TeststructOption(None), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_empty_panic_5() {
        TeststructOption(Some("".to_string())).assert_eq_allow_empty(&TeststructOption(None), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_empty_panic_6() {
        TeststructOption(None).assert_eq_allow_empty(&TeststructOption(Some("other".to_string())), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_empty_panic_7() {
        TeststructOption(None).assert_eq_allow_empty(&TeststructOption(Some("".to_string())), false);
    }

    #[test]
    fn test_assert_ne_allow_empty() {
        TeststructOption(None).assert_ne_allow_empty(&TeststructOption(Some("".to_string())));
        TeststructOption(None).assert_ne_allow_empty(&TeststructOption(Some("other".to_string())));
        TeststructOption(Some("test".to_string())).assert_ne_allow_empty(&TeststructOption(Some("".to_string())));
        TeststructOption(Some("test".to_string())).assert_ne_allow_empty(&TeststructOption(Some("other".to_string())));
        TeststructOption(Some("".to_string())).assert_ne_allow_empty(&TeststructOption(Some("other".to_string())));
        TeststructOption(Some("test".to_string())).assert_ne_allow_empty(&TeststructOption(None));
        TeststructOption(Some("".to_string())).assert_ne_allow_empty(&TeststructOption(None));
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_allow_empty_panic_1() {
        TeststructOption(None).assert_ne_allow_empty(&TeststructOption(None));
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_allow_empty_panic_2() {
        TeststructOption(Some("".to_string())).assert_ne_allow_empty(&TeststructOption(Some("".to_string())));
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_allow_empty_panic_3() {
        TeststructOption(Some("test".to_string())).assert_ne_allow_empty(&TeststructOption(Some("test".to_string())));
    }

    #[test]
    fn test_is_eq_allow_empty() {
        assert!(TeststructOption(None).is_eq_allow_empty(&TeststructOption(None), true));
        assert!(TeststructOption(Some("".to_string())).is_eq_allow_empty(&TeststructOption(Some("".to_string())), true));
        assert!(TeststructOption(Some("test".to_string())).is_eq_allow_empty(&TeststructOption(Some("test".to_string())), true));
        assert!(!TeststructOption(None).is_eq_allow_empty(&TeststructOption(Some("".to_string())), false));
        assert!(!TeststructOption(None).is_eq_allow_empty(&TeststructOption(Some("other".to_string())), false));
        assert!(!TeststructOption(Some("".to_string())).is_eq_allow_empty(&TeststructOption(Some("other".to_string())), false));
        assert!(!TeststructOption(Some("test".to_string())).is_eq_allow_empty(&TeststructOption(Some("other".to_string())), false));
        assert!(!TeststructOption(Some("".to_string())).is_eq_allow_empty(&TeststructOption(None), false));
        assert!(!TeststructOption(Some("test".to_string())).is_eq_allow_empty(&TeststructOption(None), false));
    }

    #[test]
    fn test_assert_eq_allow_option() {
        let none: Option<&str> = None;
        TeststructOption::assert_eq_allow_option(&none, &none, true);
        TeststructOption::assert_eq_allow_option(&Some("test"), &Some("test"), true);
        TeststructOption::assert_eq_allow_option(&Some(""), &Some(""), true);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_option_panic_1() {
        let none: Option<&str> = None;
        TeststructOption::assert_eq_allow_option(&none, &Some(""), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_option_panic_2() {
        let none: Option<&str> = None;
        TeststructOption::assert_eq_allow_option(&none, &Some("test"), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_option_panic_3() {
        let none: Option<&str> = None;
        TeststructOption::assert_eq_allow_option(&Some(""), &none, false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_option_panic_4() {
        let none: Option<&str> = None;
        TeststructOption::assert_eq_allow_option(&Some("test"), &none, false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_option_panic_5() {
        TeststructOption::assert_eq_allow_option(&Some("test"), &Some("test2"), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_option_panic_6() {
        TeststructOption::assert_eq_allow_option(&Some(""), &Some("test2"), false);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq_allow_option_panic_7() {
        TeststructOption::assert_eq_allow_option(&Some("test"), &Some(""), false);
    }

    #[test]
    fn test_assert_ne_allow_option() {
        let none: Option<&str> = None;
        TeststructOption::assert_ne_allow_option(&none, &Some(""));
        TeststructOption::assert_ne_allow_option(&none, &Some("test"));
        TeststructOption::assert_ne_allow_option(&Some(""), &none);
        TeststructOption::assert_ne_allow_option(&Some("test"), &none);
        TeststructOption::assert_ne_allow_option(&Some("test"), &Some("test2"));
        TeststructOption::assert_ne_allow_option(&Some(""), &Some("test2"));
        TeststructOption::assert_ne_allow_option(&Some("test"), &Some(""));
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_allow_option_panic_1() {
        let none: Option<&str> = None;
        TeststructOption::assert_ne_allow_option(&none, &none);
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_allow_option_panic_2() {
        TeststructOption::assert_ne_allow_option(&Some("test"), &Some("test"));
    }

    #[test]
    #[should_panic]
    fn test_assert_ne_allow_option_panic_3() {
        TeststructOption::assert_ne_allow_option(&Some(""), &Some(""));
    }

    #[test]
    fn test_is_eq_allow_option() {
        let none: Option<&str> = None;
        assert!(TeststructOption::is_eq_allow_option(&none, &none, true));
        assert!(TeststructOption::is_eq_allow_option(&Some("test"), &Some("test"), true));
        assert!(TeststructOption::is_eq_allow_option(&Some(""), &Some(""), true));
        assert!(!TeststructOption::is_eq_allow_option(&none, &Some(""), false));
        assert!(!TeststructOption::is_eq_allow_option(&none, &Some("test"), false));
        assert!(!TeststructOption::is_eq_allow_option(&Some(""), &none, false));
        assert!(!TeststructOption::is_eq_allow_option(&Some("test"), &none, false));
        assert!(!TeststructOption::is_eq_allow_option(&Some(""), &Some("test"), false));
        assert!(!TeststructOption::is_eq_allow_option(&Some("test"), &Some(""), false));
        assert!(!TeststructOption::is_eq_allow_option(&Some("test"), &Some("other"), false));
    }

    #[test]
    fn test_is_eq_allow_option_allow_empty() {
        let none: Option<Node> = None;
        assert!(TeststructOption::is_eq_allow_option_allow_empty(&none, &none, true));
        assert!(TeststructOption::is_eq_allow_option_allow_empty(&Some(Node::new_unchecked_s("test")), &Some(Node::new_unchecked_s("test")), true));
        assert!(TeststructOption::is_eq_allow_option_allow_empty(&Some(Node::new_unchecked_s("")), &Some(Node::new_unchecked_s("")), true));
        assert!(!TeststructOption::is_eq_allow_option_allow_empty(&none, &Some(Node::new_unchecked_s("")), false));
        assert!(!TeststructOption::is_eq_allow_option_allow_empty(&none, &Some(Node::new_unchecked_s("test")), false));
        assert!(!TeststructOption::is_eq_allow_option_allow_empty(&Some(Node::new_unchecked_s("")), &none, false));
        assert!(!TeststructOption::is_eq_allow_option_allow_empty(&Some(Node::new_unchecked_s("test")), &none, false));
        assert!(!TeststructOption::is_eq_allow_option_allow_empty(&Some(Node::new_unchecked_s("")), &Some(Node::new_unchecked_s("test")), false));
        assert!(!TeststructOption::is_eq_allow_option_allow_empty(&Some(Node::new_unchecked_s("test")), &Some(Node::new_unchecked_s("")), false));
        assert!(!TeststructOption::is_eq_allow_option_allow_empty(&Some(Node::new_unchecked_s("test")), &Some(Node::new_unchecked_s("other")), false));
    }
}