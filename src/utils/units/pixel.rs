//! Module for the unit Pixel used in GDTF

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;

use crate::utils::read;

///Integer value representing one Pixel inside a MediaFile. Pixel count starts with zero in the top left corner.
#[derive(Debug, PartialEq)]
pub struct Pixel(pub u32);

impl Pixel {
    /// Parses a string defined in xml-gdtf-description to Pixel
    /// ```rust
    /// use gdtf_parser::utils::units::pixel::Pixel;
    ///
    /// assert_eq!(Pixel::new_from_str("0").unwrap(), Pixel(0));
    /// assert_eq!(Pixel::new_from_str("12").unwrap(), Pixel(12));
    /// assert!(Pixel::new_from_str("-1").is_err());
    /// assert!(Pixel::new_from_str("Something else").is_err());
    /// ```
    pub fn new_from_str(s: &str) -> Result<Self, GdtfPixelError> {
        Ok(Self(u32::from_str(s)?))
    }

    /// Parses a quick-xml-attribute defined in xml-gdtf-description to Pixel
    /// ```rust
    /// use gdtf_parser::utils::units::pixel::Pixel;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    ///
    /// assert_eq!(Pixel::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"0")}).unwrap(), Pixel(0));
    /// assert_eq!(Pixel::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"12")}).unwrap(), Pixel(12));
    /// assert!(Pixel::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"-1")}).is_err());
    /// assert!(Pixel::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Something else")}).is_err());
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Result<Self, GdtfPixelError> {
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

#[derive(Debug)]
pub struct GdtfPixelError {}

impl Display for GdtfPixelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Pixel must be formatted as unsigned integer")
    }
}

impl Error for GdtfPixelError {}

impl From<ParseIntError> for GdtfPixelError {
    fn from(_: ParseIntError) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::pixel::Pixel;

    #[test]
    pub fn test_new_from_str() {
        assert_eq!(Pixel::new_from_str("0").unwrap(), Pixel(0));
        assert_eq!(Pixel::new_from_str("12").unwrap(), Pixel(12));
        assert!(Pixel::new_from_str("-1").is_err());
        assert!(Pixel::new_from_str("Something else").is_err());
    }

    #[test]
    pub fn test_new_from_attr_owned() {
        assert_eq!(Pixel::new_from_attr(testdata::to_attr_owned(b"0")).unwrap(), Pixel(0));
        assert_eq!(Pixel::new_from_attr(testdata::to_attr_owned(b"12")).unwrap(), Pixel(12));
        assert!(Pixel::new_from_attr(testdata::to_attr_owned(b"-1")).is_err());
        assert!(Pixel::new_from_attr(testdata::to_attr_owned(b"Something else")).is_err());
    }

    #[test]
    pub fn test_new_from_attr_borrowed() {
        assert_eq!(Pixel::new_from_attr(testdata::to_attr_borrowed(b"0")).unwrap(), Pixel(0));
        assert_eq!(Pixel::new_from_attr(testdata::to_attr_borrowed(b"12")).unwrap(), Pixel(12));
        assert!(Pixel::new_from_attr(testdata::to_attr_borrowed(b"-1")).is_err());
        assert!(Pixel::new_from_attr(testdata::to_attr_borrowed(b"Something else")).is_err());
    }
}