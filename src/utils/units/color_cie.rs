//! Module for the unit ColorCIE used in GDTF
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::Debug;
use std::fmt;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;

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
///Only used for testing to check if variables are same
impl PartialEq for ColorCIE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.Y == other.Y
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

    use crate::utils::testdata;
    use crate::utils::units::color_cie::ColorCIE;

    #[test]
    fn test_parse_valid_str() {
        assert_eq!(
            ColorCIE { x: 234.2, y: 123.123, Y: 123. },
            ColorCIE::try_from("234.2,123.123,123.000").unwrap()
        );
    }

    #[test]
    fn test_parse_valid_attr_borrowed() {
        assert_eq!(
            ColorCIE { x: 234.2, y: 123.123, Y: 123. },
            testdata::to_attr_borrowed(b"234.2,123.123,123.000").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_valid_attr_owned() {
        assert_eq!(
            ColorCIE { x: 234.2, y: 123.123, Y: 123. },
            testdata::to_attr_owned(b"234.2,123.123,123.000").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_invalid_str() {
        if ColorCIE::try_from("something invalid").is_ok() {
            panic!("test_invalid should return an error");
        }
    }

    #[test]
    fn test_parse_invalid_attr_owned() {
        if ColorCIE::try_from(testdata::to_attr_owned(b"Something invalid")).is_ok() {
            panic!("test_invalid should return an error");
        }
    }

    #[test]
    fn test_parse_invalid_attr_borrowed() {
        if ColorCIE::try_from(testdata::to_attr_owned(b"Something invalid")).is_ok() {
            panic!("test_invalid should return an error");
        }
    }

    #[test]
    fn test_partial_eq_x() {
        let left = ColorCIE {
            x: 1.0,
            y: 1.0,
            Y: 1.0,
        };
        let right = ColorCIE {
            x: 2.0,
            y: 1.0,
            Y: 1.0,
        };
        if left == right {
            panic!("The PartialEq of ColorCIE is broken");
        }
    }

    #[test]
    fn test_partial_eq_y() {
        let left = ColorCIE {
            x: 1.0,
            y: 2.0,
            Y: 1.0,
        };
        let right = ColorCIE {
            x: 1.0,
            y: 1.0,
            Y: 1.0,
        };
        if left == right {
            panic!("The PartialEq of ColorCIE is broken");
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_partial_eq_Y() {
        let left = ColorCIE {
            x: 1.0,
            y: 1.0,
            Y: 2.0,
        };
        let right = ColorCIE {
            x: 1.0,
            y: 1.0,
            Y: 1.0,
        };
        if left == right {
            panic!("The PartialEq of ColorCIE is broken");
        }
    }

    #[test]
    fn test_partial_eq_valid() {
        let left = ColorCIE {
            x: 0.0,
            y: 1.0,
            Y: 2.0,
        };
        let right = ColorCIE {
            x: 0.0,
            y: 1.0,
            Y: 2.0,
        };
        if left != right {
            panic!("The PartialEq of ColorCIE is broken");
        }
    }
}