//!Module for the unit Resource used in GDTF

use std::fmt::Formatter;

use serde::{Deserialize, Deserializer};
use serde::de::{Error, Visitor};

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
        formatter.write_str("Expected valid Resource string")
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
}

#[cfg(test)]
mod tests {
    use crate::utils::units::resource::Resource;

    #[test]
    pub fn test_new_from_str() {
        assert_eq!(
            Resource::new_from_str("head.png"),
            Resource("head.png".to_string())
        );
    }
}
