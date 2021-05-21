//! Module for the unit GUID used in GDTF
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

use crate::utils::errors::GdtfError;

///GUID representation used in GDTF
#[derive(Debug)]
pub struct GUID {
    ///The string value of the GUID
    value: String,
}

impl Default for GUID {
    fn default() -> Self {
        Self { value: String::new() }
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

///Interprets a UTF8 formated hex charbyte to a halfbyte
fn hexcharbyte_to_halfbyte(c: u8) -> Result<u8, GDTFGUIDError> {
    match c {
        0x30 => Ok(0),
        0x31 => Ok(1),
        0x32 => Ok(2),
        0x33 => Ok(3),
        0x34 => Ok(4),
        0x35 => Ok(5),
        0x36 => Ok(6),
        0x37 => Ok(7),
        0x38 => Ok(8),
        0x39 => Ok(9),
        0x41 => Ok(10),
        0x42 => Ok(11),
        0x43 => Ok(12),
        0x44 => Ok(13),
        0x45 => Ok(14),
        0x46 => Ok(15),
        _ => Err(GDTFGUIDError {})
    }
}

///Tells if the bit of a byte is 1 at a specific index
fn is_byte_one_at_index(byte: u8, index: u8) -> Result<bool, GDTFGUIDError> {
    match index {
        0 => Ok(0b1000_0000_u8 & byte == 0b1000_0000_u8),
        1 => Ok(0b0100_0000_u8 & byte == 0b0100_0000_u8),
        2 => Ok(0b0010_0000_u8 & byte == 0b0010_0000_u8),
        3 => Ok(0b0001_0000_u8 & byte == 0b0001_0000_u8),
        4 => Ok(0b0000_1000_u8 & byte == 0b0000_1000_u8),
        5 => Ok(0b0000_0100_u8 & byte == 0b0000_0100_u8),
        6 => Ok(0b0000_0010_u8 & byte == 0b0000_0010_u8),
        7 => Ok(0b0000_0001_u8 & byte == 0b0000_0001_u8),
        _ => Err(GDTFGUIDError {})
    }
}

//shifts bytes from lower to upper. 0b0000_0100 -> 0b0100_0000
fn shift_byte_lower_to_upper(byte: u8) -> Result<u8, GDTFGUIDError> {
    let s1 = if is_byte_one_at_index(byte, 4)? { 0b1000_0000_u8 } else { 0 };
    let s2 = if is_byte_one_at_index(byte, 5)? { 0b0100_0000_u8 } else { 0 };
    let s3 = if is_byte_one_at_index(byte, 6)? { 0b0010_0000_u8 } else { 0 };
    let s4 = if is_byte_one_at_index(byte, 7)? { 0b0001_0000_u8 } else { 0 };
    Ok(s1 + s2 + s3 + s4)
}

//joins two halfbytes to one byte. 0b0000_0101 + 0b0000_0001 -> 0b0101_0001
fn join_two_halfbytes(first_half: u8, second_half: u8) -> Result<u8, GDTFGUIDError> {
    Ok(shift_byte_lower_to_upper(first_half)? + second_half)
}

///Interprets two UTF8 formated hex charbyte to a byte
fn hexcharbytes_to_byte(c1: u8, c2: u8) -> Result<u8, GDTFGUIDError> {
    Ok(join_two_halfbytes(hexcharbyte_to_halfbyte(c1)?, hexcharbyte_to_halfbyte(c2)?)?)
}

#[derive(Debug)]
pub struct GDTFGUIDError {}

impl Display for GDTFGUIDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrong argument for GUID in GDTF. Format must be RFC 4122. The format is XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX where XX is a byte in hex in UTF8 format!")
    }
}

