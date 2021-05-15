use std::convert::TryFrom;
use std::str::FromStr;

use crate::errors::GdtfError;

#[derive(Debug)]
pub struct ColorCIE {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TryFrom<&str> for ColorCIE {
    type Error = GdtfError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split(",").collect();
        if split.len() != 3 {
            return Err(GdtfError::ColorCIENotValidError("The color CIE must contain three comma separated values".to_string()));
        }
        Ok(
            ColorCIE {
                x: f32::from_str(split[0]).or_else(|_| { return Err(GdtfError::ColorCIENotValidError("First argument of ColorCIE not valid".to_string())); })?,
                y: f32::from_str(split[1]).or_else(|_| { return Err(GdtfError::ColorCIENotValidError("Second argument of ColorCIE not valid".to_string())); })?,
                z: f32::from_str(split[2]).or_else(|_| { return Err(GdtfError::ColorCIENotValidError("Third argument of ColorCIE not valid".to_string())); })?,
            }
        )
    }
}

#[cfg(test)]
impl PartialEq for ColorCIE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}