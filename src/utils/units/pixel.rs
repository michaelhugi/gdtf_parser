//! Module for the unit Pixel used in GDTF

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use std::str::FromStr;

///Integer value representing one Pixel inside a MediaFile. Pixel count starts with zero in the top left corner.
#[derive(Debug, PartialEq, Clone)]
pub struct Pixel(pub f32);

impl Pixel {
    /// Parses a string defined in xml-gdtf-description to Pixel
    /// ```rust
    /// use gdtf_parser::utils::units::pixel::Pixel;
    ///
    /// assert_eq!(Pixel::new_from_str("0").unwrap(), Pixel(0.0));
    /// assert_eq!(Pixel::new_from_str("12").unwrap(), Pixel(12.0));
    /// assert_eq!(Pixel::new_from_str("12.032120").unwrap(), Pixel(12.032120));
    /// assert_eq!(Pixel::new_from_str("-1").unwrap(), Pixel(-1.0));
    /// assert!(Pixel::new_from_str("Something else").is_err());
    /// ```
    pub fn new_from_str(s: &str) -> Result<Self, GdtfPixelError> {
        Ok(Self(f32::from_str(s.trim())?))
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

impl From<ParseFloatError> for GdtfPixelError {
    fn from(_: ParseFloatError) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::units::pixel::Pixel;

    #[test]
    pub fn test_new_from_str() {
        assert_eq!(Pixel::new_from_str("0").unwrap(), Pixel(0.0));
        assert_eq!(Pixel::new_from_str("12").unwrap(), Pixel(12.0));
        assert_eq!(Pixel::new_from_str("12.032120").unwrap(), Pixel(12.032120));
        assert_eq!(Pixel::new_from_str("-1").unwrap(), Pixel(-1.0));
        assert!(Pixel::new_from_str("Something else").is_err());
    }
}
