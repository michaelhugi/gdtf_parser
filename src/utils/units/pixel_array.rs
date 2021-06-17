//! Module for the unit Pixel used in GDTF

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

use crate::utils::read;
use crate::utils::units::pixel::{GdtfPixelError, Pixel};

///First Pixel is X-axis and second is Y-axis
#[derive(Debug, PartialEq, Clone)]
pub struct PixelArray(pub Pixel, pub Pixel);

impl PixelArray {
    /// Parses a string defined in xml-gdtf-description to PixelArray
    /// ```rust
    /// use gdtf_parser::utils::units::pixel::Pixel;
    /// use gdtf_parser::utils::units::pixel_array::PixelArray;
    ///
    /// assert_eq!(PixelArray::new_from_str("0,12").unwrap(), PixelArray(Pixel(0.0), Pixel(12.0)));
    /// assert_eq!(PixelArray::new_from_str("12,13.000001").unwrap(), PixelArray(Pixel(12.0), Pixel(13.000001)));
    /// assert_eq!(PixelArray::new_from_str("-1,16").unwrap(), PixelArray(Pixel(-1.0), Pixel(16.0)));
    /// assert!(PixelArray::new_from_str("12").is_err());
    /// assert!(PixelArray::new_from_str("Something else").is_err());
    /// ```
    pub fn new_from_str(s: &str) -> Result<Self, GdtfPixelArrayError> {
        let s: Vec<&str> = s.split(',').collect();
        let x = s.get(0).ok_or(GdtfPixelArrayError {})?;
        let y = s.get(1).ok_or(GdtfPixelArrayError {})?;
        Ok(Self(Pixel::new_from_str(x)?, Pixel::new_from_str(y)?))
    }

    /// Parses a quick-xml-attribute defined in xml-gdtf-description to PixelArray
    /// ```rust
    /// use gdtf_parser::utils::units::pixel::Pixel;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// use gdtf_parser::utils::units::pixel_array::PixelArray;
    ///
    /// assert_eq!(PixelArray::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"0,12")}).unwrap(), PixelArray(Pixel(0.0), Pixel(12.0)));
    /// assert_eq!(PixelArray::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"12,13.000001")}).unwrap(), PixelArray(Pixel(12.0), Pixel(13.000001)));
    /// assert_eq!(PixelArray::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"-1,16")}).unwrap(), PixelArray(Pixel(-1.0), Pixel(16.0)));
    /// assert!(PixelArray::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"-1")}).is_err());
    /// assert!(PixelArray::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"12")}).is_err());
    /// assert!(PixelArray::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Something else")}).is_err());
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Result<Self, GdtfPixelArrayError> {
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

#[derive(Debug)]
pub struct GdtfPixelArrayError {}

impl Display for GdtfPixelArrayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Pixel must be formatted as two comma separated unsigned integers")
    }
}

impl Error for GdtfPixelArrayError {}

impl From<GdtfPixelError> for GdtfPixelArrayError {
    fn from(_: GdtfPixelError) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::pixel::Pixel;
    use crate::utils::units::pixel_array::PixelArray;

    #[test]
    pub fn test_new_from_str() {
        assert_eq!(PixelArray::new_from_str("0,12").unwrap(), PixelArray(Pixel(0.0), Pixel(12.0)));
        assert_eq!(PixelArray::new_from_str("12,13.000001").unwrap(), PixelArray(Pixel(12.0), Pixel(13.000001)));
        assert_eq!(PixelArray::new_from_str("-1,16").unwrap(), PixelArray(Pixel(-1.0), Pixel(16.0)));
        assert!(PixelArray::new_from_str("12").is_err());
        assert!(PixelArray::new_from_str("Something else").is_err());
    }

    #[test]
    pub fn test_new_from_attr_owned() {
        assert_eq!(PixelArray::new_from_attr(testdata::to_attr_owned(b"0, 12")).unwrap(), PixelArray(Pixel(0.0), Pixel(12.0)));
        assert_eq!(PixelArray::new_from_attr(testdata::to_attr_owned(b"12, 13.000001")).unwrap(), PixelArray(Pixel(12.0), Pixel(13.000001)));
        assert_eq!(PixelArray::new_from_attr(testdata::to_attr_owned(b"-1,16")).unwrap(), PixelArray(Pixel(-1.0), Pixel(16.0)));
        assert!(PixelArray::new_from_attr(testdata::to_attr_owned(b"12")).is_err());
        assert!(PixelArray::new_from_attr(testdata::to_attr_owned(b"Something else")).is_err());
    }

    #[test]
    pub fn test_new_from_attr_borrowed() {
        assert_eq!(PixelArray::new_from_attr(testdata::to_attr_borrowed(b"0, 12")).unwrap(), PixelArray(Pixel(0.0), Pixel(12.0)));
        assert_eq!(PixelArray::new_from_attr(testdata::to_attr_borrowed(b"12, 13.000001")).unwrap(), PixelArray(Pixel(12.0), Pixel(13.000001)));
        assert_eq!(PixelArray::new_from_attr(testdata::to_attr_borrowed(b"-1,16")).unwrap(), PixelArray(Pixel(-1.0), Pixel(16.0)));
        assert!(PixelArray::new_from_attr(testdata::to_attr_borrowed(b"12")).is_err());
        assert!(PixelArray::new_from_attr(testdata::to_attr_borrowed(b"Something else")).is_err());
    }
}