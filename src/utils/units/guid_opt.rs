use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Deserializer};
use serde::de::Visitor;

use crate::utils::units::guid::Guid;

#[derive(Debug, PartialEq, Clone)]
pub struct GuidOpt(pub Option<Guid>);


impl<'de> Deserialize<'de> for GuidOpt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(GuidOptVisitor)
    }
}

struct GuidOptVisitor;

impl<'de> Visitor<'de> for GuidOptVisitor {
    type Value = GuidOpt;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Unique ID corresponding to RFC 4122: X–1 digit in hexadecimal notation. Example: \"308EA87D-7164-42DE-8106-A6D273F57A51\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        if v.is_empty() {
            return Ok(GuidOpt(None));
        }
        match Guid::new_from_str(v) {
            Ok(v) => Ok(GuidOpt(Some(v))),
            Err(e) => Err(serde::de::Error::custom(format!("{}", e)))
        }
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        self.visit_str(&v)
    }
}