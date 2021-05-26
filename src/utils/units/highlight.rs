//! Module for the unit Highlight used for DMXChannel used in GDTF


use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use quick_xml::events::attributes::Attribute;

use crate::utils::units::dmx_value::DMXValue;

///Highlight used for DMXChannel in GDTF
#[derive(Debug)]
pub enum Highlight {
    ///Highlight value for current channel;
    Value(DMXValue),
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
        match DMXValue::try_from(s) {
            Ok(s) => Highlight::Value(s),
            Err(_) => Highlight::default()
        }
    }
}

#[cfg(test)]
impl PartialEq for Highlight {
    fn eq(&self, other: &Self) -> bool {
        use Highlight::*;
        match self {
            Value(one) => match other {
                Value(two) => one == two,
                _ => false
            },
            None => match other {
                None => true,
                _ => false
            }
        }
    }
}

///Displays the highlight value or None
impl Display for Highlight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Highlight::*;
        match self {
            Value(x) => write!(f, "{}", x),
            None => write!(f, "None")
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::dmx_value::DMXValue;
    use crate::utils::units::highlight::Highlight;

    #[test]
    fn test_from_str_valid() {
        assert_eq!(
            Highlight::Value(DMXValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: false,
            }),
            "12/3".into()
        );
        assert_eq!(
            Highlight::Value(DMXValue {
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
            Highlight::Value(DMXValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: false,
            }),
            testdata::to_attr_owned(b"12/3").into()
        );
        assert_eq!(
            Highlight::Value(DMXValue {
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
            Highlight::Value(DMXValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: false,
            }),
            testdata::to_attr_borrowed(b"12/3").into()
        );
        assert_eq!(
            Highlight::Value(DMXValue {
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


    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Highlight::Value(DMXValue {
            initial_value: 12,
            n: 3,
            is_byte_shifting: false,
        })), "12/3");
        assert_eq!(format!("{}", Highlight::Value(DMXValue {
            initial_value: 12,
            n: 3,
            is_byte_shifting: true,
        })), "12/3s");
        assert_eq!(format!("{}", Highlight::None), "None");
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(Highlight::Value(DMXValue { initial_value: 12, n: 3, is_byte_shifting: false }),
                   Highlight::Value(DMXValue { initial_value: 12, n: 3, is_byte_shifting: false }));
        assert_eq!(Highlight::None, Highlight::None);
        assert_ne!(Highlight::Value(DMXValue { initial_value: 12, n: 3, is_byte_shifting: false }),
                   Highlight::None);
        assert_ne!(Highlight::None,
                   Highlight::Value(DMXValue { initial_value: 12, n: 3, is_byte_shifting: false }));
        assert_ne!(Highlight::Value(DMXValue { initial_value: 12, n: 3, is_byte_shifting: false }),
                   Highlight::Value(DMXValue { initial_value: 14, n: 3, is_byte_shifting: false }));
        assert_ne!(Highlight::Value(DMXValue { initial_value: 12, n: 3, is_byte_shifting: false }),
                   Highlight::Value(DMXValue { initial_value: 12, n: 1, is_byte_shifting: false }));
        assert_ne!(Highlight::Value(DMXValue { initial_value: 12, n: 3, is_byte_shifting: false }),
                   Highlight::Value(DMXValue { initial_value: 12, n: 3, is_byte_shifting: true }));
    }
}