//! Module for the unit Master for LogicalChannel in GDTF

use quick_xml::events::attributes::Attribute;

use crate::utils::deparse;

///Master representation for logicalChannel in GDTF
///Defines if all the subordinate channel functions react to a Group Control defined by the control system
#[derive(Debug, PartialEq, Clone)]
pub enum Master {
    None,
    Grand,
    Group,
}

///Default value of Master is None
impl Default for Master {
    fn default() -> Self {
        Master::None
    }
}

impl Master {
    ///Parses a string defined in gdtf-xml-description to a Master
    /// ```rust
    /// use gdtf_parser::utils::units::master::Master;
    /// assert_eq!(Master::None, Master::new_from_str("None"));
    /// assert_eq!(Master::Grand, Master::new_from_str("Grand"));
    /// assert_eq!(Master::Group, Master::new_from_str("Group"));
    /// assert_eq!(Master::None, Master::new_from_str("Anything strange like ȸ"));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        use Master::*;
        match s {
            "Grand" => Grand,
            "Group" => Group,
            _ => None
        }
    }

    ///Parses a quick-xml-attribute defined in gdtf-xml-description to a Master
    /// ```rust
    /// use gdtf_parser::utils::units::master::Master;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(Master::None, Master::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"None")}));
    /// assert_eq!(Master::Grand, Master::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Grand")}));
    /// assert_eq!(Master::Group, Master::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Group")}));
    /// assert_eq!(Master::None, Master::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Anything strange like {")}));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::master::Master;

    #[test]
    fn test_default() {
        assert_eq!(Master::None, Default::default())
    }

    #[test]
    fn test_new_from_str() {
        assert_eq!(Master::None, Master::new_from_str("None"));
        assert_eq!(Master::Grand, Master::new_from_str("Grand"));
        assert_eq!(Master::Group, Master::new_from_str("Group"));
        assert_eq!(Master::None, Master::new_from_str("Anything strange like ȸ"));
    }

    #[test]
    fn test_new_from_attr_owned() {
        assert_eq!(Master::None, Master::new_from_attr(testdata::to_attr_owned(b"None")));
        assert_eq!(Master::Grand, Master::new_from_attr(testdata::to_attr_owned(b"Grand")));
        assert_eq!(Master::Group, Master::new_from_attr(testdata::to_attr_owned(b"Group")));
        assert_eq!(Master::None, Master::new_from_attr(testdata::to_attr_owned(b"Anything strange like {")));
    }

    #[test]
    fn test_new_from_attr_borrowed() {
        assert_eq!(Master::None, Master::new_from_attr(testdata::to_attr_borrowed(b"None")));
        assert_eq!(Master::Grand, Master::new_from_attr(testdata::to_attr_borrowed(b"Grand")));
        assert_eq!(Master::Group, Master::new_from_attr(testdata::to_attr_borrowed(b"Group")));
        assert_eq!(Master::None, Master::new_from_attr(testdata::to_attr_borrowed(b"Anything strange like {")));
    }
}