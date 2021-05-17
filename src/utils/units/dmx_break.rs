//! Module for the unit DMXBreak used for DMXChannel used in GDTF


use std::fmt::{Display, Formatter};
use std::str::FromStr;

///DMXBreak used for DMXChannel in GDTF
#[derive(Debug)]
pub enum DMXBreak {
    ///Number of the DMXBreak; Default value: 1
    Value(u32),
    ///means that this number will be overwritten by Geometry Reference
    Overwrite,
}

impl Default for DMXBreak {
    fn default() -> Self {
        DMXBreak::Value(1)
    }
}

impl From<&str> for DMXBreak {
    fn from(s: &str) -> Self {
        use DMXBreak::*;
        if s == "Overwrite" {
            Overwrite
        } else {
            Value(u32::from_str(s).unwrap_or_else(|_| 1))
        }
    }
}

#[cfg(test)]
impl PartialEq for DMXBreak {
    fn eq(&self, other: &Self) -> bool {
        use DMXBreak::*;
        match self {
            Value(one) => match other {
                Value(two) => one == two,
                _ => false
            },
            Overwrite => match other {
                Overwrite => true,
                _ => false
            }
        }
    }
}

impl Display for DMXBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DMXBreak::*;
        match self {
            Value(x) => write!(f, "{}", x),
            Overwrite => write!(f, "Overwrite")
        }
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::dmx_break::DMXBreak;

    #[test]
    fn test_valid() {
        assert_eq!(
            DMXBreak::Value(23),
            DMXBreak::try_from("23").unwrap()
        );
    }

    #[test]
    fn test_valid_2() {
        assert_eq!(
            DMXBreak::Value(1),
            DMXBreak::try_from("1").unwrap()
        );
    }

    #[test]
    fn test_valid_3() {
        assert_eq!(
            DMXBreak::Overwrite,
            DMXBreak::try_from("Overwrite").unwrap()
        );
    }


    #[test]
    fn test_invalid() {
        assert_eq!(
            DMXBreak::default(),
            DMXBreak::try_from("Something").unwrap()
        );
    }


    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DMXBreak::Value(12)), "12");
    }

    #[test]
    fn test_display_2() {
        assert_eq!(format!("{}", DMXBreak::Value(1)), "1");
    }

    #[test]
    fn test_display_3() {
        assert_eq!(format!("{}", DMXBreak::Overwrite), "Overwrite");
    }
}