//! Module for the unit ColorCIE used in GDTF
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;

///CIE color representation xyY 1931 used in GDTF
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ColorCIE {
    ///x for color representation xyY 1931
    pub x: f32,
    ///y for color representation xyY 1931
    pub y: f32,
    ///Y for color representation xyY 1931
    pub Y: f32,
}

///Parses an attribute directly into ColorCIE
impl TryFrom<Attribute<'_>> for ColorCIE {
    type Error = GDTFColorCIEError;
    fn try_from(attr: Attribute<'_>) -> Result<Self, Self::Error> {
        Ok(std::str::from_utf8(attr.value.borrow())?.try_into()?)
    }
}

///Parses a str from GDTF xml to ColorCIE
impl TryFrom<&str> for ColorCIE {
    type Error = GDTFColorCIEError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use GDTFColorCIEError::*;
        let value: Vec<&str> = value.split(",").collect();
        if value.len() != 3 {
            return Err(MalformatError);
        }
        Ok(
            ColorCIE {
                x: f32::from_str(value[0]).or_else(|_| { return Err(MalformatError); })?,
                y: f32::from_str(value[1]).or_else(|_| { return Err(MalformatError); })?,
                Y: f32::from_str(value[2]).or_else(|_| { return Err(MalformatError); })?,
            }
        )
    }
}

#[cfg(test)]
impl PartialEqAllowEmpty for ColorCIE {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        if self.x != other.x || self.y != other.y || self.Y != other.Y {
            return Self::print_structs_not_equal(self, other, log);
        }
        true
    }
}

#[derive(Debug)]
///Error when ColorCIE could not be parsed
pub enum GDTFColorCIEError {
    ///Error when passed argument was not UTF8
    Utf8Error(std::str::Utf8Error),
    //Error when format of string was not x,y,Y
    MalformatError,
}

impl From<std::str::Utf8Error> for GDTFColorCIEError {
    fn from(e: std::str::Utf8Error) -> Self {
        GDTFColorCIEError::Utf8Error(e)
    }
}

impl fmt::Display for GDTFColorCIEError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GDTFColorCIEError::*;
        match self {
            Utf8Error(_) => write!(f, "ColorCIE Error. Utf8 Error"),
            MalformatError => write!(f, "ColorCIE must be formatted floatx, floaty, floatY")
        }
    }
}

impl Error for GDTFColorCIEError {}


#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::color_cie::ColorCIE;

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        ColorCIE { x: 234.2, y: 123.123, Y: 123. }.assert_eq_allow_empty(&ColorCIE::try_from("234.2,123.123,123.000")?, true);
        assert!(ColorCIE::try_from("something invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_try_from_attr_borrowed() -> Result<(), GdtfError> {
        ColorCIE { x: 234.2, y: 123.123, Y: 123. }.assert_eq_allow_empty(&testdata::to_attr_borrowed(b"234.2,123.123,123.000").try_into()?, true);
        assert!(ColorCIE::try_from(testdata::to_attr_borrowed(b"Something invalid")).is_err());
        Ok(())
    }

    #[test]
    fn test_try_from_attr_owned() -> Result<(), GdtfError> {
        ColorCIE { x: 234.2, y: 123.123, Y: 123. }.assert_eq_allow_empty(&testdata::to_attr_owned(b"234.2,123.123,123.000").try_into()?, true);
        assert!(ColorCIE::try_from(testdata::to_attr_owned(b"Something invalid")).is_err());
        Ok(())
    }


    #[test]
    #[allow(non_snake_case)]
    fn test_partial_eq_allow_empty() -> Result<(), GdtfError> {
        assert!(!ColorCIE::try_from("1.0,1.0,1.0")?.is_eq_allow_empty(&ColorCIE::try_from("2.0,1.0,1.0")?, false));
        assert!(!ColorCIE::try_from("1.0,1.0,1.0")?.is_eq_allow_empty(&ColorCIE::try_from("1.0,2.0,1.0")?, false));
        assert!(!ColorCIE::try_from("1.0,1.0,1.0")?.is_eq_allow_empty(&ColorCIE::try_from("1.0,1.0,2.0")?, false));
        assert!(ColorCIE::try_from("1.0,1.0,1.0")?.is_eq_allow_empty(&ColorCIE::try_from("1.0,1.0,1.0")?, true));
        Ok(())
    }
}