impl Error for GDTFGUIDError {}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::guid::{GUID, hexcharbyte_to_halfbyte, hexcharbytes_to_byte, is_byte_one_at_index, join_two_halfbytes, shift_byte_lower_to_upper};

    #[test]
    fn test_is_byte_one_at_index() {
        assert_eq!(is_byte_one_at_index(0b1111_1111, 0).unwrap(), true);
        assert_eq!(is_byte_one_at_index(0b0111_1111, 0).unwrap(), false);

        assert_eq!(is_byte_one_at_index(0b1111_1111, 1).unwrap(), true);
        assert_eq!(is_byte_one_at_index(0b1011_1111, 1).unwrap(), false);

        assert_eq!(is_byte_one_at_index(0b1111_1111, 2).unwrap(), true);
        assert_eq!(is_byte_one_at_index(0b1101_1111, 2).unwrap(), false);

        assert_eq!(is_byte_one_at_index(0b1111_1111, 3).unwrap(), true);
        assert_eq!(is_byte_one_at_index(0b1110_1111, 3).unwrap(), false);

        assert_eq!(is_byte_one_at_index(0b1111_1111, 4).unwrap(), true);
        assert_eq!(is_byte_one_at_index(0b1111_0111, 4).unwrap(), false);

        assert_eq!(is_byte_one_at_index(0b1111_1111, 5).unwrap(), true);
        assert_eq!(is_byte_one_at_index(0b1111_1011, 5).unwrap(), false);

        assert_eq!(is_byte_one_at_index(0b1111_1111, 6).unwrap(), true);
        assert_eq!(is_byte_one_at_index(0b1111_1101, 6).unwrap(), false);

        assert_eq!(is_byte_one_at_index(0b1111_1111, 7).unwrap(), true);
        assert_eq!(is_byte_one_at_index(0b1111_1110, 7).unwrap(), false);

        assert!(is_byte_one_at_index(0b1111_1111, 8).is_err())
    }

    #[test]
    fn test_hexcharbyte_to_halfbyte() {
        assert_eq!(hexcharbyte_to_halfbyte((*"0".as_bytes())[0]).unwrap(), 0);
        assert_eq!(hexcharbyte_to_halfbyte((*"1".as_bytes())[0]).unwrap(), 1);
        assert_eq!(hexcharbyte_to_halfbyte((*"2".as_bytes())[0]).unwrap(), 2);
        assert_eq!(hexcharbyte_to_halfbyte((*"3".as_bytes())[0]).unwrap(), 3);
        assert_eq!(hexcharbyte_to_halfbyte((*"4".as_bytes())[0]).unwrap(), 4);
        assert_eq!(hexcharbyte_to_halfbyte((*"5".as_bytes())[0]).unwrap(), 5);
        assert_eq!(hexcharbyte_to_halfbyte((*"6".as_bytes())[0]).unwrap(), 6);
        assert_eq!(hexcharbyte_to_halfbyte((*"7".as_bytes())[0]).unwrap(), 7);
        assert_eq!(hexcharbyte_to_halfbyte((*"8".as_bytes())[0]).unwrap(), 8);
        assert_eq!(hexcharbyte_to_halfbyte((*"9".as_bytes())[0]).unwrap(), 9);
        assert_eq!(hexcharbyte_to_halfbyte((*"A".as_bytes())[0]).unwrap(), 10);
        assert_eq!(hexcharbyte_to_halfbyte((*"B".as_bytes())[0]).unwrap(), 11);
        assert_eq!(hexcharbyte_to_halfbyte((*"C".as_bytes())[0]).unwrap(), 12);
        assert_eq!(hexcharbyte_to_halfbyte((*"D".as_bytes())[0]).unwrap(), 13);
        assert_eq!(hexcharbyte_to_halfbyte((*"E".as_bytes())[0]).unwrap(), 14);
        assert_eq!(hexcharbyte_to_halfbyte((*"F".as_bytes())[0]).unwrap(), 15);

        assert!(hexcharbyte_to_halfbyte((*"/".as_bytes())[0]).is_err());
        assert!(hexcharbyte_to_halfbyte((*";".as_bytes())[0]).is_err());
        assert!(hexcharbyte_to_halfbyte((*"<".as_bytes())[0]).is_err());
        assert!(hexcharbyte_to_halfbyte((*"=".as_bytes())[0]).is_err());
        assert!(hexcharbyte_to_halfbyte((*">".as_bytes())[0]).is_err());
        assert!(hexcharbyte_to_halfbyte((*"?".as_bytes())[0]).is_err());
        assert!(hexcharbyte_to_halfbyte((*"G".as_bytes())[0]).is_err());
    }

    #[test]
    fn test_shift_byte_lower_to_upper() {
        assert_eq!(shift_byte_lower_to_upper(0x0).unwrap(), 0);
        assert_eq!(shift_byte_lower_to_upper(0x1).unwrap(), 16);
        assert_eq!(shift_byte_lower_to_upper(0x2).unwrap(), 32);
        assert_eq!(shift_byte_lower_to_upper(0x3).unwrap(), 48);
        assert_eq!(shift_byte_lower_to_upper(0x4).unwrap(), 64);
        assert_eq!(shift_byte_lower_to_upper(0x5).unwrap(), 80);
        assert_eq!(shift_byte_lower_to_upper(0x6).unwrap(), 96);
        assert_eq!(shift_byte_lower_to_upper(0x7).unwrap(), 112);
        assert_eq!(shift_byte_lower_to_upper(0x8).unwrap(), 128);
        assert_eq!(shift_byte_lower_to_upper(0x9).unwrap(), 144);
        assert_eq!(shift_byte_lower_to_upper(0xA).unwrap(), 160);
        assert_eq!(shift_byte_lower_to_upper(0xB).unwrap(), 176);
        assert_eq!(shift_byte_lower_to_upper(0xC).unwrap(), 192);
        assert_eq!(shift_byte_lower_to_upper(0xD).unwrap(), 208);
        assert_eq!(shift_byte_lower_to_upper(0xE).unwrap(), 224);
        assert_eq!(shift_byte_lower_to_upper(0xF).unwrap(), 240);
    }

    #[test]
    fn test_join_two_halfbytes() {
        assert_eq!(join_two_halfbytes(0x0, 0xF).unwrap(), 0x0F);
        assert_eq!(join_two_halfbytes(0xF, 0x0).unwrap(), 0xF0);
        assert_eq!(join_two_halfbytes(0xF, 0xA).unwrap(), 0xFA);
        assert_eq!(join_two_halfbytes(0x3, 0x4).unwrap(), 0x34);
        assert_eq!(join_two_halfbytes(0x3, 0x0).unwrap(), 0x30);
    }


    #[test]
    fn test_hexcharbytes_to_byte() {
        assert_eq!(hexcharbytes_to_byte((*"0".as_bytes())[0], (*"0".as_bytes())[0]).unwrap(), 0_u8);
        assert_eq!(hexcharbytes_to_byte((*"A".as_bytes())[0], (*"0".as_bytes())[0]).unwrap(), 160_u8);
        assert_eq!(hexcharbytes_to_byte((*"0".as_bytes())[0], (*"7".as_bytes())[0]).unwrap(), 7_u8);
        assert_eq!(hexcharbytes_to_byte((*"7".as_bytes())[0], (*"0".as_bytes())[0]).unwrap(), 112_u8);
        assert_eq!(hexcharbytes_to_byte((*"7".as_bytes())[0], (*"7".as_bytes())[0]).unwrap(), 119_u8);
        assert_eq!(hexcharbytes_to_byte((*"7".as_bytes())[0], (*"C".as_bytes())[0]).unwrap(), 124_u8);

        assert!(hexcharbytes_to_byte((*"/".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(hexcharbytes_to_byte((*";".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(hexcharbytes_to_byte((*"<".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(hexcharbytes_to_byte((*"=".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(hexcharbytes_to_byte((*"F".as_bytes())[0], (*"=".as_bytes())[0]).is_err());
        assert!(hexcharbytes_to_byte((*"?".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
        assert!(hexcharbytes_to_byte((*"G".as_bytes())[0], (*"0".as_bytes())[0]).is_err());
    }

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