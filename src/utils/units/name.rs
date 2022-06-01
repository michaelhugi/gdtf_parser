//! Module for the unit Name used in GDTF
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Deserializer};
use serde::de::Visitor;
use unicode_segmentation::UnicodeSegmentation;

///Name representation used in GDTF spec
///Name contains a String that only can hold letters with restricted literals `[32..=122] = (SPACE..='z')` due to GDTF specifications.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Name(pub String);

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(NameVisitor)
    }
}

struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Expecting string without special chars")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        match Name::new(v) {
            Ok(v) => Ok(v),
            Err(e) => Err(serde::de::Error::custom(format!("{}", e)))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        self.visit_str(&v)
    }
}

///Default is an empty Name
/// ```rust
/// use gdtf_parser::utils::units::name::Name;
/// assert_eq!(Name::new("").unwrap(), Default::default());
/// ```
impl Default for Name {
    fn default() -> Self {
        Name("".to_string())
    }
}

impl Name {
    ///Creates a new instance of Name from a str. Only chars `[32..=122] = (SPACE..='z')` are allowed. if one of the other chars is passed to the function, it will return an Error
    /// ## Examples
    /// ```rust
    /// use gdtf_parser::utils::units::name::Name;
    /// assert_eq!(Name("".to_string()), Name::new("").unwrap());
    /// assert_eq!(Name("Some Name".to_string()), Name::new("Some Name").unwrap());
    /// assert!(Name::new("Some Name with invalid char {").is_err());
    /// assert!(Name::new("Some Name with invalid char ȸ").is_err());
    ///
    /// ```
    pub fn new(name: &str) -> Result<Self, GdtfNameError> {
        Self::validate_chars(name)?;
        Ok(Self(name.to_string()))
    }

    ///Validates if all chars in a string are in `[32..=122] = (SPACE..='z')` due to GDTF specifications for Name
    /// ## Examples
    /// ```rust
    /// use gdtf_parser::utils::units::name::Name;
    /// assert!(Name::validate_chars("").is_ok());
    /// assert!(Name::validate_chars("Some String").is_ok());
    /// assert!(Name::validate_chars("Some String with invalid char {").is_err());
    /// assert!(Name::validate_chars("Some String with invalid char ȸ").is_err());
    /// ```
    /// ## Usage
    /// ```rust
    /// use gdtf_parser::utils::errors::GdtfError;
    /// use gdtf_parser::utils::units::name::Name;
    /// fn main() -> Result<(),GdtfError>{
    ///     let test_string: &str = "Some String";
    ///     Name::validate_chars(test_string)?;
    ///     //String is valid for Name
    ///     Ok(())
    /// }
    /// ```
    pub fn validate_chars(s: &str) -> Result<(), GdtfNameError> {
        for grapheme in s.graphemes(true) {
            if grapheme.len() != 1 {
                return Err(GdtfNameError::NotAllowedCharError(grapheme.to_string()));
            } else {
                for char in grapheme.chars() {
                    let char = char as u8;
                    if !(32..=122).contains(&char) {
                        let char = [char];
                        match std::str::from_utf8(&char) {
                            Ok(char) => {
                                return Err(GdtfNameError::NotAllowedCharError(char.to_string()));
                            }
                            Err(_) => {
                                return Err(GdtfNameError::NotAllowedCharError(
                                    "Invalid char for Name in GDTF found".to_string(),
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
///Error used to indicate an Error during creating of Name
pub enum GdtfNameError {
    ///Error used when a Name was tried to be created with a &str that contains a char out of the scope `[32..=122] = (SPACE..='z')` defined by Gdtf-specifications for Name
    NotAllowedCharError(String),
}

impl Error for GdtfNameError {}

impl fmt::Display for GdtfNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GdtfNameError::*;
        match self {
            //GDTFNameError(_) => write!(f, "ColorCIE Error. Utf8 Error"),
            NotAllowedCharError(s) => write!(
                f,
                "GdtfNameError: '{}' is not an allowed char for Name in GDTF",
                s
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use crate::utils::errors::GdtfError;
    use crate::utils::units::name::Name;

    #[test]
    fn test_new() -> Result<(), GdtfError> {
        assert_eq!(Name("".to_string()), Name::new("")?);
        assert_eq!(Name("Some Name".to_string()), Name::new("Some Name")?);
        assert!(Name::new("Some Name with invalid char {").is_err());
        assert!(Name::new("Some Name with invalid char ȸ").is_err());
        assert!(Name::new(std::str::from_utf8(&[20, 19, 21])?).is_err());
        Ok(())
    }

    #[test]
    fn test_display_error() {
        match Name::new("Some Name with the invalid char { in the middle") {
            Ok(_) => panic!("Should return an error"),
            Err(e) => assert_eq!(
                format!("{}", e),
                "GdtfNameError: '{' is not an allowed char for Name in GDTF"
            ),
        }
        match Name::new("Some Name with the invalid char ȸ in the middle") {
            Ok(_) => panic!("Should return an error"),
            Err(e) => assert_eq!(
                format!("{}", e),
                "GdtfNameError: 'ȸ' is not an allowed char for Name in GDTF"
            ),
        }
    }

    #[test]
    fn test_validate_chars() {
        let one_byte = vec![0, 127];
        for i in one_byte[0]..one_byte[1] {
            let mut first = format!("{:X}", i);
            first = make_even(first);
            test_validate_char(first);
        }
        let two_bytes = vec![192, 223, 64, 191];
        for i in two_bytes[0]..two_bytes[1] {
            for j in two_bytes[2]..two_bytes[3] {
                let mut first = format!("{:X}", i);
                let mut second = format!("{:X}", j);

                first = make_even(first);
                second = make_even(second);

                test_validate_char(first.to_string() + &second.to_string());
            }
        }
        let three_bytes = vec![224, 239, 64, 191, 64, 191];
        for i in three_bytes[0]..three_bytes[1] {
            for j in three_bytes[2]..three_bytes[3] {
                for k in three_bytes[4]..three_bytes[5] {
                    let mut first = format!("{:X}", i);
                    let mut second = format!("{:X}", j);
                    let mut third = format!("{:X}", k);

                    first = make_even(first);
                    second = make_even(second);
                    third = make_even(third);

                    test_validate_char(
                        first.to_string() + &second.to_string() + &third.to_string(),
                    );
                }
            }
        }
    }

    fn make_even(mut s: String) -> String {
        if s.len() % 2 == 1 {
            s = "0".to_string() + &s.to_string();
        }
        return s;
    }

    fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }

    fn test_validate_char(hex: String) {
        match &decode_hex(&hex) {
            Ok(dh) => match std::str::from_utf8(dh) {
                Ok(v) => {
                    if format!("{:?}", v).len() < 7 {
                        let z = i64::from_str_radix(&hex, 16).unwrap();
                        if z >= 20 && z <= 122 {
                            assert!(Name::validate_chars(v).is_ok());
                        } else {
                            assert!(Name::validate_chars(v).is_err());
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
