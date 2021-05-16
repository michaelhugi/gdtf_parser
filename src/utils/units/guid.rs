//! Module for the unit GUID used in GDTF
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::utils::errors::GdtfError;

///GUID representation used in GDTF
#[derive(Debug)]
pub struct GUID {
    ///The string value of the GUID
    value: String,
}

impl GUID {
    pub fn new() -> GUID {
        GUID {
            value: String::new()
        }
    }
    pub fn is_empty(&self) -> bool {
        self.value == ""
    }
}


impl TryFrom<&str> for GUID {
    type Error = GdtfError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(GUID { value: String::from(s) })
    }
}

impl PartialEq for GUID {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Display for GUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::guid::GUID;

    #[test]
    fn test_valid() {
        assert_eq!(
            GUID { value: "308EA87D-7164-42DE-8106-A6D273F57A51".to_string() },
            GUID::try_from("308EA87D-7164-42DE-8106-A6D273F57A51").unwrap()
        );
    }

    #[test]
    fn test_invalid_2() {
        assert_eq!(
            GUID { value: "something invalid".to_string() },
            GUID::try_from("something invalid").unwrap()
        );
    }

    #[test]
    fn test_invalid_3() {
        assert_eq!(
            GUID { value: "".to_string() },
            GUID::try_from("").unwrap()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", GUID { value: "308EA87D-7164-42DE-8106-A6D273F57A51".to_string() }), "308EA87D-7164-42DE-8106-A6D273F57A51");
    }
}