//! Module for the unit GUID used in FixtureType in GDTF
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

use quick_xml::events::attributes::Attribute;
use serde::{Serialize, Deserialize};

use crate::utils::read;

/// Char - represented as u8 for matching
const CHAR_MINUS_AS_U8: u8 = 0x2D;
/// Char 0 represented as u8 for matching
const CHAR_0_AS_U8: u8 = 0x30;
/// Char 1 represented as u8 for matching
const CHAR_1_AS_U8: u8 = 0x31;
/// Char 2 represented as u8 for matching
const CHAR_2_AS_U8: u8 = 0x32;
/// Char 3 represented as u8 for matching
const CHAR_3_AS_U8: u8 = 0x33;
/// Char 4 represented as u8 for matching
const CHAR_4_AS_U8: u8 = 0x34;
/// Char 5 represented as u8 for matching
const CHAR_5_AS_U8: u8 = 0x35;
/// Char 6 represented as u8 for matching
const CHAR_6_AS_U8: u8 = 0x36;
/// Char 7 represented as u8 for matching
const CHAR_7_AS_U8: u8 = 0x37;
/// Char 8 represented as u8 for matching
const CHAR_8_AS_U8: u8 = 0x38;
/// Char 9 represented as u8 for matching
const CHAR_9_AS_U8: u8 = 0x39;
/// Char A represented as u8 for matching
const CHAR_A_AS_U8: u8 = 0x41;
/// Char B represented as u8 for matching
const CHAR_B_AS_U8: u8 = 0x42;
/// Char C represented as u8 for matching
const CHAR_C_AS_U8: u8 = 0x43;
/// Char D represented as u8 for matching
const CHAR_D_AS_U8: u8 = 0x44;
/// Char E represented as u8 for matching
const CHAR_E_AS_U8: u8 = 0x45;
/// Char F represented as u8 for matching
const CHAR_F_AS_U8: u8 = 0x46;

///GUID representation used in FixtureType in GDTF
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Guid(pub [u8; 16]);

