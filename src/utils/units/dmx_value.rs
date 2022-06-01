//! Module for the unit DMXValue used in GDTF
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::{FromStr, Utf8Error};

use serde::{Deserialize, Deserializer};
use serde::de::Visitor;

///DMXValue used in GDTF
/// Special type to define DMX value where n is the byte count. The byte count can be individually specified without depending on the resolution of the DMX Channel.
/// By default byte mirroring is used for the conversion. So 255/1 in a 16 bit channel will result in 65535.
/// You can use the byte shifting operator to use byte shifting for the conversion. So 255/1s in a 16 bit channel will result in 65280.
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

impl DmxValue {
    ///Parses a string defined in gdtf-xml-description to DmxValue
    ///```rust
    /// use gdtf_parser::utils::units::dmx_value::DmxValue;
    /// assert_eq!(DmxValue::new_from_str("255/1").unwrap(), DmxValue{ initial_value: 255, n: 1, is_byte_shifting: false});
    /// assert_eq!(DmxValue::new_from_str("255/1s").unwrap(), DmxValue{ initial_value: 255, n: 1, is_byte_shifting: true});
    /// assert!(DmxValue::new_from_str("Something invalid").is_err());
    /// ```
    pub fn new_from_str(value: &str) -> Result<Self, GdtfDmxValueError> {
        let (value, is_byte_shifting) = if value.strip_suffix('s').is_some() {
            (&value[..value.len() - 1], true)
        } else {
            (value, false)
        };
        let value: Vec<&str> = value.split('/').collect();
        if value.len() != 2 {
            return Err(GdtfDmxValueError {});
        }

        Ok(DmxValue {
            initial_value: u32::from_str(value[0]).map_err(|_| GdtfDmxValueError {})?,
            n: u8::from_str(value[1]).map_err(|_| GdtfDmxValueError {})?,
            is_byte_shifting,
        })
    }
}

#[derive(Debug)]
/// Error that occures if the format of DmxValue is wrong e.q. not Uint/n or Uint/ns
pub struct GdtfDmxValueError {}

impl Display for GdtfDmxValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The DMXValue must be formatted Uint/n or Uint/ns. Current form"
        )
    }
}

impl Error for GdtfDmxValueError {}

impl From<Utf8Error> for GdtfDmxValueError {
    fn from(_: Utf8Error) -> Self {
        GdtfDmxValueError {}
    }
}

impl<'de> Deserialize<'de> for DmxValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(DmxValueVisitor)
    }
}

struct DmxValueVisitor;

impl<'de> Visitor<'de> for DmxValueVisitor {
    type Value = DmxValue;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("valid DmxValue String")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        match DmxValue::new_from_str(v) {
            Ok(item) => Ok(item),
            Err(e) => Err(serde::de::Error::custom(format!("{}", e)))
        }
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        self.visit_str(&v)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::units::dmx_value::DmxValue;

    #[test]
    fn test_new_from_str() {
        assert_eq!(
            DmxValue {
                initial_value: 255,
                n: 1,
                is_byte_shifting: false,
            },
            DmxValue::new_from_str("255/1").unwrap()
        );
        assert_eq!(
            DmxValue {
                initial_value: 14,
                n: 2,
                is_byte_shifting: false,
            },
            DmxValue::new_from_str("14/2").unwrap()
        );
        assert_eq!(
            DmxValue {
                initial_value: 14,
                n: 2,
                is_byte_shifting: true,
            },
            DmxValue::new_from_str("14/2s").unwrap()
        );
        assert_eq!(
            DmxValue {
                initial_value: 255,
                n: 1,
                is_byte_shifting: true,
            },
            DmxValue::new_from_str("255/1s").unwrap()
        );
        assert!(DmxValue::new_from_str("Something invalid").is_err());
        assert!(DmxValue::new_from_str("12").is_err());
        assert!(DmxValue::new_from_str("2").is_err());
        assert!(DmxValue::new_from_str("12s").is_err());
        assert!(DmxValue::new_from_str("-1/3s").is_err());
        assert!(DmxValue::new_from_str("-1/3").is_err());
        assert!(DmxValue::new_from_str("-1/-3s").is_err());
        assert!(DmxValue::new_from_str("-1/-3").is_err());
        assert!(DmxValue::new_from_str("1/-3s").is_err());
        assert!(DmxValue::new_from_str("1/-3").is_err());
    }
}
