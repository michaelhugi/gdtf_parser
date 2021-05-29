//! Module for the unit DataVersion used in GDTF
use std::borrow::Borrow;

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;

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
impl PartialEqAllowEmpty for DataVersion {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        use DataVersion::*;
        match self {
            Version1_0 => if let Version1_0 = other { true } else {
                Self::print_structs_not_equal(self, other, log)
            }
            Version1_1 => if let Version1_1 = other { true } else {
                Self::print_structs_not_equal(self, other, log)
            }
            Unknown => if let Unknown = other { true } else {
                Self::print_structs_not_equal(self, other, log)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::data_version::DataVersion;

    #[test]
    fn test_from_attr_owned() {
        DataVersion::Version1_0.assert_eq_allow_empty(&testdata::to_attr_owned(b"1.0").into(), true);
        DataVersion::Version1_1.assert_eq_allow_empty(&testdata::to_attr_owned(b"1.1").into(), true);
        //Test must be rewritten when 1.2 is introduced
        DataVersion::Unknown.assert_eq_allow_empty(&testdata::to_attr_owned(b"1.2").into(), true);
        DataVersion::Unknown.assert_eq_allow_empty(&testdata::to_attr_owned(b"something invalid").into(), true);
        DataVersion::Unknown.assert_eq_allow_empty(&testdata::to_attr_owned(b"1.1.2").into(), true);
        DataVersion::Unknown.assert_eq_allow_empty(&testdata::to_attr_owned(b"1.1.").into(), true);
        DataVersion::Unknown.assert_eq_allow_empty(&testdata::to_attr_owned(b".1.1").into(), true);
        DataVersion::Unknown.assert_eq_allow_empty(&testdata::to_attr_owned(b".1").into(), true);
        DataVersion::Unknown.assert_eq_allow_empty(&testdata::to_attr_owned(b"1.").into(), true);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(DataVersion::Version1_1, DataVersion::Version1_1);
        assert_eq!(DataVersion::Version1_0, DataVersion::Version1_0);
        assert_ne!(DataVersion::Unknown, DataVersion::Unknown);
        assert_ne!(DataVersion::Version1_1, DataVersion::Version1_0);
        assert_ne!(DataVersion::Version1_0, DataVersion::Unknown);
    }

    #[test]
    fn test_partial_eq_allow_empty() {
        DataVersion::Version1_1.assert_eq_allow_empty(&DataVersion::Version1_1, true);
        DataVersion::Version1_0.assert_eq_allow_empty(&DataVersion::Version1_0, true);
        DataVersion::Unknown.assert_eq_allow_empty(&DataVersion::Unknown, true);
        DataVersion::Version1_1.assert_ne_allow_empty(&DataVersion::Version1_0);
        DataVersion::Version1_0.assert_ne_allow_empty(&DataVersion::Unknown);
    }
}