//! Module for the unit Offset used for DMXChannel used in GDTF

use std::str::FromStr;

use quick_xml::events::attributes::Attribute;

use crate::utils::deparse;

///The unit Offset used for DMXChannel used in GDTF
///Relative addresses of the current DMX channel from highest to least significant
#[derive(Debug, PartialEq, Clone)]
pub struct Offset(pub Vec<i32>);

impl Offset {
    ///Creates a new Option<Offset> from a string defined in gdtf-xml
    /// ## Examples
    /// ```rust
    ///
    /// use gdtf_parser::utils::units::offset::Offset;
    /// assert!(Offset::new_from_str("None").is_none());
    /// assert_eq!(Offset(vec![1]), Offset::new_from_str("1").unwrap());
    /// assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_str("0,1,2,-3").unwrap());
    ///
    /// //Handling of wrong values
    ///
    /// //More than i32::MAX
    /// assert!(Offset::new_from_str("2147483648").is_none());
    /// //Less than i32::MIN
    /// assert!(Offset::new_from_str("-2147483649").is_none());
    /// assert!(Offset::new_from_str("").is_none());
    /// assert!(Offset::new_from_str("Something else").is_none());
    /// ```
    pub fn new_from_str(s: &str) -> Option<Self> {
        if s == "None" {
            return None;
        }
        let mut v = Vec::new();
        for s in s.split(',').into_iter() {
            match i32::from_str(s) {
                Ok(s) => v.push(s),
                Err(_) => return None
            }
        }
        Some(Offset(v))
    }

    ///Creates a new Option<Offset> from a quick-xml-attribute defined in gdtf-xml
    /// ## Examples
    /// ```rust
    /// use gdtf_parser::utils::units::offset::Offset;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert!(Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"None")}).is_none());
    /// assert_eq!(Offset(vec![1]), Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"1")}).unwrap());
    /// assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"0,1,2,-3")}).unwrap());
    ///
    /// //Handling wrong values
    ///
    /// //More than i32::MAX
    /// assert!(Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"2147483648")}).is_none());
    /// //Less than i32::MIN
    /// assert!(Offset::new_from_str("-2147483649").is_none());
    /// assert!(Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"")}).is_none());
    /// assert!(Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Something else")}).is_none());
    /// ```
    pub fn new_from_attr(attr: Attribute) -> Option<Self> {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
    /// Creates a new Offset from a Vec<i32>
    /// ## Examples
    /// ```rust
    ///  use gdtf_parser::utils::units::offset::Offset;
    ///  assert_eq!(Offset(vec![]), Offset::new(vec![]));
    ///  assert_eq!(Offset(vec![1]), Offset::new(vec![1]));
    ///  assert_eq!(Offset(vec![1, 3]), Offset::new(vec![1, 3]));
    /// ```
    pub fn new(offsets: Vec<i32>) -> Self {
        Self(offsets)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::offset::Offset;

    #[test]
    fn test_new_from_str() {
        assert!(Offset::new_from_str("None").is_none());
        assert_eq!(Offset(vec![1]), Offset::new_from_str("1").unwrap());
        assert_eq!(Offset(vec![-1]), Offset::new_from_str("-1").unwrap());
        assert_eq!(Offset(vec![1, 2]), Offset::new_from_str("1,2").unwrap());
        assert_eq!(Offset(vec![1, -2]), Offset::new_from_str("1,-2").unwrap());
        assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_str("0,1,2,-3").unwrap());
        assert_eq!(Offset(vec![i32::MAX, i32::MIN]), Offset::new_from_str("2147483647,-2147483648").unwrap());

        assert!(Offset::new_from_str("").is_none());
        assert!(Offset::new_from_str("Something else").is_none());
        assert!(Offset::new_from_str("2147483648,-2147483648").is_none());
        assert!(Offset::new_from_str("2147483648,-2147483649").is_none());
    }

    #[test]
    fn test_new_from_attr_owned() {
        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"None")).is_none());
        assert_eq!(Offset(vec![1]), Offset::new_from_attr(testdata::to_attr_owned(b"1")).unwrap());
        assert_eq!(Offset(vec![-1]), Offset::new_from_attr(testdata::to_attr_owned(b"-1")).unwrap());
        assert_eq!(Offset(vec![1, 2]), Offset::new_from_attr(testdata::to_attr_owned(b"1,2")).unwrap());
        assert_eq!(Offset(vec![1, -2]), Offset::new_from_attr(testdata::to_attr_owned(b"1,-2")).unwrap());
        assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_attr(testdata::to_attr_owned(b"0,1,2,-3")).unwrap());
        assert_eq!(Offset(vec![i32::MAX, i32::MIN]), Offset::new_from_attr(testdata::to_attr_owned(b"2147483647,-2147483648")).unwrap());

        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"Something else")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"2147483648,-2147483648")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"2147483648,-2147483649")).is_none());
    }

    #[test]
    fn test_new_from_attr_borrowed() {
        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"None")).is_none());
        assert_eq!(Offset(vec![1]), Offset::new_from_attr(testdata::to_attr_borrowed(b"1")).unwrap());
        assert_eq!(Offset(vec![-1]), Offset::new_from_attr(testdata::to_attr_borrowed(b"-1")).unwrap());
        assert_eq!(Offset(vec![1, 2]), Offset::new_from_attr(testdata::to_attr_borrowed(b"1,2")).unwrap());
        assert_eq!(Offset(vec![1, -2]), Offset::new_from_attr(testdata::to_attr_borrowed(b"1,-2")).unwrap());
        assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_attr(testdata::to_attr_borrowed(b"0,1,2,-3")).unwrap());
        assert_eq!(Offset(vec![i32::MAX, i32::MIN]), Offset::new_from_attr(testdata::to_attr_borrowed(b"2147483647,-2147483648")).unwrap());

        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"Something else")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"2147483648,-2147483648")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"2147483648,-2147483649")).is_none());
    }

    #[test]
    fn test_new() {
        assert_eq!(Offset(vec![]), Offset::new(vec![]));
        assert_eq!(Offset(vec![1]), Offset::new(vec![1]));
        assert_eq!(Offset(vec![1, 3]), Offset::new(vec![1, 3]));
    }
}