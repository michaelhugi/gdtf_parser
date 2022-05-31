//!Module for the unit Resource used in GDTF

use std::fmt::Formatter;
use quick_xml::events::attributes::Attribute;
use serde::{Deserialize, Deserializer};
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};

use crate::utils::read;

///File name of the resource file without extension and without subfolder.
#[derive(Debug, PartialEq, Clone)]
pub struct Resource(pub String);

impl<'de> Deserialize<'de> for Resource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(ResourceVisitor)
    }
}

struct ResourceVisitor;

impl<'de> Visitor<'de> for ResourceVisitor {
    type Value = Resource;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expected string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(Resource::new_from_str(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        self.visit_str(&v)
    }
}

impl Resource {
    ///Parses a string defined in gdtf-xml-description to Resource
    /// ```rust
    /// use gdtf_parser::utils::units::resource::Resource;
    ///
    /// assert_eq!(Resource::new_from_str("head.png"), Resource("head.png".to_string()));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        Self(s.to_string())
    }

    ///Parses a quick-xml-attribute defined in gdtf-xml-description to Resource
    /// ```rust
    /// use gdtf_parser::utils::units::resource::Resource;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(Resource::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"head.png")}), Resource("head.png".to_string()))
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::resource::Resource;

    #[test]
    pub fn test_new_from_str() {
        assert_eq!(
            Resource::new_from_str("head.png"),
            Resource("head.png".to_string())
        );
    }

    #[test]
    pub fn test_new_from_attr_owned() {
        assert_eq!(
            Resource::new_from_attr(testdata::to_attr_owned(b"head.png")),
            Resource("head.png".to_string())
        );
    }

    #[test]
    pub fn test_new_from_attr_borrowed() {
        assert_eq!(
            Resource::new_from_attr(testdata::to_attr_borrowed(b"head.png")),
            Resource("head.png".to_string())
        );
    }
}
