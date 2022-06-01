//! Module for the unit Node used in GDTF
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Deserializer};
use serde::de::Visitor;

use crate::utils::units::name::{GdtfNameError, Name};

#[derive(Debug, PartialEq, Clone)]
pub struct Node(pub Vec<Name>);

///Node representation used in GDTF. A Node is a link to another xml node
///Link to an element: “Name” is the value of the attribute “Name” of a defined XML node. The starting point defines each attribute separately.
impl Node {
    ///Creates a vec of Names from a vec of str whereOnly chars `[32..=122] = (SPACE..='z')` are allowed. If one of other chars is passed to the function, it will return an Error
    /// ```rust
    /// use gdtf_parser::utils::units::name::Name;
    /// use gdtf_parser::utils::units::node::Node;
    ///
    /// assert_eq!(Node::strs_to_names_vec(vec!["Name1", "Name2"]).unwrap(), vec![Name("Name1".to_string()), Name("Name2".to_string())] );
    /// assert!(Node::strs_to_names_vec(vec!["Name1", "Name with invalid char {"]).is_err());
    /// ```
    pub fn strs_to_names_vec(names: Vec<&str>) -> Result<Vec<Name>, GdtfNodeError> {
        let mut ns = vec![];
        for name in names.iter() {
            ns.push(Name::new(name)?)
        }
        Ok(ns)
    }

    ///Creates a vec of Names from a vec of str whereOnly chars `[32..=122] = (SPACE..='z')` are allowed. If one of other chars is passed to the function, it will return an Error
    /// ```rust
    /// use gdtf_parser::utils::units::name::Name;
    /// use gdtf_parser::utils::units::node::Node;
    ///
    /// assert_eq!(Node::str_to_names_vec("Name1.Name2").unwrap().unwrap(), vec![Name("Name1".to_string()), Name("Name2".to_string())] );
    /// assert!(Node::str_to_names_vec("Name1.Name with invalid char {").is_err());
    /// ```
    pub fn str_to_names_vec(value: &str) -> Result<Vec<Name>, GdtfNodeError> {
        let value = value.split('.');
        let mut tree: Vec<Name> = vec![];
        for value in value {
            tree.push(Name::new(value)?);
        }
        Ok(tree)
    }

    ///Parses a string from gdtf-xml-description to a Node
    /// ```rust
    /// use gdtf_parser::utils::units::name::Name;
    /// use gdtf_parser::utils::units::node::Node;
    ///
    /// assert_eq!(Node::new_from_str("Name").unwrap().unwrap(), Node(vec![Name::new("Name").unwrap()]));
    /// assert_eq!(Node::new_from_str("Name1.Name2").unwrap().unwrap(), Node(vec![Name::new("Name1").unwrap(), Name::new("Name2").unwrap()]));
    /// assert!(Node::new_from_str("Name1.Name with invalid Char {").is_err());
    /// ```
    pub fn new_from_str(value: &str) -> Result<Self, GdtfNodeError> {
        Ok(Self(Self::str_to_names_vec(value)?))
    }
}

#[derive(Debug)]
/// Error that occures if the format of Node is wrong
pub struct GdtfNodeError {}

impl Display for GdtfNodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Wrong argument for Node in GDTF. Format must be Name.Name.Name...!"
        )
    }
}

impl From<GdtfNameError> for GdtfNodeError {
    fn from(_: GdtfNameError) -> Self {
        GdtfNodeError {}
    }
}

impl Error for GdtfNodeError {}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(NodeVisitor)
    }
}

struct NodeVisitor;

impl<'de> Visitor<'de> for NodeVisitor {
    type Value = Node;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("valid Node String")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        match Node::new_from_str(v) {
            Ok(item) => Ok(item),
            Err(e) => Err(serde::de::Error::custom(format!("{}", e)))
        }
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        self.visit_str(&v)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::units::name::Name;
    use crate::utils::units::node::{GdtfNodeError, Node as T};

    #[test]
    fn test_strs_to_names_vec() -> Result<(), GdtfNodeError> {
        assert_eq!(
            vec![Name::new("Test")?, Name::new("Test2")?, Name::new("Test3")?],
            T::strs_to_names_vec(vec!["Test", "Test2", "Test3"])?
        );
        assert_eq!(
            vec![Name::new("Test")?, Name::new("Test 3")?],
            T::strs_to_names_vec(vec!["Test", "Test 3"])?
        );
        assert_eq!(
            vec![Name::new("Test")?],
            T::strs_to_names_vec(vec!["Test"])?
        );
        assert!(T::strs_to_names_vec(vec!["Test", "Te{"]).is_err());
        assert!(T::strs_to_names_vec(vec!["Te{"]).is_err());
        Ok(())
    }

    #[test]
    fn test_str_to_names_vec() -> Result<(), GdtfNodeError> {
        assert_eq!(
            vec![Name::new("Test")?],
            T::str_to_names_vec("Test")?
        );
        assert_eq!(
            vec![Name::new("Test")?, Name::new("Test3")?],
            T::str_to_names_vec("Test.Test3")?
        );
        assert_eq!(
            vec![Name::new("Test")?, Name::new("Test2")?, Name::new("Test3")?],
            T::str_to_names_vec("Test.Test2.Test3")?
        );
        assert!(T::str_to_names_vec("Te{").is_err());
        assert!(T::str_to_names_vec("Te{.Test").is_err());
        assert!(T::str_to_names_vec("Test.Te{.Test").is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_str() -> Result<(), GdtfNodeError> {
        assert_eq!(
            T::new_from_str("Name")?,
            T(vec![Name::new("Name")?])
        );
        assert_eq!(
            T::new_from_str("Name.Name2")?,
            T(vec![Name::new("Name")?, Name::new("Name2")?])
        );
        assert!(T::new_from_str("Invalid char {").is_err());
        assert!(T::new_from_str("Invalid char ȸ").is_err());
        Ok(())
    }
}
