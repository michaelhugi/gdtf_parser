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
    fn test_invalid_char_low() {
        if Name::try_from(std::str::from_utf8(&[19]).unwrap()).is_ok() {
            panic!("Deparsing Name from string allowed invalid char");
        }
    }

    #[test]
    fn test_invalid_char_high() {
        if Name::try_from("{").is_ok() {
            panic!("Deparsing Name from string allowed invalid char");
        }
    }

    #[test]
    fn test_valid_char_high() {
        Name::try_from("z").unwrap();
    }

    #[test]
    fn test_valid_char_low() {
        Name::try_from(" ").unwrap();
    }

    #[test]
    fn test_parse_from_str_empty() {
        assert_eq!(
            Name::Empty,
            Name::try_from("").unwrap()
        );
    }

    #[test]
    fn test_parse_from_str_space() {
        assert_eq!(
            Name::Name(" ".to_string()),
            Name::try_from(" ").unwrap()
        );
    }

    #[test]
    fn test_parse_from_str_double_space() {
        assert_eq!(
            Name::Name("  ".to_string()),
            Name::try_from("  ").unwrap()
        );
    }

    #[test]
    fn test_parse_from_str_valid() {
        assert_eq!(
            Name::Name("Some Name".to_string()),
            Name::try_from("Some Name").unwrap()
        );
    }

    #[test]
    fn test_parse_from_str_invalid() {
        if Name::try_from("Some {Name").is_ok() {
            panic!("Parsing from invalid str was allowed but shouldn't be")
        }
    }

    #[test]
    fn test_parse_valid_attr_borrowed() {
        assert_eq!(
            Name::Name("My Name".to_string()),
            testdata::to_attr_borrowed(b"My Name").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_valid_attr_owned() {
        assert_eq!(
            Name::Name("My Name".to_string()),
            testdata::to_attr_owned(b"My Name").try_into().unwrap()
        );
    }

    #[test]
    ///Invalid Chars are allowed from GDTF to application because Rust can handle it
    fn test_parse_invalid_attr_owned() {
        assert_eq!(
            Name::Name("My N{ame".to_string()),
            testdata::to_attr_owned(b"My N{ame").into()
        );
    }

    #[test]
    ///Invalid Chars are allowed from GDTF to application because Rust can handle it
    fn test_parse_invalid_attr_borrowed() {
        assert_eq!(
            Name::Name("My N{ame".to_string()),
            testdata::to_attr_borrowed(b"My N{ame").into()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Name::Name("My Name".to_string())), "My Name");
    }

    #[test]
    fn test_parse_empty_attr_owned() {
        assert_eq!(
            Name::Empty,
            testdata::to_attr_owned(b"").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_empty_attr_borrowed() {
        assert_eq!(
            Name::Empty,
            testdata::to_attr_borrowed(b"").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_space_attr_owned() {
        assert_eq!(
            Name::Name(" ".to_string()),
            testdata::to_attr_owned(b" ").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_space_attr_borrowed() {
        assert_eq!(
            Name::Name(" ".to_string()),
            testdata::to_attr_borrowed(b" ").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_doublespace_attr_owned() {
        assert_eq!(
            Name::Name("  ".to_string()),
            testdata::to_attr_owned(b"  ").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_doublespace_attr_borrowed() {
        assert_eq!(
            Name::Name("  ".to_string()),
            testdata::to_attr_borrowed(b"  ").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_with_empty_attr_owned() {
        assert_eq!(
            Name::Name(" My Name ".to_string()),
            testdata::to_attr_owned(b" My Name ").try_into().unwrap()
        );
    }

    #[test]
    fn test_parse_with_empty_attr_borrowed() {
        assert_eq!(
            Name::Name(" My Name ".to_string()),
            testdata::to_attr_borrowed(b" My Name ").try_into().unwrap()
        );
    }
}