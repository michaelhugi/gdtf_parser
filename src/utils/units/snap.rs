//! Module for the unit Snap for LogicalChannel used in GDTF


use quick_xml::events::attributes::Attribute;

use crate::utils::deparse;

///Snap representation for logicalChannel used in GDTF
/// If snap is enabled, the logical channel will not fade between values. Instead, it will jump directly to the new value
#[derive(Debug, PartialEq, Clone)]
pub enum Snap {
    No,
    Yes,
    On,
    Off,
}

///Default for Snap is No
impl Default for Snap {
    fn default() -> Self {
        Snap::No
    }
}

impl Snap {
    ///Creates a new snap from a string defined in gdtf-xml
    ///## Examples
    /// ```rust
    /// use gdtf_parser::utils::units::snap::Snap;
    ///
    /// assert_eq!(Snap::No,Snap::new_from_str("No"));
    /// assert_eq!(Snap::Yes,Snap::new_from_str("Yes"));
    /// assert_eq!(Snap::On,Snap::new_from_str("On"));
    /// assert_eq!(Snap::Off,Snap::new_from_str("Off"));
    /// assert_eq!(Snap::No,Snap::new_from_str("Anything else"));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        use Snap::*;
        match s {
            "No" => No,
            "Yes" => Yes,
            "On" => On,
            "Off" => Off,
            _ => Default::default()
        }
    }
    ///Creates a new snap from an xml attribute deparsed by quick-xml
    /// ## Examples
    /// ```rust
    /// use gdtf_parser::utils::units::snap::Snap;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    ///
    /// assert_eq!(Snap::No,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"No") }));
    /// assert_eq!(Snap::Yes,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Yes") }));
    /// assert_eq!(Snap::On,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"On") }));
    /// assert_eq!(Snap::Off,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Off") }));
    /// assert_eq!(Snap::No,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Anything else") }));
    /// ```

    pub fn new_from_attr(attr: Attribute) -> Self {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::snap::Snap;

    #[test]
    fn test_new_from_str() {
        assert_eq!(Snap::No, Snap::new_from_str("No"));
        assert_eq!(Snap::Yes, Snap::new_from_str("Yes"));
        assert_eq!(Snap::On, Snap::new_from_str("On"));
        assert_eq!(Snap::Off, Snap::new_from_str("Off"));
        assert_eq!(Snap::No, Snap::new_from_str("Anything else"));
    }

    #[test]
    fn test_new_from_attr_owned() {
        assert_eq!(Snap::No, Snap::new_from_attr(testdata::to_attr_owned(b"No")));
        assert_eq!(Snap::Yes, Snap::new_from_attr(testdata::to_attr_owned(b"Yes")));
        assert_eq!(Snap::On, Snap::new_from_attr(testdata::to_attr_owned(b"On")));
        assert_eq!(Snap::Off, Snap::new_from_attr(testdata::to_attr_owned(b"Off")));
        assert_eq!(Snap::No, Snap::new_from_attr(testdata::to_attr_owned(b"Anything else")));
    }

    #[test]
    fn test_new_from_attr_borrowed() {
        assert_eq!(Snap::No, Snap::new_from_attr(testdata::to_attr_borrowed(b"No")));
        assert_eq!(Snap::Yes, Snap::new_from_attr(testdata::to_attr_borrowed(b"Yes")));
        assert_eq!(Snap::On, Snap::new_from_attr(testdata::to_attr_borrowed(b"On")));
        assert_eq!(Snap::Off, Snap::new_from_attr(testdata::to_attr_borrowed(b"Off")));
        assert_eq!(Snap::No, Snap::new_from_attr(testdata::to_attr_borrowed(b"Anything else")));
    }


    #[test]
    fn test_default() {
        assert_eq!(Snap::No, Default::default());
    }
}