//! Module for the unit DMXValue used in GDTF
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::utils::errors::GdtfError;

///DMXValue used in GDTF
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct DMXValue {
    ///The initial value without byte shift
    pub initial_value: u32,
    ///Byte shift count
    pub n: u8,
    ///If it is not byte_shifting it is byte_mirroring
    pub is_byte_shifting: bool,
}

impl DMXValue {
    pub fn new() -> Self {
        Self {
            initial_value: 0,
            n: 1,
            is_byte_shifting: false,
        }
    }
}

impl TryFrom<&str> for DMXValue {
    type Error = GdtfError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (value, is_byte_shifting) = if value.ends_with('s') {
            (&value[..value.len() - 1], true)
        } else {
            (value, false)
        };
        let value: Vec<&str> = value.split("/").collect();
        if value.len() != 2 { return Err(GdtfError::DMXValueNotValidError("The DMXValue must have format Uint/n or Uint/ns".to_string())); }


        Ok(
            DMXValue {
                initial_value: u32::from_str(value[0]).or_else(|_| { return Err(GdtfError::DMXValueNotValidError("The DMXValue must have format Uint/n or Uint/ns".to_string())); })?,
                n: u8::from_str(value[1]).or_else(|_| { return Err(GdtfError::DMXValueNotValidError("The DMXValue must have format Uint/n or Uint/ns".to_string())); })?,
                is_byte_shifting,
            }
        )
    }
}

#[cfg(test)]
impl PartialEq for DMXValue {
    fn eq(&self, other: &Self) -> bool {
        self.is_byte_shifting == other.is_byte_shifting && self.initial_value == other.initial_value && self.n == other.n
    }
}

impl Display for DMXValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_byte_shifting {
            write!(f, "{}/{}s", self.initial_value, self.n)
        } else {
            write!(f, "{}/{}", self.initial_value, self.n)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::dmx_value::DMXValue;

    #[test]
    fn test_valid() {
        assert_eq!(
            DMXValue { initial_value: 255, n: 1, is_byte_shifting: false },
            DMXValue::try_from("255/1").unwrap()
        );
    }

    #[test]
    fn test_valid_2() {
        assert_eq!(
            DMXValue { initial_value: 23, n: 2, is_byte_shifting: false },
            DMXValue::try_from("23/2").unwrap()
        );
    }

    #[test]
    fn test_valid_3() {
        DMXValue::try_from("23/2s").unwrap();
        assert_eq!(
            DMXValue { initial_value: 23, n: 2, is_byte_shifting: true },
            DMXValue::try_from("23/2s").unwrap()
        );
    }


    #[test]
    fn test_invalid() {
        match DMXValue::try_from("something invalid") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_1() {
        match DMXValue::try_from("-1/3") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_2() {
        match DMXValue::try_from("1/-3") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_3() {
        match DMXValue::try_from("-1/-3") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_4() {
        match DMXValue::try_from("-1/-3s") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_5() {
        match DMXValue::try_from("1/-3s") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_6() {
        match DMXValue::try_from("-1/3s") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DMXValue { initial_value: 23, n: 2, is_byte_shifting: true }), "23/2s");
    }

    #[test]
    fn test_display_2() {
        assert_eq!(format!("{}", DMXValue { initial_value: 23, n: 2, is_byte_shifting: false }), "23/2");
    }
}