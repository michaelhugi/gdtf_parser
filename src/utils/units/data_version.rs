//! Module for the unit DataVersion used in GDTF

use quick_xml::events::attributes::Attribute;

use crate::utils::deparse;

///The DataVersion attribute defines the minimal version of compatibility. The Version format is “Major.Minor”, where major and minor is Uint with size 1 byte
#[derive(Debug, PartialEq, Clone)]
pub enum GdtfDataVersion {
    ///Enum for GDTF Version 1.0
    Version1_0,
    ///Enum for GDTF Version 1.1
    Version1_1,
    ///Enum for other GDTF Version (most likely not supported yet in this library)
    ///Unknown contains original String
    Unknown(String),
}

impl GdtfDataVersion {
    ///Creates a dummy object to initiate if value is not yet defined
    /// ```rust
    /// use gdtf_parser::utils::units::data_version::GdtfDataVersion;
    /// assert_eq!(GdtfDataVersion::dummy(), GdtfDataVersion::Unknown("?".to_string()));
    /// ```
    pub fn dummy() -> Self {
        Self::Unknown("?".to_string())
    }

    ///Parses a string defined in gdtf-xml-description to GdtfDataVersion.
    /// ```rust
    /// use gdtf_parser::utils::units::data_version::GdtfDataVersion;
    /// assert_eq!(GdtfDataVersion::new_from_str("1.0"), GdtfDataVersion::Version1_0);
    /// assert_eq!(GdtfDataVersion::new_from_str("1.1"), GdtfDataVersion::Version1_1);
    /// assert_eq!(GdtfDataVersion::new_from_str("1.2"), GdtfDataVersion::Unknown("1.2".to_string()));
    /// assert_eq!(GdtfDataVersion::new_from_str("Something invalid"), GdtfDataVersion::Unknown("Something invalid".to_string()));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        let mut value = s.split('.');
        let major = value.next().unwrap_or("").parse::<i32>().unwrap_or(-1);
        let minor = value.next().unwrap_or("").parse::<i32>().unwrap_or(-1);
        if value.next().is_some() { return Self::Unknown(s.to_string()); }

        match (major, minor) {
            (1, 0) => Self::Version1_0,
            (1, 1) => Self::Version1_1,
            (_, _) => Self::Unknown(s.to_string())
        }
    }
    ///Parses a string defined in gdtf-xml-description to GdtfDataVersion.
    /// ```rust
    /// use gdtf_parser::utils::units::data_version::GdtfDataVersion;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(GdtfDataVersion::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"1.0")}), GdtfDataVersion::Version1_0);
    /// assert_eq!(GdtfDataVersion::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"1.1")}), GdtfDataVersion::Version1_1);
    /// assert_eq!(GdtfDataVersion::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"1.2")}), GdtfDataVersion::Unknown("1.2".to_string()));
    /// assert_eq!(GdtfDataVersion::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Something invalid")}), GdtfDataVersion::Unknown("Something invalid".to_string()));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::data_version::GdtfDataVersion as T;

    #[test]
    fn test_new_from_str() {
        assert_eq!(T::Version1_0, T::new_from_str("1.0"));
        assert_eq!(T::Version1_1, T::new_from_str("1.1"));
        //Test must be rewritten when 1.2 is introduced
        assert_eq!(T::Unknown("1.2".to_string()), T::new_from_str("1.2"));
        assert_eq!(T::Unknown("something invalid".to_string()), T::new_from_str("something invalid"));
        assert_eq!(T::Unknown("1.1.2".to_string()), T::new_from_str("1.1.2"));
        assert_eq!(T::Unknown("1.1.".to_string()), T::new_from_str("1.1."));
        assert_eq!(T::Unknown(".1.1".to_string()), T::new_from_str(".1.1"));
        assert_eq!(T::Unknown(".1".to_string()), T::new_from_str(".1"));
        assert_eq!(T::Unknown("1.".to_string()), T::new_from_str("1."));
    }

    #[test]
    fn test_new_from_attr_owned() {
        assert_eq!(T::Version1_0, T::new_from_attr(testdata::to_attr_owned(b"1.0")));
        assert_eq!(T::Version1_1, T::new_from_attr(testdata::to_attr_owned(b"1.1")));
        //Test must be rewritten when 1.2 is introduced
        assert_eq!(T::Unknown("1.2".to_string()), T::new_from_attr(testdata::to_attr_owned(b"1.2")));
        assert_eq!(T::Unknown("something invalid".to_string()), T::new_from_attr(testdata::to_attr_owned(b"something invalid")));
        assert_eq!(T::Unknown("1.1.2".to_string()), T::new_from_attr(testdata::to_attr_owned(b"1.1.2")));
        assert_eq!(T::Unknown("1.1.".to_string()), T::new_from_attr(testdata::to_attr_owned(b"1.1.")));
        assert_eq!(T::Unknown(".1.1".to_string()), T::new_from_attr(testdata::to_attr_owned(b".1.1")));
        assert_eq!(T::Unknown(".1".to_string()), T::new_from_attr(testdata::to_attr_owned(b".1")));
        assert_eq!(T::Unknown("1.".to_string()), T::new_from_attr(testdata::to_attr_owned(b"1.")));
    }

    #[test]
    fn test_new_from_attr_borrowed() {
        assert_eq!(T::Version1_0, T::new_from_attr(testdata::to_attr_borrowed(b"1.0")));
        assert_eq!(T::Version1_1, T::new_from_attr(testdata::to_attr_borrowed(b"1.1")));
        //Test must be rewritten when 1.2 is introduced
        assert_eq!(T::Unknown("1.2".to_string()), T::new_from_attr(testdata::to_attr_borrowed(b"1.2")));
        assert_eq!(T::Unknown("something invalid".to_string()), T::new_from_attr(testdata::to_attr_borrowed(b"something invalid")));
        assert_eq!(T::Unknown("1.1.2".to_string()), T::new_from_attr(testdata::to_attr_borrowed(b"1.1.2")));
        assert_eq!(T::Unknown("1.1.".to_string()), T::new_from_attr(testdata::to_attr_borrowed(b"1.1.")));
        assert_eq!(T::Unknown(".1.1".to_string()), T::new_from_attr(testdata::to_attr_borrowed(b".1.1")));
        assert_eq!(T::Unknown(".1".to_string()), T::new_from_attr(testdata::to_attr_borrowed(b".1")));
        assert_eq!(T::Unknown("1.".to_string()), T::new_from_attr(testdata::to_attr_borrowed(b"1.")));
    }

    #[test]
    fn test_dummy() {
        assert_eq!(T::dummy(), T::Unknown("?".to_string()));
    }
}