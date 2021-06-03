//TODO check
//! Module for the unit DMXValue used in GDTF
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{FromStr, Utf8Error};

use quick_xml::events::attributes::Attribute;

///DMXValue used in GDTF
#[derive(Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
pub struct DmxValue {
    ///The initial value without byte shift
    pub initial_value: u32,
    ///Byte shift count
    pub n: u8,
    ///If it is not byte_shifting it is byte_mirroring
    pub is_byte_shifting: bool,
}

///Parses a str in format Uint/n or Uint/ns to DMXValue
impl TryFrom<&str> for DmxValue {
    type Error = GdtfDmxValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (value, is_byte_shifting) = if value.strip_suffix('s').is_some() {
            (&value[..value.len() - 1], true)
        } else {
            (value, false)
        };
        let value: Vec<&str> = value.split('/').collect();
        if value.len() != 2 { return Err(GdtfDmxValueError {}); }

        Ok(
            DmxValue {
                initial_value: u32::from_str(value[0]).map_err(|_| GdtfDmxValueError {})?,
                n: u8::from_str(value[1]).map_err(|_| GdtfDmxValueError {})?,
                is_byte_shifting,
            }
        )
    }
}

///Parses an xml attribute in format Uint/n or Uint/ns to DMXValue
impl TryFrom<Attribute<'_>> for DmxValue {
    type Error = GdtfDmxValueError;

    fn try_from(attr: Attribute<'_>) -> Result<Self, Self::Error> {
        std::str::from_utf8(attr.value.borrow())?.try_into()
    }
}

#[derive(Debug)]
/// Error that occures if the format of DmxValue is wrong e.q. not Uint/n or Uint/ns
pub struct GdtfDmxValueError {}

impl Display for GdtfDmxValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The DMXValue must be formatted Uint/n or Uint/ns")
    }
}

impl Error for GdtfDmxValueError {}

impl From<Utf8Error> for GdtfDmxValueError {
    fn from(_: Utf8Error) -> Self {
        GdtfDmxValueError {}
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::testdata;
    use crate::utils::units::dmx_value::DmxValue;

    #[test]
    fn test_from_str_valid() {
        assert_eq!(
            DmxValue { initial_value: 255, n: 1, is_byte_shifting: false },
            "255/1".try_into().unwrap()
        );
        assert_eq!(
            DmxValue { initial_value: 14, n: 2, is_byte_shifting: false },
            "14/2".try_into().unwrap()
        );
        assert_eq!(
            DmxValue { initial_value: 14, n: 2, is_byte_shifting: true },
            "14/2s".try_into().unwrap()
        );
        assert_eq!(
            DmxValue { initial_value: 255, n: 1, is_byte_shifting: true },
            "255/1s".try_into().unwrap()
        );
    }

    #[test]
    fn test_from_attr_owned_valid() {
        assert_eq!(
            DmxValue { initial_value: 255, n: 1, is_byte_shifting: false },
            testdata::to_attr_owned(b"255/1").try_into().unwrap()
        );
        assert_eq!(
            DmxValue { initial_value: 14, n: 2, is_byte_shifting: false },
            testdata::to_attr_owned(b"14/2").try_into().unwrap()
        );
        assert_eq!(
            DmxValue { initial_value: 14, n: 2, is_byte_shifting: true },
            testdata::to_attr_owned(b"14/2s").try_into().unwrap()
        );
        assert_eq!(
            DmxValue { initial_value: 255, n: 1, is_byte_shifting: true },
            testdata::to_attr_owned(b"255/1s").try_into().unwrap()
        );
    }

    #[test]
    fn test_from_attr_borrowed_valid() {
        assert_eq!(
            DmxValue { initial_value: 255, n: 1, is_byte_shifting: false },
            testdata::to_attr_borrowed(b"255/1").try_into().unwrap()
        );
        assert_eq!(
            DmxValue { initial_value: 14, n: 2, is_byte_shifting: false },
            testdata::to_attr_borrowed(b"14/2").try_into().unwrap()
        );
        assert_eq!(
            DmxValue { initial_value: 14, n: 2, is_byte_shifting: true },
            testdata::to_attr_borrowed(b"14/2s").try_into().unwrap()
        );
        assert_eq!(
            DmxValue { initial_value: 255, n: 1, is_byte_shifting: true },
            testdata::to_attr_borrowed(b"255/1s").try_into().unwrap()
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(DmxValue::try_from("something invalid").is_err());
        assert!(DmxValue::try_from("12").is_err());
        assert!(DmxValue::try_from("2").is_err());
        assert!(DmxValue::try_from("12s").is_err());
        assert!(DmxValue::try_from("-1/3s").is_err());
        assert!(DmxValue::try_from("-1/3").is_err());
        assert!(DmxValue::try_from("-1/-3s").is_err());
        assert!(DmxValue::try_from("-1/-3").is_err());
        assert!(DmxValue::try_from("1/-3s").is_err());
        assert!(DmxValue::try_from("1/-3").is_err());
    }

    #[test]
    fn test_from_attr_owned_invalid() {
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"something invalid")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"12")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"2")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"12s")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"-1/3")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"-1/3s")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"-1/-3")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"-1/-3s")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"1/-3")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_owned(b"1/-3s")).is_err());
    }

    #[test]
    fn test_from_attr_borrowed_invalid() {
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"something invalid")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"12")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"2")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"12s")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"-1/3")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"-1/3s")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"-1/-3")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"-1/-3s")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"1/-3")).is_err());
        assert!(DmxValue::try_from(testdata::to_attr_borrowed(b"1/-3s")).is_err());
    }

    #[test]
    fn test_eq() {
        assert_eq!(DmxValue { initial_value: 1, n: 2, is_byte_shifting: true },
                   DmxValue { initial_value: 1, n: 2, is_byte_shifting: true });
        assert_ne!(DmxValue { initial_value: 1, n: 2, is_byte_shifting: true },
                   DmxValue { initial_value: 3, n: 2, is_byte_shifting: true });
        assert_ne!(DmxValue { initial_value: 1, n: 2, is_byte_shifting: true },
                   DmxValue { initial_value: 1, n: 3, is_byte_shifting: true });
        assert_ne!(DmxValue { initial_value: 1, n: 2, is_byte_shifting: true },
                   DmxValue { initial_value: 1, n: 2, is_byte_shifting: false });
    }
}