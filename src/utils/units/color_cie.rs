//! Module for the unit ColorCIE used in GDTF
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;

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
    ///Creates a new ColorCIE from a &str defined by GDTF-XML.
    pub fn new_from_str(value: &str) -> Result<Self, GdtfColorCieError> {
        use GdtfColorCieError::*;
        let value: Vec<&str> = value.split(',').collect();
        if value.len() != 3 {
            return Err(MalformatError);
        }
        Ok(
            ColorCie {
                x: f32::from_str(value[0]).map_err(|_| { MalformatError })?,
                y: f32::from_str(value[1]).map_err(|_| { MalformatError })?,
                Y: f32::from_str(value[2]).map_err(|_| { MalformatError })?,
            }
        )
    }
}

///Creates a new ColorCIE from an xml-attribute defined by GDTF-XML.
impl TryFrom<Attribute<'_>> for ColorCie {
    type Error = GdtfColorCieError;
    fn try_from(attr: Attribute<'_>) -> Result<Self, Self::Error> {
        Self::new_from_str(std::str::from_utf8(attr.value.borrow())?)
    }
}

///Creates a new ColorCIE from a &str defined by GDTF-XML.
impl TryFrom<&str> for ColorCie {
    type Error = GdtfColorCieError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_from_str(value)
    }
}

#[derive(Debug)]
///Error when ColorCIE could not be parsed
pub enum GdtfColorCieError {
    ///Error when passed argument was not UTF8
    Utf8Error(std::str::Utf8Error),
    //Error when format of string was not x,y,Y
    MalformatError,
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
            MalformatError => write!(f, "ColorCIE must be formatted floatx, floaty, floatY")
        }
    }
}

impl Error for GdtfColorCieError {}


#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::color_cie::ColorCie;

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        assert_eq!(ColorCie { x: 234.2, y: 123.123, Y: 123. }, ColorCie::try_from("234.2,123.123,123.000")?);
        assert!(ColorCie::try_from("something invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_try_from_attr_borrowed() -> Result<(), GdtfError> {
        assert_eq!(ColorCie { x: 234.2, y: 123.123, Y: 123. }, testdata::to_attr_borrowed(b"234.2,123.123,123.000").try_into()?);
        assert!(ColorCie::try_from(testdata::to_attr_borrowed(b"Something invalid")).is_err());
        Ok(())
    }

    #[test]
    fn test_try_from_attr_owned() -> Result<(), GdtfError> {
        assert_eq!(ColorCie { x: 234.2, y: 123.123, Y: 123. }, testdata::to_attr_owned(b"234.2,123.123,123.000").try_into()?);
        assert!(ColorCie::try_from(testdata::to_attr_owned(b"Something invalid")).is_err());
        Ok(())
    }
}