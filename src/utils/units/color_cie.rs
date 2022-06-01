//! Module for the unit ColorCIE used in GDTF
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

///CIE color representation xyY 1931 used in GDTF
#[derive(Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
pub struct ColorCie {
    ///x for color representation xyY 1931
    pub x: f32,
    ///y for color representation xyY 1931
    pub y: f32,
    ///Y for color representation xyY 1931
    pub Y: f32,
}

impl ColorCie {
    ///Parses a string defined in gdtf-xml-description to ColorCie
    /// ```rust
    /// use gdtf_parser::utils::units::color_cie::ColorCie;
    /// assert_eq!(ColorCie::new_from_str("1.2,3.5,8.2").unwrap(), ColorCie{ x: 1.2, y: 3.5, Y: 8.2});
    /// assert!(ColorCie::new_from_str("Something invalid").is_err());
    /// ```
    pub fn new_from_str(value: &str) -> Result<Self, GdtfColorCieError> {
        use GdtfColorCieError::*;
        let value: Vec<&str> = value.split(',').collect();
        if value.len() != 3 {
            return Err(MalformedError);
        }
        Ok(ColorCie {
            x: f32::from_str(value[0]).map_err(|_| MalformedError)?,
            y: f32::from_str(value[1]).map_err(|_| MalformedError)?,
            Y: f32::from_str(value[2]).map_err(|_| MalformedError)?,
        })
    }
}

///ColorCIE representation of white (often used as default)
pub const COLOR_CIE_WHITE: ColorCie = ColorCie {
    x: 0.3127,
    y: 0.329,
    Y: 100.0,
};

#[derive(Debug)]
///Error when ColorCIE could not be parsed
pub enum GdtfColorCieError {
    ///Error when passed argument was not UTF8
    Utf8Error(std::str::Utf8Error),
    //Error when format of string was not x,y,Y
    MalformedError,
}

impl From<std::str::Utf8Error> for GdtfColorCieError {
    fn from(e: std::str::Utf8Error) -> Self {
        GdtfColorCieError::Utf8Error(e)
    }
}

impl fmt::Display for GdtfColorCieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GdtfColorCieError::*;
        match self {
            Utf8Error(_) => write!(f, "ColorCIE Error. Utf8 Error"),
            MalformedError => write!(f, "ColorCIE must be formatted floatx, floaty, floatY"),
        }
    }
}

impl Error for GdtfColorCieError {}


impl<'de> Deserialize<'de> for ColorCie {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(ColorCieVisitor)
    }
}


struct ColorCieVisitor;

impl<'de> Visitor<'de> for ColorCieVisitor {
    type Value = ColorCie;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("valid ColorCIE String")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        match ColorCie::new_from_str(v) {
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
    use crate::utils::errors::GdtfError;
    use crate::utils::units::color_cie::{COLOR_CIE_WHITE, ColorCie};

    #[test]
    fn test_new_from_str() -> Result<(), GdtfError> {
        assert_eq!(
            ColorCie {
                x: 234.2,
                y: 123.123,
                Y: 123.,
            },
            ColorCie::new_from_str("234.2,123.123,123.000")?
        );
        assert_eq!(
            ColorCie {
                x: 234.2,
                y: 0.329003,
                Y: 123.,
            },
            ColorCie::new_from_str("234.2,0.329003,123.000")?
        );
        assert!(ColorCie::new_from_str("something invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_white() {
        assert_eq!(
            COLOR_CIE_WHITE,
            ColorCie {
                x: 0.3127,
                y: 0.3290,
                Y: 100.0,
            }
        );
    }
}
