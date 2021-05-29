//TODO check
//! Module for the unit Name used in GDTF
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::error::Error;

use std::fmt;

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;

///Name representation used in GDTF
#[derive(Debug)]
pub struct Name(String);

///Default is an empty Name
impl Default for Name {
    ///Default is an empty Name
    fn default() -> Self {
        Name("".to_string())
    }
}



///Deparses Name from Attribute safely. In case of error it will return default. It will also allow not valid chars from GDTF-Spec because Rust can handle it!
impl From<Attribute<'_>> for Name {
    ///Depares Name safely from Attribute. In case of error it returns default. It will also allow not valid chars from GDTF-Spec because Rust can handle it!
    fn from(attr: Attribute) -> Self {
        Name::new_unchecked(std::str::from_utf8(attr.value.borrow()).unwrap_or_else(|_| ""))
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
        Self::new(s)
    }
}

impl Name {
    ///Creates a new instance of name. Only chars to GDTF Spec are allowed
    pub fn new(name: &str) -> Result<Self, GDTFNameError> {
        Self::validate_chars(name)?;
        Ok(Self(name.to_string()))
    }
    ///Creates an new_unchecked name with any given char allowed
    pub(crate) fn new_unchecked(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn validate_chars(s: &str) -> Result<(), GDTFNameError> {
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

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == "" || other.0 == "" {
            false
        } else {
            self.0 == other.0
        }
    }
}

#[cfg(test)]
impl PartialEqAllowEmpty for Name {
    fn is_eq_allow_empty_impl(&self, other: &Self, _: bool) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::name::{GDTFNameError, Name};

    #[test]
    fn test_default() -> Result<(), GdtfError> {
        Name::new("")?.assert_eq_allow_empty(&Default::default(), true);
        Ok(())
    }

    #[test]
    fn test_new() -> Result<(), GDTFNameError> {
        Name::new("test")?.assert_eq_allow_empty(&Name::new("test")?, true);
        Name::new("")?.assert_eq_allow_empty(&Name::new("")?, true);
        assert!(Name::new("asd{").is_err());
        Ok(())
    }

    #[test]
    fn test_new_unchecked() -> Result<(), GDTFNameError> {
        Name("test".to_string()).assert_eq_allow_empty(&Name::new_unchecked("test"), true);
        Name("".to_string()).assert_eq_allow_empty(&Name::new_unchecked(""), true);
        Name("asd{".to_string()).assert_eq_allow_empty(&Name::new_unchecked("asd{"), true);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        //Name has restricted set of valid chars
        assert!(Name::try_from(std::str::from_utf8(&[19]).unwrap()).is_err());
        //Name has restricted set of valid chars
        assert!(Name::try_from("{").is_err());
        //Name has restricted set of valid chars
        assert!(Name::try_from("Some {Name").is_err());
        Name::new("")?.assert_eq_allow_empty(&"".try_into().unwrap(), true);
        assert_eq!(Name::new(" ")?, " ".try_into().unwrap());
        assert_eq!(Name::new("  ")?, "  ".try_into().unwrap());
        assert_eq!(Name::new("Some Name")?, "Some Name".try_into().unwrap());
        assert_eq!(Name::new("Some Other Name")?, "Some Other Name".try_into().unwrap());
        assert_eq!(Name::new("  Some  Name  ")?, "  Some  Name  ".try_into().unwrap());
        Ok(())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        //Name has restricted set of valid chars but it's allowed when coming from xml because Rust can handle it
        assert_eq!(Name::new_unchecked("{"), testdata::to_attr_borrowed(b"{").into());
        //Name has restricted set of valid chars but it's allowed when coming from xml because Rust can handle it
        assert_eq!(Name::new_unchecked("Some {Name"), testdata::to_attr_borrowed(b"Some {Name").into());
        Name::new("")?.assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        assert_eq!(Name::new(" ")?, testdata::to_attr_borrowed(b" ").into());
        assert_eq!(Name::new("  ")?, testdata::to_attr_borrowed(b"  ").into());
        assert_eq!(Name::new("Some Name")?, testdata::to_attr_borrowed(b"Some Name").into());
        assert_eq!(Name::new("Some Other Name")?, testdata::to_attr_borrowed(b"Some Other Name").into());
        assert_eq!(Name::new("  Some  Name  ")?, testdata::to_attr_borrowed(b"  Some  Name  ").into());
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        //Name has restricted set of valid chars but it's allowed when coming from xml because Rust can handle it
        assert_eq!(Name::new_unchecked("{"), testdata::to_attr_owned(b"{").into());
        //Name has restricted set of valid chars but it's allowed when coming from xml because Rust can handle it
        assert_eq!(Name::new_unchecked("Some {Name"), testdata::to_attr_owned(b"Some {Name").into());
        Name::new("")?.assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        assert_eq!(Name::new(" ")?, testdata::to_attr_owned(b" ").into());
        assert_eq!(Name::new("  ")?, testdata::to_attr_owned(b"  ").into());
        assert_eq!(Name::new("Some Name")?, testdata::to_attr_owned(b"Some Name").into());
        assert_eq!(Name::new("Some Other Name")?, testdata::to_attr_owned(b"Some Other Name").into());
        assert_eq!(Name::new("  Some  Name  ")?, testdata::to_attr_owned(b"  Some  Name  ").into());
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_ne!(Name::new("")?, Name::new("")?);
        assert_eq!(Name::new("Hello")?, Name::new("Hello")?);
        assert_eq!(Name::new("MyName")?, Name::new("MyName")?);
        assert_ne!(Name::new("Hello")?, Name::new("")?);
        assert_ne!(Name::new("")?, Name::new("Hello")?);
        assert_ne!(Name::new("Hello")?, Name::new("MyName")?);
        Ok(())
    }

    #[test]
    fn test_partial_eq_allow_empty() -> Result<(), GdtfError> {
        Name::new("")?.assert_eq_allow_empty(&Name::new("")?, true);
        Name::new("Hello")?.assert_eq_allow_empty(&Name::new("Hello")?, true);
        Name::new("MyName")?.assert_eq_allow_empty(&Name::new("MyName")?, true);
        Name::new("Hello")?.assert_ne_allow_empty(&Name::new("")?);
        Name::new("")?.assert_ne_allow_empty(&Name::new("Hello")?);
        Name::new("Hello")?.assert_ne_allow_empty(&Name::new("MyName")?);
        Ok(())
    }
}