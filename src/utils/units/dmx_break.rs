//TODO check
//! Module for the unit DMXBreak used for DMXChannel used in GDTF
//!
//! # Definition of DMXBreak
//! term used when a fixture needs more than one DMX start address


use std::borrow::Borrow;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;

///DMXBreak used for DMXChannel in GDTF
#[derive(Debug, PartialEq, Clone)]
pub enum DmxBreak {
    ///Number of the DMXBreak; Default value: 1
    Value(u32),
    ///means that this number will be overwritten by Geometry Reference
    Overwrite,
}

///Default value for DMXBreak is 1
impl Default for DmxBreak {
    fn default() -> Self {
        Self::Value(1)
    }
}

///Parses &str to DMXBreak. In case of any error it returns Default
impl From<&str> for DmxBreak {
    fn from(s: &str) -> Self {
        use DmxBreak::*;
        if s == "Overwrite" {
            Overwrite
        } else {
            Value(u32::from_str(s).unwrap_or( 1))
        }
    }
}

/// Parses an xml attribute from an xml attribute formatted yyyy-mm-ddThh:mm:ss
impl From<Attribute<'_>> for DmxBreak {
    fn from(attr: Attribute<'_>) -> Self {
        match std::str::from_utf8(attr.value.borrow()) {
            Ok(attr) => attr.into(),
            Err(_) => Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::dmx_break::DmxBreak;

    #[test]
    fn test_from_str_valid() {
        assert_eq!(DmxBreak::Value(23), "23".into());
        assert_eq!(DmxBreak::Value(1), "1".into());
        assert_eq!(DmxBreak::Value(145), "145".into());
        assert_eq!(DmxBreak::Overwrite, "Overwrite".into());
    }

    #[test]
    fn test_from_attr_owned_valid() {
        assert_eq!(DmxBreak::Value(23), testdata::to_attr_owned(b"23").into());
        assert_eq!(DmxBreak::Value(1), testdata::to_attr_owned(b"1").into());
        assert_eq!(DmxBreak::Value(145), testdata::to_attr_owned(b"145").into());
        assert_eq!(DmxBreak::Overwrite, testdata::to_attr_owned(b"Overwrite").into());
    }

    #[test]
    fn test_from_attr_borrowed_valid() {
        assert_eq!(DmxBreak::Value(23), testdata::to_attr_borrowed(b"23").into());
        assert_eq!(DmxBreak::Value(1), testdata::to_attr_borrowed(b"1").into());
        assert_eq!(DmxBreak::Value(145), testdata::to_attr_borrowed(b"145").into());
        assert_eq!(DmxBreak::Overwrite, testdata::to_attr_borrowed(b"Overwrite").into());
    }

    #[test]
    fn test_default() {
        assert_eq!(DmxBreak::Value(1), Default::default());
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(DmxBreak::default(), "23a".into());
        assert_eq!(DmxBreak::default(), "".into());
        assert_eq!(DmxBreak::default(), "a3".into());
        assert_eq!(DmxBreak::default(), "Overwritee".into());
    }

    #[test]
    fn test_from_attr_owned_invalid() {
        assert_eq!(DmxBreak::default(), testdata::to_attr_owned(b"23a").into());
        assert_eq!(DmxBreak::default(), testdata::to_attr_owned(b"").into());
        assert_eq!(DmxBreak::default(), testdata::to_attr_owned(b"a3").into());
        assert_eq!(DmxBreak::default(), testdata::to_attr_owned(b"Overwritee").into());
    }

    #[test]
    fn test_from_attr_borrowed_invalid() {
        assert_eq!(DmxBreak::default(), testdata::to_attr_borrowed(b"23a").into());
        assert_eq!(DmxBreak::default(), testdata::to_attr_borrowed(b"").into());
        assert_eq!(DmxBreak::default(), testdata::to_attr_borrowed(b"a3").into());
        assert_eq!(DmxBreak::default(), testdata::to_attr_borrowed(b"Overwritee").into());
    }
}