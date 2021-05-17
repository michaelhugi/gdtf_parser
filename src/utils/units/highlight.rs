//! Module for the unit Highlight used for DMXChannel used in GDTF


use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::utils::units::dmx_value::DMXValue;

///Highlight used for DMXChannel in GDTF
#[derive(Debug)]
pub enum Highlight {
    ///Highlight value for current channel;
    Value(DMXValue),
    ///means that this number will be overwritten by Geometry Reference
    None,
}

impl Default for Highlight {
    fn default() -> Self {
        Highlight::None
    }
}

impl From<&str> for Highlight {
    fn from(s: &str) -> Self {
        use Highlight::*;
        if s == "None" {
            return None;
        }
        match DMXValue::try_from(s) {
            Ok(s) => Highlight::Value(s),
            Err(_) => Highlight::default()
        }
    }
}

#[cfg(test)]
impl PartialEq for Highlight {
    fn eq(&self, other: &Self) -> bool {
        use Highlight::*;
        match self {
            Value(one) => match other {
                Value(two) => one == two,
                _ => false
            },
            None => match other {
                None => true,
                _ => false
            }
        }
    }
}

impl Display for Highlight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Highlight::*;
        match self {
            Value(x) => write!(f, "{}", x),
            None => write!(f, "None")
        }
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::dmx_value::DMXValue;
    use crate::utils::units::highlight::Highlight;

    #[test]
    fn test_valid() {
        assert_eq!(
            Highlight::Value(DMXValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: false,
            }),
            Highlight::try_from("12/3").unwrap()
        );
    }

    #[test]
    fn test_valid_2() {
        assert_eq!(
            Highlight::Value(DMXValue {
                initial_value: 12,
                n: 3,
                is_byte_shifting: true,
            }),
            Highlight::try_from("12/3s").unwrap()
        );
    }

    #[test]
    fn test_valid_3() {
        assert_eq!(
            Highlight::None,
            Highlight::try_from("None").unwrap()
        );
    }


    #[test]
    fn test_invalid() {
        assert_eq!(
            Highlight::None,
            Highlight::try_from("/12").unwrap()
        );
    }


    #[test]
    fn test_invalid_2() {
        assert_eq!(
            Highlight::None,
            Highlight::try_from("/12s").unwrap()
        );
    }

    #[test]
    fn test_invalid_3() {
        assert_eq!(
            Highlight::None,
            Highlight::try_from("2/s").unwrap()
        );
    }

    #[test]
    fn test_invalid_4() {
        assert_eq!(
            Highlight::None,
            Highlight::try_from("2/").unwrap()
        );
    }

    #[test]
    fn test_invalid_5() {
        assert_eq!(
            Highlight::None,
            Highlight::try_from("/").unwrap()
        );
    }

    #[test]
    fn test_invalid_6() {
        assert_eq!(
            Highlight::None,
            Highlight::try_from("something else").unwrap()
        );
    }

    #[test]
    fn test_display_2() {
        assert_eq!(format!("{}", Highlight::Value(DMXValue {
            initial_value: 12,
            n: 3,
            is_byte_shifting: false,
        })), "12/3");
    }

    #[test]
    fn test_display_3() {
        assert_eq!(format!("{}", Highlight::None), "None");
    }

    #[test]
    fn test_display_4() {
        assert_eq!(format!("{}", Highlight::Value(DMXValue {
            initial_value: 12,
            n: 3,
            is_byte_shifting: true,
        })), "12/3s");
    }
}