//! Module for the unit DataVersion used in GDTF
use std::fmt::{Display, Formatter};

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

impl From<&str> for DataVersion {
    fn from(value: &str) -> Self {
        let mut value = value.split('.');
        match value.next() {
            Some(major) => {
                match major.parse::<i32>() {
                    Ok(major) => {
                        match value.next() {
                            Some(minor) => {
                                match minor.parse::<i32>() {
                                    Ok(minor) => {
                                        match major {
                                            1 => {
                                                match minor {
                                                    0 => DataVersion::Version1_0,
                                                    1 => DataVersion::Version1_1,
                                                    _ => DataVersion::Unknown
                                                }
                                            }
                                            _ => DataVersion::Unknown
                                        }
                                    }
                                    _ => DataVersion::Unknown
                                }
                            }
                            _ => DataVersion::Unknown
                        }
                    }
                    _ => DataVersion::Unknown
                }
            }
            _ => DataVersion::Unknown
        }
    }
}

impl PartialEq for DataVersion {
    fn eq(&self, other: &Self) -> bool {
        match self {
            DataVersion::Version1_0 => match other {
                DataVersion::Version1_0 => true,
                _ => false
            }
            DataVersion::Version1_1 => match other {
                DataVersion::Version1_1 => true,
                _ => false
            }
            DataVersion::Unknown => match other {
                DataVersion::Unknown => true,
                _ => false
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::data_version::DataVersion;

    #[test]
    fn test_valid_1_0() {
        assert_eq!(
            DataVersion::Version1_0,
            DataVersion::try_from("1.0").unwrap()
        );
    }

    #[test]
    fn test_valid_1_1() {
        assert_eq!(
            DataVersion::Version1_1,
            DataVersion::try_from("1.1").unwrap()
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(
            DataVersion::Unknown,
            DataVersion::try_from("something invalid").unwrap()
        );
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
}