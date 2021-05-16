//! Module for the unit Name used in GDTF
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::utils::errors::GdtfError;

///Name representation used in GDTF
#[derive(Debug)]
pub struct Name {
    ///The string value of the name
    name: String,
}

impl Name {
    pub fn new() -> Name {
        Name {
            name: String::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name == ""
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TryFrom<&str> for Name {
    type Error = GdtfError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Name { name: String::from(s) })
    }
}

#[cfg(test)]
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::name::Name;

    #[test]
    fn test_valid() {
        assert_eq!(
            Name { name: "Name".to_string() },
            Name::try_from("Name").unwrap()
        );
    }

    #[test]
    fn test_invalid_2() {
        assert_eq!(
            Name { name: "something invalid".to_string() },
            Name::try_from("something invalid").unwrap()
        );
    }

    #[test]
    fn test_invalid_3() {
        assert_eq!(
            Name { name: "".to_string() },
            Name::try_from("").unwrap()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Name { name: "Name".to_string() }), "Name");
    }
}