//! Module for the unit DataVersion used in GDTF
use std::borrow::Borrow;

use quick_xml::events::attributes::Attribute;

///The DataVersion attribute defines the minimal version of compatibility. The Version format is “Major.Minor”, where major and minor is Uint with size 1 byte
#[derive(Debug, PartialEq, Clone)]
pub enum DataVersion {
    ///Enum for GDTF Version 1.0
    Version1_0,
    ///Enum for GDTF Version 1.1
    Version1_1,
    ///Enum for other GDTF Version (most likely not supported yet in this library)
    Unknown,
}


///Deparses Version from Attribute safely. In case of error it will return Unknown
impl From<Attribute<'_>> for DataVersion {
    ///Deparses Version from Attribute safely. In case of error it will return Unknown
    fn from(attr: Attribute) -> Self {
        let value = std::str::from_utf8(attr.value.borrow()).unwrap_or("");
        let mut value = value.split('.');
        let major = value.next().unwrap_or("").parse::<i32>().unwrap_or(-1);
        let minor = value.next().unwrap_or("").parse::<i32>().unwrap_or(-1);
        if value.next().is_some() { return DataVersion::Unknown; }

        match (major, minor) {
            (1, 0) => DataVersion::Version1_0,
            (1, 1) => DataVersion::Version1_1,
            (_, _) => DataVersion::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::data_version::DataVersion;

    #[test]
    fn test_from_attr_owned() {
        assert_eq!(DataVersion::Version1_0, testdata::to_attr_owned(b"1.0").into());
        assert_eq!(DataVersion::Version1_1, testdata::to_attr_owned(b"1.1").into());
        //Test must be rewritten when 1.2 is introduced
        assert_eq!(DataVersion::Unknown, testdata::to_attr_owned(b"1.2").into());
        assert_eq!(DataVersion::Unknown, testdata::to_attr_owned(b"something invalid").into());
        assert_eq!(DataVersion::Unknown, testdata::to_attr_owned(b"1.1.2").into());
        assert_eq!(DataVersion::Unknown, testdata::to_attr_owned(b"1.1.").into());
        assert_eq!(DataVersion::Unknown, testdata::to_attr_owned(b".1.1").into());
        assert_eq!(DataVersion::Unknown, testdata::to_attr_owned(b".1").into());
        assert_eq!(DataVersion::Unknown, testdata::to_attr_owned(b"1.").into());
    }
}