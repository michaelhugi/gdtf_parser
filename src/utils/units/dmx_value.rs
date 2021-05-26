//! Module for the unit DMXValue used in GDTF
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{FromStr, Utf8Error};

use quick_xml::events::attributes::Attribute;

///DMXValue used in GDTF
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct DMXValue {
    ///The initial value without byte shift
    pub initial_value: u32,
    ///Byte shift count
    pub n: u8,
    ///If it is not byte_shifting it is byte_mirroring
    pub is_byte_shifting: bool,
}

///Parses a str in format Uint/n or Uint/ns to DMXValue
impl TryFrom<&str> for DMXValue {
    type Error = GDTFDmxValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (value, is_byte_shifting) = if value.ends_with('s') {
            (&value[..value.len() - 1], true)
        } else {
            (value, false)
        };
        let value: Vec<&str> = value.split("/").collect();
        if value.len() != 2 { return Err(GDTFDmxValueError {}); }

        Ok(
            DMXValue {
                initial_value: u32::from_str(value[0]).or_else(|_| { return Err(GDTFDmxValueError {}); })?,
                n: u8::from_str(value[1]).or_else(|_| { return Err(GDTFDmxValueError {}); })?,
                is_byte_shifting,
            }
        )
    }
}

///Parses an xml attribute in format Uint/n or Uint/ns to DMXValue
impl TryFrom<Attribute<'_>> for DMXValue {
    type Error = GDTFDmxValueError;

    fn try_from(attr: Attribute<'_>) -> Result<Self, Self::Error> {
        Ok(std::str::from_utf8(attr.value.borrow())?.try_into()?)
    }
}

#[derive(Debug)]
/// Error that occures if the format of DmxValue is wrong e.q. not Uint/n or Uint/ns
pub struct GDTFDmxValueError {}

impl Display for GDTFDmxValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The DMXValue must be formatted Uint/n or Uint/ns")
    }
}

impl Error for GDTFDmxValueError {}

impl From<Utf8Error> for GDTFDmxValueError {
    fn from(_: Utf8Error) -> Self {
        GDTFDmxValueError {}
    }
}

#[cfg(test)]
impl PartialEq for DMXValue {
    fn eq(&self, other: &Self) -> bool {
        self.is_byte_shifting == other.is_byte_shifting && self.initial_value == other.initial_value && self.n == other.n
    }
}

///Displays the DMXValue in format Uint/n or Uint/ns
impl Display for DMXValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_byte_shifting {
            write!(f, "{}/{}s", self.initial_value, self.n)
        } else {
            write!(f, "{}/{}", self.initial_value, self.n)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::testdata;
    use crate::utils::units::dmx_value::DMXValue;

    #[test]
    fn test_from_str_valid() {
        assert_eq!(
            DMXValue { initial_value: 255, n: 1, is_byte_shifting: false },
            "255/1".try_into().unwrap()
        );
        assert_eq!(
            DMXValue { initial_value: 14, n: 2, is_byte_shifting: false },
            "14/2".try_into().unwrap()
        );
        assert_eq!(
            DMXValue { initial_value: 14, n: 2, is_byte_shifting: true },
            "14/2s".try_into().unwrap()
        );
        assert_eq!(
            DMXValue { initial_value: 255, n: 1, is_byte_shifting: true },
            "255/1s".try_into().unwrap()
        );
    }

    #[test]
    fn test_from_attr_owned_valid() {
        assert_eq!(
            DMXValue { initial_value: 255, n: 1, is_byte_shifting: false },
            testdata::to_attr_owned(b"255/1").try_into().unwrap()
        );
        assert_eq!(
            DMXValue { initial_value: 14, n: 2, is_byte_shifting: false },
            testdata::to_attr_owned(b"14/2").try_into().unwrap()
        );
        assert_eq!(
            DMXValue { initial_value: 14, n: 2, is_byte_shifting: true },
            testdata::to_attr_owned(b"14/2s").try_into().unwrap()
        );
        assert_eq!(
            DMXValue { initial_value: 255, n: 1, is_byte_shifting: true },
            testdata::to_attr_owned(b"255/1s").try_into().unwrap()
        );
    }

    #[test]
    fn test_from_attr_borrowed_valid() {
        assert_eq!(
            DMXValue { initial_value: 255, n: 1, is_byte_shifting: false },
            testdata::to_attr_borrowed(b"255/1").try_into().unwrap()
        );
        assert_eq!(
            DMXValue { initial_value: 14, n: 2, is_byte_shifting: false },
            testdata::to_attr_borrowed(b"14/2").try_into().unwrap()
        );
        assert_eq!(
            DMXValue { initial_value: 14, n: 2, is_byte_shifting: true },
            testdata::to_attr_borrowed(b"14/2s").try_into().unwrap()
        );
        assert_eq!(
            DMXValue { initial_value: 255, n: 1, is_byte_shifting: true },
            testdata::to_attr_borrowed(b"255/1s").try_into().unwrap()
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(DMXValue::try_from("something invalid").is_err());
        assert!(DMXValue::try_from("12").is_err());
        assert!(DMXValue::try_from("2").is_err());
        assert!(DMXValue::try_from("12s").is_err());
        assert!(DMXValue::try_from("-1/3s").is_err());
        assert!(DMXValue::try_from("-1/3").is_err());
        assert!(DMXValue::try_from("-1/-3s").is_err());
        assert!(DMXValue::try_from("-1/-3").is_err());
        assert!(DMXValue::try_from("1/-3s").is_err());
        assert!(DMXValue::try_from("1/-3").is_err());
    }

    #[test]
    fn test_from_attr_owned_invalid() {
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"something invalid")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"12")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"2")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"12s")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"-1/3")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"-1/3s")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"-1/-3")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"-1/-3s")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"1/-3")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_owned(b"1/-3s")).is_err());
    }

    #[test]
    fn test_from_attr_borrowed_invalid() {
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"something invalid")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"12")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"2")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"12s")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"-1/3")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"-1/3s")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"-1/-3")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"-1/-3s")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"1/-3")).is_err());
        assert!(DMXValue::try_from(testdata::to_attr_borrowed(b"1/-3s")).is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DMXValue { initial_value: 23, n: 2, is_byte_shifting: true }), "23/2s");
        assert_eq!(format!("{}", DMXValue { initial_value: 23, n: 2, is_byte_shifting: false }), "23/2");
    }

    #[test]
    fn test_eq() {
        assert_eq!(DMXValue { initial_value: 1, n: 2, is_byte_shifting: true },
                   DMXValue { initial_value: 1, n: 2, is_byte_shifting: true });
        assert_ne!(DMXValue { initial_value: 1, n: 2, is_byte_shifting: true },
                   DMXValue { initial_value: 3, n: 2, is_byte_shifting: true });
        assert_ne!(DMXValue { initial_value: 1, n: 2, is_byte_shifting: true },
                   DMXValue { initial_value: 1, n: 3, is_byte_shifting: true });
        assert_ne!(DMXValue { initial_value: 1, n: 2, is_byte_shifting: true },
                   DMXValue { initial_value: 1, n: 2, is_byte_shifting: false });
    }
}