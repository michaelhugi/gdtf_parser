use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Deserializer};
use serde::de::Visitor;

use crate::utils::units::node::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct NodeOpt(pub Option<Node>);


impl<'de> Deserialize<'de> for NodeOpt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(NodeOptVisitor)
    }
}

struct NodeOptVisitor;

impl<'de> Visitor<'de> for NodeOptVisitor {
    type Value = NodeOpt;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("valid Node String or empty string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        if v.is_empty() {
            return Ok(NodeOpt(None));
        }
        match Node::new_from_str(v) {
            Ok(v) => Ok(NodeOpt(Some(v))),
            Err(e) => Err(serde::de::Error::custom(format!("{}", e)))
        }
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        self.visit_str(&v)
    }
}