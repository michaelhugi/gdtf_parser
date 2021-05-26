//! Module for the unit Name used in GDTF
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

///Name representation used in GDTF
#[derive(Debug)]
pub enum Name {
    ///The string value of the name
    Name(String),
    ///Variant of empty string
    Empty,
}

///Default is an empty Name
impl Default for Name {
    ///Default is an empty Name
    fn default() -> Self {
        Name::Empty
    }
}

///Writes just the string for the name
impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Name::Name(name) => write!(f, "{}", name),
            Name::Empty => write!(f, "")
        }
    }
}

///Deparses Name from Attribute safely. In case of error it will return default. It will also allow not valid chars from GDTF-Spec because Rust can handle it!
impl From<Attribute<'_>> for Name {
    ///Depares Name safely from Attribute. In case of error it returns default. It will also allow not valid chars from GDTF-Spec because Rust can handle it!
    fn from(attr: Attribute) -> Self {
        match std::str::from_utf8(attr.value.borrow()).unwrap_or_else(|_| "") {
            "" => Name::Empty,
            value => Name::Name(String::from(value))
        }
    }
}

#[derive(Debug)]
pub enum GDTFNameError {
    NotAllowedCharError(String)
}

impl Error for GDTFNameError {}

impl fmt::Display for GDTFNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GDTFNameError::*;
        match self {
            //GDTFNameError(_) => write!(f, "ColorCIE Error. Utf8 Error"),
            NotAllowedCharError(s) => write!(f, "GdtfNameError: {}", s)
        }
    }
}

impl TryFrom<&str> for Name {
    type Error = GDTFNameError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::validate_chars(s)?;
        match s {
            "" => Ok(Name::Empty),
            s => Ok(Name::Name(String::from(s)))
        }
    }
}

impl Name {
    fn validate_chars(s: &str) -> Result<(), GDTFNameError> {
        for char in s.chars() {
            let char = char as u8;
            if char < 32 || char > 122 {
                let char = [char];
                match std::str::from_utf8(&char) {
                    Ok(char) => return Err(GDTFNameError::NotAllowedCharError(format!(" '{}' is not an allowed char for Name in GDTF", char))),
                    Err(_) => return Err(GDTFNameError::NotAllowedCharError(format!("Invalid char for Name in GDTF found")))
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Name::Name(left) => if let Name::Name(right) = other {
                left == right
            } else {
                false
            },
            Name::Empty => if let Name::Empty = other {
                true
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::testdata;
    use crate::utils::units::name::Name;

    #[test]
    fn test_default() {
        assert_eq!(Name::Empty, Default::default());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Name::Name("My Name".to_string())), "My Name");
        assert_eq!(format!("{}", Name::Name("Something else".to_string())), "Something else");
        assert_eq!(format!("{}", Name::Empty), "");
    }

    #[test]
    fn test_try_from_str() {
        //Name has restricted set of valid chars
        assert!(Name::try_from(std::str::from_utf8(&[19]).unwrap()).is_err());
        //Name has restricted set of valid chars
        assert!(Name::try_from("{").is_err());
        //Name has restricted set of valid chars
        assert!(Name::try_from("Some {Name").is_err());
        assert_eq!(Name::Empty, "".try_into().unwrap());
        assert_eq!(Name::Name(" ".to_string()), " ".try_into().unwrap());
        assert_eq!(Name::Name("  ".to_string()), "  ".try_into().unwrap());
        assert_eq!(Name::Name("Some Name".to_string()), "Some Name".try_into().unwrap());
        assert_eq!(Name::Name("Some Other Name".to_string()), "Some Other Name".try_into().unwrap());
        assert_eq!(Name::Name("  Some  Name  ".to_string()), "  Some  Name  ".try_into().unwrap());
    }

    #[test]
    fn test_from_attr_borrowed() {
        //Name has restricted set of valid chars but it's allowed when coming from xml because Rust can handle it
        assert_eq!(Name::Name("{".to_string()), testdata::to_attr_borrowed(b"{").into());
        //Name has restricted set of valid chars but it's allowed when coming from xml because Rust can handle it
        assert_eq!(Name::Name("Some {Name".to_string()), testdata::to_attr_borrowed(b"Some {Name").into());
        assert_eq!(Name::Empty, testdata::to_attr_borrowed(b"").into());
        assert_eq!(Name::Name(" ".to_string()), testdata::to_attr_borrowed(b" ").into());
        assert_eq!(Name::Name("  ".to_string()), testdata::to_attr_borrowed(b"  ").into());
        assert_eq!(Name::Name("Some Name".to_string()), testdata::to_attr_borrowed(b"Some Name").into());
        assert_eq!(Name::Name("Some Other Name".to_string()), testdata::to_attr_borrowed(b"Some Other Name").into());
        assert_eq!(Name::Name("  Some  Name  ".to_string()), testdata::to_attr_borrowed(b"  Some  Name  ").into());
    }

    #[test]
    fn test_from_attr_owned() {
        //Name has restricted set of valid chars but it's allowed when coming from xml because Rust can handle it
        assert_eq!(Name::Name("{".to_string()), testdata::to_attr_owned(b"{").into());
        //Name has restricted set of valid chars but it's allowed when coming from xml because Rust can handle it
        assert_eq!(Name::Name("Some {Name".to_string()), testdata::to_attr_owned(b"Some {Name").into());
        assert_eq!(Name::Empty, testdata::to_attr_owned(b"").into());
        assert_eq!(Name::Name(" ".to_string()), testdata::to_attr_owned(b" ").into());
        assert_eq!(Name::Name("  ".to_string()), testdata::to_attr_owned(b"  ").into());
        assert_eq!(Name::Name("Some Name".to_string()), testdata::to_attr_owned(b"Some Name").into());
        assert_eq!(Name::Name("Some Other Name".to_string()), testdata::to_attr_owned(b"Some Other Name").into());
        assert_eq!(Name::Name("  Some  Name  ".to_string()), testdata::to_attr_owned(b"  Some  Name  ").into());
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(Name::Empty, Name::Empty);
        assert_eq!(Name::Name("Hello".to_string()), Name::Name("Hello".to_string()));
        assert_eq!(Name::Name("MyName".to_string()), Name::Name("MyName".to_string()));
        assert_ne!(Name::Name("Hello".to_string()), Name::Empty);
        assert_ne!(Name::Empty, Name::Name("Hello".to_string()));
        assert_ne!(Name::Name("Hello".to_string()), Name::Name("MyName".to_string()));
    }
}