impl Guid {
    ///Returns a dummy FixtureType_Guid with all zeros
    /// ```rust
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid([0; 16]), Guid::dummy());
    /// ```
    pub fn dummy() -> Self {
        Self([0; 16])
    }

    /// converts a string in format XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX where XX is a byte in hex in UTF8 format to a GUID. Retunrs an error if the format is not correct
    /// ```rust
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(
    ///     Guid([48, 142, 168, 125, 113, 100, 66, 222, 129, 6, 166, 210, 115, 245, 122, 81]),
    ///     Guid::new_from_str("308EA87D-7164-42DE-8106-A6D273F57A51").unwrap()
    /// );
    /// assert!(Guid::new_from_str("Somthing invalid").is_err());
    /// ```
    pub fn new_from_str(s: &str) -> Result<Self, GdtfGuidError> {
        let s: Vec<char> = s.chars().collect();
        let mut s = s.iter().map(|c| *c as u8).collect::<Vec<_>>();

        let mut bytes = [0_u8; 16];
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[15] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[14] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[13] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[12] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[11] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[10] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        match s.pop() {
            Some(CHAR_MINUS_AS_U8) => {}
            _ => return Err(GdtfGuidError {}),
        }
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[9] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[8] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        match s.pop() {
            Some(CHAR_MINUS_AS_U8) => {}
            _ => return Err(GdtfGuidError {}),
        }
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[7] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[6] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        match s.pop() {
            Some(CHAR_MINUS_AS_U8) => {}
            _ => return Err(GdtfGuidError {}),
        }
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[5] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[4] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        match s.pop() {
            Some(CHAR_MINUS_AS_U8) => {}
            _ => return Err(GdtfGuidError {}),
        }
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[3] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[2] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[1] = Self::pop_last_byte(&mut s)?;
        //Is safe because only 0..F and - chars will succeed anyway
        bytes[0] = Self::pop_last_byte(&mut s)?;

        Ok(Guid(bytes))
    }

    /// Converts a quick-xml-attribute from gdtf-xml-description to a FixtureType_Guid. Returns an error if the format is not correct.
    /// ```rust
    /// use gdtf_parser::utils::units::guid::Guid;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(
    ///     Guid([48, 142, 168, 125, 113, 100, 66, 222, 129, 6, 166, 210, 115, 245, 122, 81]),
    ///     Guid::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"308EA87D-7164-42DE-8106-A6D273F57A51")}).unwrap()
    /// );
    /// assert!(Guid::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Something invalid")}).is_err());
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Result<Self, GdtfGuidError> {
        Self::new_from_str(read::attr_to_str(&attr))
    }

    ///Returns the GUID as a string in format  XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX where XX is a byte in hex in UTF8 format or "" if GUID is empty
    ///```rust
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!( Guid([48, 142, 168, 125, 113, 100, 66, 222, 129, 6, 166, 210, 115, 245, 122, 81]).to_str().unwrap(), "308EA87D-7164-42DE-8106-A6D273F57A51".to_string());
    ///```
    pub fn to_str(&self) -> Result<String, GdtfGuidError> {
        let val = self.0;
        let mut v = [0_u8; 36];

        let (v1, v2) = Self::split_into_two_halfbytes(val[0])?;
        v[0] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[1] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[1])?;
        v[2] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[3] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[2])?;
        v[4] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[5] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[3])?;
        v[6] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[7] = Self::halfbyte_to_hexcharbyte(v2)?;

        v[8] = CHAR_MINUS_AS_U8;

        let (v1, v2) = Self::split_into_two_halfbytes(val[4])?;
        v[9] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[10] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[5])?;
        v[11] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[12] = Self::halfbyte_to_hexcharbyte(v2)?;

        v[13] = CHAR_MINUS_AS_U8;

        let (v1, v2) = Self::split_into_two_halfbytes(val[6])?;
        v[14] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[15] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[7])?;
        v[16] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[17] = Self::halfbyte_to_hexcharbyte(v2)?;

        v[18] = CHAR_MINUS_AS_U8;

        let (v1, v2) = Self::split_into_two_halfbytes(val[8])?;
        v[19] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[20] = Self::halfbyte_to_hexcharbyte(v2)?;

        let (v1, v2) = Self::split_into_two_halfbytes(val[9])?;
        v[21] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[22] = Self::halfbyte_to_hexcharbyte(v2)?;

        v[23] = CHAR_MINUS_AS_U8;

        let (v1, v2) = Self::split_into_two_halfbytes(val[10])?;
        v[24] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[25] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[11])?;
        v[26] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[27] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[12])?;
        v[28] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[29] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[13])?;
        v[30] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[31] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[14])?;
        v[32] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[33] = Self::halfbyte_to_hexcharbyte(v2)?;
        let (v1, v2) = Self::split_into_two_halfbytes(val[15])?;
        v[34] = Self::halfbyte_to_hexcharbyte(v1)?;
        v[35] = Self::halfbyte_to_hexcharbyte(v2)?;

        Ok(std::str::from_utf8(&v)?.to_string())
    }

    ///Helper method to convert a str to GUID. It pops the last byte from a str in format XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX where XX is a byte in hex in UTF8 format and returns it's value as u8. This method will always pop the last two chars who represent a byte.
    fn pop_last_byte(vec: &mut Vec<u8>) -> Result<u8, GdtfGuidError> {
        let (first, second) = match vec.pop() {
            None => Err(GdtfGuidError {}),
            Some(val2) => match vec.pop() {
                None => Err(GdtfGuidError {}),
                Some(val1) => Ok((val1, val2)),
            },
        }?;
        Self::hexcharbytes_to_byte(first, second)
    }

    ///Interprets a UTF8 formated hex charbyte to a halfbyte.
    /// ```ignore
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid::hexcharbyte_to_halfbyte(0x39).unwrap(), 9);
    /// assert_eq!(Guid::hexcharbyte_to_halfbyte(0x41).unwrap(), 10);
    /// ```
    fn hexcharbyte_to_halfbyte(c: u8) -> Result<u8, GdtfGuidError> {
        match c {
            CHAR_0_AS_U8 => Ok(0),
            CHAR_1_AS_U8 => Ok(1),
            CHAR_2_AS_U8 => Ok(2),
            CHAR_3_AS_U8 => Ok(3),
            CHAR_4_AS_U8 => Ok(4),
            CHAR_5_AS_U8 => Ok(5),
            CHAR_6_AS_U8 => Ok(6),
            CHAR_7_AS_U8 => Ok(7),
            CHAR_8_AS_U8 => Ok(8),
            CHAR_9_AS_U8 => Ok(9),
            CHAR_A_AS_U8 => Ok(10),
            CHAR_B_AS_U8 => Ok(11),
            CHAR_C_AS_U8 => Ok(12),
            CHAR_D_AS_U8 => Ok(13),
            CHAR_E_AS_U8 => Ok(14),
            CHAR_F_AS_U8 => Ok(15),
            _ => Err(GdtfGuidError {}),
        }
    }

    ///Interprets a u8 as UTF8 formated hex charbyte to a halfbyte.
    /// ```ignore
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid::halfbyte_to_hexcharbyte(13).unwrap(), 0x44);
    /// assert_eq!(Guid::halfbyte_to_hexcharbyte(14).unwrap(), 0x45);
    ///  ```
    fn halfbyte_to_hexcharbyte(c: u8) -> Result<u8, GdtfGuidError> {
        match c {
            0 => Ok(CHAR_0_AS_U8),
            1 => Ok(CHAR_1_AS_U8),
            2 => Ok(CHAR_2_AS_U8),
            3 => Ok(CHAR_3_AS_U8),
            4 => Ok(CHAR_4_AS_U8),
            5 => Ok(CHAR_5_AS_U8),
            6 => Ok(CHAR_6_AS_U8),
            7 => Ok(CHAR_7_AS_U8),
            8 => Ok(CHAR_8_AS_U8),
            9 => Ok(CHAR_9_AS_U8),
            10 => Ok(CHAR_A_AS_U8),
            11 => Ok(CHAR_B_AS_U8),
            12 => Ok(CHAR_C_AS_U8),
            13 => Ok(CHAR_D_AS_U8),
            14 => Ok(CHAR_E_AS_U8),
            15 => Ok(CHAR_F_AS_U8),
            _ => Err(GdtfGuidError {}),
        }
    }

    ///Tells if the bit of a byte is 1 at a specific index
    /// ```ignore
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid::is_byte_one_at_index(0b1001_0101,0).unwrap(),true);
    /// assert_eq!(Guid::is_byte_one_at_index(0b0001_0101,0).unwrap(),false);
    /// assert_eq!(Guid::is_byte_one_at_index(0b1001_0101,3).unwrap(),true);
    /// assert_eq!(Guid::is_byte_one_at_index(0b1000_0101,3).unwrap(),false);
    ///  ```
    fn is_byte_one_at_index(byte: u8, index: u8) -> Result<bool, GdtfGuidError> {
        match index {
            0 => Ok(0b1000_0000_u8 & byte == 0b1000_0000_u8),
            1 => Ok(0b0100_0000_u8 & byte == 0b0100_0000_u8),
            2 => Ok(0b0010_0000_u8 & byte == 0b0010_0000_u8),
            3 => Ok(0b0001_0000_u8 & byte == 0b0001_0000_u8),
            4 => Ok(0b0000_1000_u8 & byte == 0b0000_1000_u8),
            5 => Ok(0b0000_0100_u8 & byte == 0b0000_0100_u8),
            6 => Ok(0b0000_0010_u8 & byte == 0b0000_0010_u8),
            7 => Ok(0b0000_0001_u8 & byte == 0b0000_0001_u8),
            _ => Err(GdtfGuidError {}),
        }
    }

    ///shifts bytes from lower to upper.
    /// ```ignore
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid::shift_byte_lower_to_upper(0b0100_0101).unwrap(), 0b0101_0000);
    /// ```
    fn shift_byte_lower_to_upper(byte: u8) -> Result<u8, GdtfGuidError> {
        let s1 = if Self::is_byte_one_at_index(byte, 4)? {
            0b1000_0000_u8
        } else {
            0
        };
        let s2 = if Self::is_byte_one_at_index(byte, 5)? {
            0b0100_0000_u8
        } else {
            0
        };
        let s3 = if Self::is_byte_one_at_index(byte, 6)? {
            0b0010_0000_u8
        } else {
            0
        };
        let s4 = if Self::is_byte_one_at_index(byte, 7)? {
            0b0001_0000_u8
        } else {
            0
        };
        Ok(s1 + s2 + s3 + s4)
    }

    ///joins two halfbytes to one byte.
    /// ```ignore
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid::join_two_halfbytes(0b0000_0101,0b0000_0001).unwrap(),0b0101_0001);
    /// assert_eq!(Guid::join_two_halfbytes(0b0000_1101,0b0000_1001).unwrap(),0b1101_1001);
    /// ```
    fn join_two_halfbytes(first_half: u8, second_half: u8) -> Result<u8, GdtfGuidError> {
        Ok(Self::shift_byte_lower_to_upper(first_half)? + second_half)
    }

    ///shifts bytes from upper to lower.
    /// ```ignore
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid::get_upper_halfbyte(0b0000_0100).unwrap(),0b0000_0000);
    /// assert_eq!(Guid::get_upper_halfbyte(0b0010_1100).unwrap(),0b0000_0010);
    ///  ```
    fn get_upper_halfbyte(byte: u8) -> Result<u8, GdtfGuidError> {
        let s1 = if Self::is_byte_one_at_index(byte, 0)? {
            0b0000_1000_u8
        } else {
            0
        };
        let s2 = if Self::is_byte_one_at_index(byte, 1)? {
            0b0000_0100_u8
        } else {
            0
        };
        let s3 = if Self::is_byte_one_at_index(byte, 2)? {
            0b0000_0010_u8
        } else {
            0
        };
        let s4 = if Self::is_byte_one_at_index(byte, 3)? {
            0b0000_0001_u8
        } else {
            0
        };
        Ok(s1 + s2 + s3 + s4)
    }

    ///removes the upper half of a byte.
    ///```ignore
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid::get_lower_halfbyte(0b0000_0100).unwrap(),0b0000_0100);
    /// assert_eq!(Guid::get_lower_halfbyte(0b0010_1100).unwrap(),0b0000_1100);
    ///  ```
    ///
    fn get_lower_halfbyte(byte: u8) -> Result<u8, GdtfGuidError> {
        let s1 = if Self::is_byte_one_at_index(byte, 4)? {
            0b0000_1000_u8
        } else {
            0
        };
        let s2 = if Self::is_byte_one_at_index(byte, 5)? {
            0b0000_0100_u8
        } else {
            0
        };
        let s3 = if Self::is_byte_one_at_index(byte, 6)? {
            0b0000_0010_u8
        } else {
            0
        };
        let s4 = if Self::is_byte_one_at_index(byte, 7)? {
            0b0000_0001_u8
        } else {
            0
        };
        Ok(s1 + s2 + s3 + s4)
    }

    ///Splits a byte into two halfbytes.
    /// ```ignore
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid::split_into_two_halfbytes(0b0100_0101).unwrap(), (0b0000_0100,0b0000_0101));
    /// assert_eq!(Guid::split_into_two_halfbytes(0b1001_1101).unwrap(), (0b0000_1001,0b0000_1101));
    ///  ```
    fn split_into_two_halfbytes(b: u8) -> Result<(u8, u8), GdtfGuidError> {
        Ok((Self::get_upper_halfbyte(b)?, Self::get_lower_halfbyte(b)?))
    }

    ///Interprets two UTF8 formated hex charbyte to a byte
    /// ```ignore
    /// use gdtf_parser::utils::units::guid::Guid;
    /// assert_eq!(Guid::hexcharbytes_to_byte(0x41,0x33).unwrap(),163_u8);
    ///  ```
    fn hexcharbytes_to_byte(c1: u8, c2: u8) -> Result<u8, GdtfGuidError> {
        Self::join_two_halfbytes(
            Self::hexcharbyte_to_halfbyte(c1)?,
            Self::hexcharbyte_to_halfbyte(c2)?,
        )
    }
}

