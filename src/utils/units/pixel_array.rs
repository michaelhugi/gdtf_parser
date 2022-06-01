//! Module for the unit Pixel used in GDTF

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

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
}

#[derive(Debug)]
pub struct GdtfPixelArrayError {}

impl Display for GdtfPixelArrayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pixel must be formatted as two comma separated unsigned integers"
        )
    }
}

impl Error for GdtfPixelArrayError {}

impl From<GdtfPixelError> for GdtfPixelArrayError {
    fn from(_: GdtfPixelError) -> Self {
        Self {}
    }
}

impl<'de> Deserialize<'de> for PixelArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(PixelArrayVisitor)
    }
}

struct PixelArrayVisitor;

impl<'de> Visitor<'de> for PixelArrayVisitor {
    type Value = PixelArray;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("valid PixelArray String")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        match PixelArray::new_from_str(v) {
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
    use crate::utils::units::pixel::Pixel;
    use crate::utils::units::pixel_array::PixelArray;

    #[test]
    pub fn test_new_from_str() {
        assert_eq!(
            PixelArray::new_from_str("0,12").unwrap(),
            PixelArray(Pixel(0.0), Pixel(12.0))
        );
        assert_eq!(
            PixelArray::new_from_str("12,13.000001").unwrap(),
            PixelArray(Pixel(12.0), Pixel(13.000001))
        );
        assert_eq!(
            PixelArray::new_from_str("-1,16").unwrap(),
            PixelArray(Pixel(-1.0), Pixel(16.0))
        );
        assert!(PixelArray::new_from_str("12").is_err());
        assert!(PixelArray::new_from_str("Something else").is_err());
    }
}
