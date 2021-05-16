//! Module for the unit ColorCIE used in GDTF
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::utils::errors::GdtfError;

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

impl TryFrom<&str> for ColorCIE {
    type Error = GdtfError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: Vec<&str> = value.split(",").collect();
        if value.len() != 3 {
            return Err(GdtfError::ColorCIENotValidError("The ColorCIE must contain three comma separated values".to_string()));
        }
        Ok(
            ColorCIE {
                x: f32::from_str(value[0]).or_else(|_| { return Err(GdtfError::ColorCIENotValidError("First argument of ColorCIE not valid".to_string())); })?,
                y: f32::from_str(value[1]).or_else(|_| { return Err(GdtfError::ColorCIENotValidError("Second argument of ColorCIE not valid".to_string())); })?,
                Y: f32::from_str(value[2]).or_else(|_| { return Err(GdtfError::ColorCIENotValidError("Third argument of ColorCIE not valid".to_string())); })?,
            }
        )
    }
}

#[cfg(test)]
impl PartialEq for ColorCIE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.Y == other.Y
    }
}

impl Display for ColorCIE {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.Y)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::color_cie::ColorCIE;

    #[test]
    fn test_valid() {
        assert_eq!(
            ColorCIE { x: 234.2, y: 123.123, Y: 123. },
            ColorCIE::try_from("234.2,123.123,123.000").unwrap()
        );
    }

    #[test]
    fn test_invalid() {
        match ColorCIE::try_from("something invalid") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ColorCIE { x: 234.2, y: 123.123, Y: 123. }), "234.2,123.123,123");
    }
}