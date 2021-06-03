//TODO check
//! Module for the unit Highlight used for DMXChannel used in GDTF


use std::borrow::Borrow;
use std::convert::TryFrom;

use quick_xml::events::attributes::Attribute;

use crate::utils::units::dmx_value::DmxValue;

///Highlight used for DMXChannel in GDTF
#[derive(Debug, PartialEq, Clone)]
pub enum Highlight {
    ///Highlight value for current channel;
    Value(DmxValue),
    ///means that this number will be overwritten by Geometry Reference
    None,
}

///Default value for Hightlight is none
impl Default for Highlight {
    fn default() -> Self {
        Highlight::None
    }
}

///Parses an xml attribute into highlight. Returns default if an error occures
impl From<Attribute<'_>> for Highlight {
    fn from(attr: Attribute<'_>) -> Self {
        match std::str::from_utf8(attr.value.borrow()) {
            Ok(v) => match Self::try_from(v) {
                Ok(highlight) => highlight,
                Err(_) => Default::default()
            }
            Err(_) => Default::default()
        }
    }
}

///Parses a &str into highlight. Returns default if an error occures
impl From<&str> for Highlight {
    fn from(s: &str) -> Self {
        use Highlight::*;
        if s == "None" {
            return None;
        }
        match DmxValue::try_from(s) {
            Ok(s) => Highlight::Value(s),
            Err(_) => Highlight::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::dmx_value::DmxValue;
    use crate::utils::units::highlight::Highlight;

    #[test]
    fn test_from_str_valid() {
        assert_eq!(
            Highlight::Value(DmxValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: false,
            }),
            "12/3".into()
        );
        assert_eq!(
            Highlight::Value(DmxValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: true,
            }),
            "12/3s".into()
        );
        assert_eq!(
            Highlight::None,
            "None".into()
        );
    }

    #[test]
    fn test_from_attr_owned_valid() {
        assert_eq!(
            Highlight::Value(DmxValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: false,
            }),
            testdata::to_attr_owned(b"12/3").into()
        );
        assert_eq!(
            Highlight::Value(DmxValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: true,
            }),
            testdata::to_attr_owned(b"12/3s").into()
        );
        assert_eq!(
            Highlight::None,
            testdata::to_attr_owned(b"None").into()
        );
    }

    #[test]
    fn test_from_attr_borrowed_valid() {
        assert_eq!(
            Highlight::Value(DmxValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: false,
            }),
            testdata::to_attr_borrowed(b"12/3").into()
        );
        assert_eq!(
            Highlight::Value(DmxValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: true,
            }),
            testdata::to_attr_borrowed(b"12/3s").into()
        );
        assert_eq!(
            Highlight::None,
            testdata::to_attr_borrowed(b"None").into()
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Highlight::None, Default::default());
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(
            Highlight::default(),
            "/12".into()
        );
        assert_eq!(
            Highlight::default(),
            "something else".into()
        );
    }

    #[test]
    fn test_from_attr_owned_invalid() {
        assert_eq!(
            Highlight::default(),
            testdata::to_attr_owned(b"/12").into()
        );
        assert_eq!(
            Highlight::default(),
            testdata::to_attr_owned(b"something else").into()
        );
    }

    #[test]
    fn test_from_attr_borrowed_invalid() {
        assert_eq!(
            Highlight::default(),
            testdata::to_attr_borrowed(b"/12").into()
        );
        assert_eq!(
            Highlight::default(),
            testdata::to_attr_borrowed(b"something else").into()
        );
    }
}