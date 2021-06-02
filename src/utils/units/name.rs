//TODO check
//! Module for the unit Name used in GDTF
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

use quick_xml::events::attributes::Attribute;

///Name representation used in GDTF
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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

    ///creates a new vec of Nams from single str where name is not checked for validity defined by GDTF
    pub fn str_to_names_vec_unchecked(value: &str) -> Vec<Name> {
        if value == "" {
            return vec![];
        }
        let value = value.split(".");
        let mut tree: Vec<Name> = vec![];
        for value in value.into_iter() {
            tree.push(Name::new_unchecked(value));
        }
        tree
    }

    #[cfg(test)]
    ///creates a new vec of Name from vec of str  where names are not checked for validity defined by GDTF
    pub fn strs_to_names_vec_unchecked(names: Vec<&str>) -> Vec<Name> {
        let mut ns = vec![];
        for name in names.iter() {
            ns.push(Name::new_unchecked(name))
        }
        ns
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::name::{GDTFNameError, Name};

    #[test]
    fn test_default() -> Result<(), GdtfError> {
        assert_eq!(Name::new("")?, Default::default());
        Ok(())
    }

    #[test]
    fn test_new() -> Result<(), GDTFNameError> {
        assert_eq!(Name::new("test")?, Name::new("test")?);
        assert_eq!(Name::new("")?, Name::new("")?);
        assert!(Name::new("asd{").is_err());
        Ok(())
    }

    #[test]
    fn test_new_unchecked() -> Result<(), GDTFNameError> {
        assert_eq!(Name("test".to_string()), Name::new_unchecked("test"));
        assert_eq!(Name("".to_string()), Name::new_unchecked(""));
        assert_eq!(Name("asd{".to_string()), Name::new_unchecked("asd{"));
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
        assert_eq!(Name::new("")?, "".try_into().unwrap());
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
        assert_eq!(Name::new("")?, testdata::to_attr_borrowed(b"").into());
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
        assert_eq!(Name::new("")?, testdata::to_attr_owned(b"").into());
        assert_eq!(Name::new(" ")?, testdata::to_attr_owned(b" ").into());
        assert_eq!(Name::new("  ")?, testdata::to_attr_owned(b"  ").into());
        assert_eq!(Name::new("Some Name")?, testdata::to_attr_owned(b"Some Name").into());
        assert_eq!(Name::new("Some Other Name")?, testdata::to_attr_owned(b"Some Other Name").into());
        assert_eq!(Name::new("  Some  Name  ")?, testdata::to_attr_owned(b"  Some  Name  ").into());
        Ok(())
    }
}