///Displays a GUID in the format XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX where XX is a byte in hex in UTF8 format
/// ```ignore
/// use gdtf_parser::utils::units::guid::Guid;
/// assert_eq!(format!("{}", Guid([48, 142, 168, 125, 113, 100, 66, 222, 129, 6, 166, 210, 115, 245, 122, 81])), "308EA87D-7164-42DE-8106-A6D273F57A51");
///  ```
impl Display for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.to_str() {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "Unknown!!"),
        }
    }
}

#[derive(Debug)]
/// Error that occures if the format of GUID is wrong e.q. not XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX
pub struct GdtfGuidError {}

impl Display for GdtfGuidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrong argument for GUID in GDTF. Format must be RFC 4122. The format is XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX where XX is a byte in hex in UTF8 format!")
    }
}

impl From<Utf8Error> for GdtfGuidError {
    fn from(_: Utf8Error) -> Self {
        GdtfGuidError {}
    }
}

impl Error for GdtfGuidError {}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::guid::Guid as T;

    #[test]
    fn test_dummy() {
        assert_eq!(T([0; 16]), T::dummy());
    }

    #[test]
    fn test_is_byte_one_at_index() {
        assert_eq!(T::is_byte_one_at_index(0b1111_1111, 0).unwrap(), true);
        assert_eq!(T::is_byte_one_at_index(0b0111_1111, 0).unwrap(), false);

        assert_eq!(T::is_byte_one_at_index(0b1111_1111, 1).unwrap(), true);
        assert_eq!(T::is_byte_one_at_index(0b1011_1111, 1).unwrap(), false);

        assert_eq!(T::is_byte_one_at_index(0b1111_1111, 2).unwrap(), true);
        assert_eq!(T::is_byte_one_at_index(0b1101_1111, 2).unwrap(), false);

        assert_eq!(T::is_byte_one_at_index(0b1111_1111, 3).unwrap(), true);
        assert_eq!(T::is_byte_one_at_index(0b1110_1111, 3).unwrap(), false);

        assert_eq!(T::is_byte_one_at_index(0b1111_1111, 4).unwrap(), true);
        assert_eq!(T::is_byte_one_at_index(0b1111_0111, 4).unwrap(), false);

        assert_eq!(T::is_byte_one_at_index(0b1111_1111, 5).unwrap(), true);
        assert_eq!(T::is_byte_one_at_index(0b1111_1011, 5).unwrap(), false);

        assert_eq!(T::is_byte_one_at_index(0b1111_1111, 6).unwrap(), true);
        assert_eq!(T::is_byte_one_at_index(0b1111_1101, 6).unwrap(), false);

        assert_eq!(T::is_byte_one_at_index(0b1111_1111, 7).unwrap(), true);
        assert_eq!(T::is_byte_one_at_index(0b1111_1110, 7).unwrap(), false);

        assert!(T::is_byte_one_at_index(0b1111_1111, 8).is_err());
    }

    #[test]
    fn test_hexcharbyte_to_halfbyte() {
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_0_AS_BYTE).unwrap(), 0);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_1_AS_BYTE).unwrap(), 1);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_2_AS_BYTE).unwrap(), 2);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_3_AS_BYTE).unwrap(), 3);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_4_AS_BYTE).unwrap(), 4);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_5_AS_BYTE).unwrap(), 5);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_6_AS_BYTE).unwrap(), 6);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_7_AS_BYTE).unwrap(), 7);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_8_AS_BYTE).unwrap(), 8);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_9_AS_BYTE).unwrap(), 9);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_A_AS_BYTE).unwrap(), 10);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_B_AS_BYTE).unwrap(), 11);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_C_AS_BYTE).unwrap(), 12);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_D_AS_BYTE).unwrap(), 13);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_E_AS_BYTE).unwrap(), 14);
        assert_eq!(T::hexcharbyte_to_halfbyte(CHAR_F_AS_BYTE).unwrap(), 15);

        assert!(T::hexcharbyte_to_halfbyte(CHAR_SLASH_AS_BYTE).is_err());
        assert!(T::hexcharbyte_to_halfbyte(CHAR_SEMICOLON_AS_BYTE).is_err());
        assert!(T::hexcharbyte_to_halfbyte(CHAR_OPEN_AS_BYTE).is_err());
        assert!(T::hexcharbyte_to_halfbyte(CHAR_EQUALS_AS_BYTE).is_err());
        assert!(T::hexcharbyte_to_halfbyte(CHAR_CLOSE_AS_BYTE).is_err());
        assert!(T::hexcharbyte_to_halfbyte(CHAR_QUESTIONMARK_AS_BYTE).is_err());
        assert!(T::hexcharbyte_to_halfbyte(CHAR_G_AS_BYTE).is_err());
    }

    #[test]
    fn test_halfbyte_to_hexcharbyte() {
        assert_eq!(T::halfbyte_to_hexcharbyte(0).unwrap(), CHAR_0_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(1).unwrap(), CHAR_1_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(2).unwrap(), CHAR_2_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(3).unwrap(), CHAR_3_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(4).unwrap(), CHAR_4_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(5).unwrap(), CHAR_5_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(6).unwrap(), CHAR_6_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(7).unwrap(), CHAR_7_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(8).unwrap(), CHAR_8_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(9).unwrap(), CHAR_9_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(10).unwrap(), CHAR_A_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(11).unwrap(), CHAR_B_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(12).unwrap(), CHAR_C_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(13).unwrap(), CHAR_D_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(14).unwrap(), CHAR_E_AS_BYTE);
        assert_eq!(T::halfbyte_to_hexcharbyte(15).unwrap(), CHAR_F_AS_BYTE);

        assert!(T::halfbyte_to_hexcharbyte(16).is_err());
        assert!(T::halfbyte_to_hexcharbyte(17).is_err());
        assert!(T::halfbyte_to_hexcharbyte(18).is_err());
    }

    #[test]
    fn test_shift_byte_lower_to_upper() {
        assert_eq!(T::shift_byte_lower_to_upper(0x0).unwrap(), 0);
        assert_eq!(T::shift_byte_lower_to_upper(0x1).unwrap(), 16);
        assert_eq!(T::shift_byte_lower_to_upper(0x2).unwrap(), 32);
        assert_eq!(T::shift_byte_lower_to_upper(0x3).unwrap(), 48);
        assert_eq!(T::shift_byte_lower_to_upper(0x4).unwrap(), 64);
        assert_eq!(T::shift_byte_lower_to_upper(0x5).unwrap(), 80);
        assert_eq!(T::shift_byte_lower_to_upper(0x6).unwrap(), 96);
        assert_eq!(T::shift_byte_lower_to_upper(0x7).unwrap(), 112);
        assert_eq!(T::shift_byte_lower_to_upper(0x8).unwrap(), 128);
        assert_eq!(T::shift_byte_lower_to_upper(0x9).unwrap(), 144);
        assert_eq!(T::shift_byte_lower_to_upper(0xA).unwrap(), 160);
        assert_eq!(T::shift_byte_lower_to_upper(0xB).unwrap(), 176);
        assert_eq!(T::shift_byte_lower_to_upper(0xC).unwrap(), 192);
        assert_eq!(T::shift_byte_lower_to_upper(0xD).unwrap(), 208);
        assert_eq!(T::shift_byte_lower_to_upper(0xE).unwrap(), 224);
        assert_eq!(T::shift_byte_lower_to_upper(0xF).unwrap(), 240);
    }

    #[test]
    fn test_get_upper_halfbyte() {
        assert_eq!(T::get_upper_halfbyte(0x00).unwrap(), 0);
        assert_eq!(T::get_upper_halfbyte(0x10).unwrap(), 1);
        assert_eq!(T::get_upper_halfbyte(0x20).unwrap(), 2);
        assert_eq!(T::get_upper_halfbyte(0x30).unwrap(), 3);
        assert_eq!(T::get_upper_halfbyte(0x40).unwrap(), 4);
        assert_eq!(T::get_upper_halfbyte(0x50).unwrap(), 5);
        assert_eq!(T::get_upper_halfbyte(0x60).unwrap(), 6);
        assert_eq!(T::get_upper_halfbyte(0x6A).unwrap(), 6);
        assert_eq!(T::get_upper_halfbyte(0x70).unwrap(), 7);
        assert_eq!(T::get_upper_halfbyte(0x80).unwrap(), 8);
        assert_eq!(T::get_upper_halfbyte(0x90).unwrap(), 9);
        assert_eq!(T::get_upper_halfbyte(0x94).unwrap(), 9);
        assert_eq!(T::get_upper_halfbyte(0xA0).unwrap(), 10);
        assert_eq!(T::get_upper_halfbyte(0xB0).unwrap(), 11);
        assert_eq!(T::get_upper_halfbyte(0xC0).unwrap(), 12);
        assert_eq!(T::get_upper_halfbyte(0xCF).unwrap(), 12);
        assert_eq!(T::get_upper_halfbyte(0xD0).unwrap(), 13);
        assert_eq!(T::get_upper_halfbyte(0xE0).unwrap(), 14);
        assert_eq!(T::get_upper_halfbyte(0xF0).unwrap(), 15);
        assert_eq!(T::get_upper_halfbyte(0xF3).unwrap(), 15);
    }

    #[test]
    fn test_join_two_halfbytes() {
        assert_eq!(T::join_two_halfbytes(0x0, 0xF).unwrap(), 0x0F);
        assert_eq!(T::join_two_halfbytes(0xF, 0x0).unwrap(), 0xF0);
        assert_eq!(T::join_two_halfbytes(0xF, 0xA).unwrap(), 0xFA);
        assert_eq!(T::join_two_halfbytes(0x3, 0x4).unwrap(), 0x34);
        assert_eq!(T::join_two_halfbytes(0x3, 0x0).unwrap(), 0x30);
    }

    #[test]
    fn test_split_into_two_halfbytes() {
        assert_eq!(T::split_into_two_halfbytes(0xF0).unwrap(), (0xF, 0x0));
        assert_eq!(T::split_into_two_halfbytes(0xFF).unwrap(), (0xF, 0xF));
        assert_eq!(T::split_into_two_halfbytes(0x32).unwrap(), (0x3, 0x2));
        assert_eq!(T::split_into_two_halfbytes(0x45).unwrap(), (0x4, 0x5));
        assert_eq!(T::split_into_two_halfbytes(0xFE).unwrap(), (0xF, 0xE));
    }

    #[test]
    fn test_get_lower_halfbyte() {
        assert_eq!(T::get_lower_halfbyte(0xF).unwrap(), 0xF);

        assert_eq!(T::get_lower_halfbyte(0x3).unwrap(), 0x3);
        assert_eq!(T::get_lower_halfbyte(0xF3).unwrap(), 0x3);
        assert_eq!(T::get_lower_halfbyte(0x3F).unwrap(), 0xF);
        assert_eq!(T::get_lower_halfbyte(0x38).unwrap(), 0x8);
        assert_eq!(T::get_lower_halfbyte(0x0A).unwrap(), 0xA);
        assert_eq!(T::get_lower_halfbyte(0x3A).unwrap(), 0xA);
    }

    #[test]
    fn test_hexcharbytes_to_byte() {
        assert_eq!(
            T::hexcharbytes_to_byte((*"0".as_bytes())[0], (*"0".as_bytes())[0]).unwrap(),
            0_u8
        );
        assert_eq!(
            T::hexcharbytes_to_byte((*"A".as_bytes())[0], (*"0".as_bytes())[0]).unwrap(),
            160_u8
        );
        assert_eq!(
            T::hexcharbytes_to_byte((*"0".as_bytes())[0], (*"7".as_bytes())[0]).unwrap(),
            7_u8
        );
        assert_eq!(
            T::hexcharbytes_to_byte((*"7".as_bytes())[0], (*"0".as_bytes())[0]).unwrap(),
            112_u8
        );
        assert_eq!(
            T::hexcharbytes_to_byte((*"7".as_bytes())[0], (*"7".as_bytes())[0]).unwrap(),
            119_u8
        );
        assert_eq!(
            T::hexcharbytes_to_byte((*"7".as_bytes())[0], (*"C".as_bytes())[0]).unwrap(),
            124_u8
        );

        assert!(T::hexcharbytes_to_byte((*"/".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(T::hexcharbytes_to_byte((*";".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(T::hexcharbytes_to_byte((*"<".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(T::hexcharbytes_to_byte((*"=".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(T::hexcharbytes_to_byte((*"F".as_bytes())[0], (*"=".as_bytes())[0]).is_err());
        assert!(T::hexcharbytes_to_byte((*"?".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(T::hexcharbytes_to_byte((*"G".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
    }

    #[test]
    fn test_new_from_str() {
        assert_eq!(
            T([48, 142, 168, 125, 113, 100, 66, 222, 129, 6, 166, 210, 115, 245, 122, 81]),
            T::new_from_str("308EA87D-7164-42DE-8106-A6D273F57A51").unwrap()
        );
        assert!(T::new_from_str("Something invalid").is_err());
        assert!(T::new_from_str("308EA87D/7164-42DE-8106-A6D273F57A51").is_err());
        assert!(T::new_from_str("308EA87D-7164_42DE-8106-A6D273F57A51").is_err());
        assert!(T::new_from_str("308EA87D-7164-42DE/8106-A6D273F57A51").is_err());
        assert!(T::new_from_str("308EA87D-7164-42DE-8106_A6D273F57A51").is_err());
    }

    #[test]
    fn test_new_from_attr_borrowed() {
        assert_eq!(
            T([48, 142, 168, 125, 113, 100, 66, 222, 129, 6, 166, 210, 115, 245, 122, 81]),
            T::new_from_attr(testdata::to_attr_borrowed(
                b"308EA87D-7164-42DE-8106-A6D273F57A51"
            ))
            .unwrap()
        );
        assert!(T::new_from_attr(testdata::to_attr_borrowed(b"Something invalid")).is_err());
    }

    #[test]
    fn test_new_from_attr_owned() {
        assert_eq!(
            T([48, 142, 168, 125, 113, 100, 66, 222, 129, 6, 166, 210, 115, 245, 122, 81]),
            T::new_from_attr(testdata::to_attr_owned(
                b"308EA87D-7164-42DE-8106-A6D273F57A51"
            ))
            .unwrap()
        );
        assert!(T::new_from_attr(testdata::to_attr_owned(b"Something invalid")).is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!(
                "{}",
                T([48, 142, 168, 125, 113, 100, 66, 222, 129, 6, 166, 210, 115, 245, 122, 81])
            ),
            "308EA87D-7164-42DE-8106-A6D273F57A51"
        );
    }

    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_0_AS_BYTE: u8 = "0".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_1_AS_BYTE: u8 = "1".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_2_AS_BYTE: u8 = "2".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_3_AS_BYTE: u8 = "3".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_4_AS_BYTE: u8 = "4".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_5_AS_BYTE: u8 = "5".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_6_AS_BYTE: u8 = "6".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_7_AS_BYTE: u8 = "7".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_8_AS_BYTE: u8 = "8".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_9_AS_BYTE: u8 = "9".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_A_AS_BYTE: u8 = "A".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_B_AS_BYTE: u8 = "B".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_C_AS_BYTE: u8 = "C".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_D_AS_BYTE: u8 = "D".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_E_AS_BYTE: u8 = "E".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_F_AS_BYTE: u8 = "F".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_SLASH_AS_BYTE: u8 = "/".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_SEMICOLON_AS_BYTE: u8 = ";".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_OPEN_AS_BYTE: u8 = "<".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_EQUALS_AS_BYTE: u8 = "=".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_CLOSE_AS_BYTE: u8 = ">".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_QUESTIONMARK_AS_BYTE: u8 = "?".as_bytes()[0];
    #[cfg(test)]
    ///Deparsed by Rust is slower but needed in this context so code is not tested with itself
    const CHAR_G_AS_BYTE: u8 = "G".as_bytes()[0];
}
