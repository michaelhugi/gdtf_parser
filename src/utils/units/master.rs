//TODO check
//! Module for the unit Master for LogicalChannel used in GDTF
use std::borrow::Borrow;

use quick_xml::events::attributes::Attribute;

///Master representation for logicalChannel used in GDTF
#[derive(Debug, PartialEq, Clone)]
pub enum Master {
    None,
    Grand,
    Group,
}

///Default value of master is None
impl Default for Master {
    fn default() -> Self {
        Master::None
    }
}

///Parses a str to Master in case of an error it returns default
impl From<&str> for Master {
    fn from(s: &str) -> Self {
        use Master::*;
        match s {
            "None" => None,
            "Grand" => Grand,
            "Group" => Group,
            _ => None
        }
    }
}

///Parses am xml attribute to Master in case of an error it returns default
impl From<Attribute<'_>> for Master {
    fn from(attr: Attribute<'_>) -> Self {
        match std::str::from_utf8(attr.value.borrow()) {
            Ok(attr) => attr.into(),
            Err(_) => Self::default()
        }
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
    fn test_from_str() {
        assert_eq!(Master::Grand, "Grand".into());
        assert_eq!(Master::Group, "Group".into());
        assert_eq!(Master::None, "None".into());
        assert_eq!(Master::default(), "Something else".into());
    }

    #[test]
    fn test_from_attr_owned() {
        assert_eq!(Master::Grand, testdata::to_attr_owned(b"Grand").into());
        assert_eq!(Master::Group, testdata::to_attr_owned(b"Group").into());
        assert_eq!(Master::None, testdata::to_attr_owned(b"None").into());
        assert_eq!(Master::default(), testdata::to_attr_owned(b"Something else").into());
    }

    #[test]
    fn test_from_attr_borrowed() {
        assert_eq!(Master::Grand, testdata::to_attr_borrowed(b"Grand").into());
        assert_eq!(Master::Group, testdata::to_attr_borrowed(b"Group").into());
        assert_eq!(Master::None, testdata::to_attr_borrowed(b"None").into());
        assert_eq!(Master::default(), testdata::to_attr_borrowed(b"Something else").into());
    }
}