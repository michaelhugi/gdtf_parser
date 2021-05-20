//! Module for the unit DataVersion used in GDTF
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

use quick_xml::events::attributes::Attribute;

///The DataVersion attribute defines the minimal version of compatibility. The Version format is “Major.Minor”, where major and minor is Uint with size 1 byte
#[derive(Debug)]
pub enum DataVersion {
    ///Enum for GDTF Version 1.0
    Version1_0,
    ///Enum for GDTF Version 1.1
    Version1_1,
    ///Enum for other GDTF Version (most likely not supported yet in this library)
    Unknown,
}

impl Display for DataVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataVersion::Version1_0 => write!(f, "1.0"),
            DataVersion::Version1_1 => write!(f, "1.1"),
            DataVersion::Unknown => write!(f, "Unknown"),
        }
    }
}

///Deparses Version from Attribute safely. In case of error it will return Unknown
impl From<Attribute<'_>> for DataVersion {
    ///Deparses Version from Attribute safely. In case of error it will return Unknown
    fn from(attr: Attribute) -> Self {
        let value = std::str::from_utf8(attr.value.borrow()).unwrap_or_else(|_| "");
        let mut value = value.split('.');
        let major = value.next().unwrap_or_else(|| "").parse::<i32>().unwrap_or_else(|_| -1);
        let minor = value.next().unwrap_or_else(|| "").parse::<i32>().unwrap_or_else(|_| -1);
        if value.next().is_some() { return DataVersion::Unknown; }

        match (major, minor) {
            (1, 0) => DataVersion::Version1_0,
            (1, 1) => DataVersion::Version1_1,
            (_, _) => DataVersion::Unknown
        }
    }
}

///Compares two versions. Returns false in case one is Unknown no matter if the other is unknown
impl PartialEq for DataVersion {
    fn eq(&self, other: &Self) -> bool {
        use DataVersion::*;
        match self {
            Version1_0 => if let Version1_0 = other { true } else { false }
            Version1_1 => if let Version1_1 = other { true } else { false }
            Unknown => false
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::data_version::DataVersion;

    #[test]
    fn test_parse_attr_from_1_0_owned() {
        assert_eq!(
            DataVersion::Version1_0,
            testdata::to_attr_owned(b"1.0").into()
        );
    }

    #[test]
    fn test_parse_attr_from_1_0_borrowed() {
        assert_eq!(
            DataVersion::Version1_0,
            testdata::to_attr_borrowed(b"1.0").into()
        );
    }

    #[test]
    fn test_parse_attr_from_1_1_owned() {
        assert_eq!(
            DataVersion::Version1_1,
            testdata::to_attr_owned(b"1.1").into()
        );
    }

    #[test]
    fn test_parse_attr_from_1_1_borrowed() {
        assert_eq!(
            DataVersion::Version1_1,
            testdata::to_attr_borrowed(b"1.1").into()
        );
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_1_owned() {
        if let DataVersion::Unknown = testdata::to_attr_owned(b"something invalid").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_1_borrowed() {
        if let DataVersion::Unknown = testdata::to_attr_borrowed(b"something invalid").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //Test must be rewritten to 1.3 after 1.2 is introduced
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_2_owned() {
        if let DataVersion::Unknown = testdata::to_attr_owned(b"1.2").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //Test must be rewritten to 1.3 after 1.2 is introduced
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_2_borrowed() {
        if let DataVersion::Unknown = testdata::to_attr_borrowed(b"1.2").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_3_owned() {
        if let DataVersion::Unknown = testdata::to_attr_owned(b"1.1.2").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_3_borrowed() {
        if let DataVersion::Unknown = testdata::to_attr_borrowed(b"1.1.2").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_4_owned() {
        if let DataVersion::Unknown = testdata::to_attr_owned(b"1.1.").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_4_borrowed() {
        if let DataVersion::Unknown = testdata::to_attr_borrowed(b"1.1.").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_5_owned() {
        if let DataVersion::Unknown = testdata::to_attr_owned(b".1.1").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_5_borrowed() {
        if let DataVersion::Unknown = testdata::to_attr_borrowed(b".1.1").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_6_owned() {
        if let DataVersion::Unknown = testdata::to_attr_owned(b".1").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_6_borrowed() {
        if let DataVersion::Unknown = testdata::to_attr_borrowed(b".1").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_7_owned() {
        if let DataVersion::Unknown = testdata::to_attr_owned(b"1.").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    //partial eq can't be used in case one is unknown
    fn test_parse_attr_invalid_7_borrowed() {
        if let DataVersion::Unknown = testdata::to_attr_borrowed(b"1.").into() {} else {
            panic!("Invalid Version not returned as Unknown");
        }
    }

    #[test]
    fn test_display_unknown() {
        assert_eq!(format!("{}", DataVersion::Unknown), "Unknown");
    }

    #[test]
    fn test_display_1_0() {
        assert_eq!(format!("{}", DataVersion::Version1_0), "1.0");
    }

    #[test]
    fn test_display_1_1() {
        assert_eq!(format!("{}", DataVersion::Version1_1), "1.1");
    }

    #[test]
    fn test_partial_eq_1_1() {
        assert_eq!(DataVersion::Version1_1, DataVersion::Version1_1)
    }

    #[test]
    fn test_partial_eq_1_0() {
        assert_eq!(DataVersion::Version1_0, DataVersion::Version1_0)
    }

    #[test]
    fn test_partial_eq_unknown() {
        assert_ne!(DataVersion::Unknown, DataVersion::Unknown)
    }

    #[test]
    fn test_partial_ne_1_1() {
        assert_ne!(DataVersion::Version1_1, DataVersion::Version1_0)
    }

    #[test]
    fn test_partial_ne_1_0() {
        assert_ne!(DataVersion::Version1_0, DataVersion::Unknown)
    }
}