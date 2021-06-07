//! Module for the unit Node used in GDTF
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use quick_xml::events::attributes::Attribute;

use crate::utils::deparse;
use crate::utils::units::name::{GdtfNameError, Name};

pub mod node_dmx_channel_initial_function;
pub mod node_logical_channel_attribute;
pub mod node_channel_function_attribute;
pub mod node_channel_function_wheel;
pub mod node_channel_function_emitter;
pub mod node_channel_function_filter;
pub mod node_channel_function_mode_master;
pub mod node_attribute_feature;


///Node representation used in GDTF. A Node is a link to another xml node
///Link to an element: “Name” is the value of the attribute “Name” of a defined XML node. The starting point defines each attribute separately.
pub trait Node: Sized {
    ///Creates a vec of Names from a vec of str whereOnly chars `[32..=122] = (SPACE..='z')` are allowed. If one of other chars is passed to the function, it will return an Error
    /// ```rust
    /// use gdtf_parser::utils::units::node::{Node, GdtfNodeError};
    /// use gdtf_parser::utils::units::name::Name;
    ///
    /// struct T{};
    /// impl Node for T{
    ///    fn new_from_str(value: &str) -> Result<Self, GdtfNodeError> {
    ///        Ok(Self{})
    ///    }
    ///
    /// }
    ///
    /// assert_eq!(T::strs_to_names_vec(vec!["Name1", "Name2"]).unwrap(), vec![Name("Name1".to_string()), Name("Name2".to_string())] );
    /// assert!(T::strs_to_names_vec(vec!["Name1", "Name with invalid char {"]).is_err());
    /// ```
    fn strs_to_names_vec(names: Vec<&str>) -> Result<Vec<Name>, GdtfNodeError> {
        let mut ns = vec![];
        for name in names.iter() {
            ns.push(Name::new(name)?)
        }
        Ok(ns)
    }

    ///Creates a vec of Names from a vec of str whereOnly chars `[32..=122] = (SPACE..='z')` are allowed. If one of other chars is passed to the function, it will return an Error
    /// ```rust
    /// use gdtf_parser::utils::units::node::{Node, GdtfNodeError};
    /// use gdtf_parser::utils::units::name::Name;
    ///
    /// struct T{};
    /// impl Node for T{
    ///     fn new_from_str(value: &str) -> Result<Self, GdtfNodeError> {
    ///         Ok(Self{})
    ///     }
    /// }
    ///
    /// assert_eq!(T::str_to_names_vec("Name1.Name2").unwrap(), vec![Name("Name1".to_string()), Name("Name2".to_string())] );
    /// assert!(T::str_to_names_vec("Name1.Name with invalid char {").is_err());
    /// ```
    fn str_to_names_vec(value: &str) -> Result<Vec<Name>, GdtfNodeError> {
        if value.is_empty() {
            return Ok(vec![]);
        }
        let value = value.split('.');
        let mut tree: Vec<Name> = vec![];
        for value in value.into_iter() {
            tree.push(Name::new(value)?);
        }
        Ok(tree)
    }
    ///Parses a string from gdtf-xml-description to a Node
    /// ```rust
    /// use gdtf_parser::utils::units::node::{Node, GdtfNodeError};
    /// use gdtf_parser::utils::units::name::Name;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct T {
    ///     names: Vec<Name>,
    /// }
    /// impl Node for T {
    ///     fn new_from_str(value: &str) -> Result<Self, GdtfNodeError> {
    ///         Ok(Self { names: Self::str_to_names_vec(value)? })
    ///     }
    /// }
    /// assert_eq!(T::new_from_str("Name").unwrap(), T{ names: vec![Name::new("Name").unwrap()]});
    /// assert_eq!(T::new_from_str("Name1.Name2").unwrap(), T{ names: vec![Name::new("Name1").unwrap(), Name::new("Name2").unwrap()]});
    /// assert!(T::new_from_str("Name1.Name with invalid Char {").is_err());
    /// ```
    fn new_from_str(value: &str) -> Result<Self, GdtfNodeError>;

    ///Parses a quick-xml-attribute from gdtf-xml-description to a Node
    /// ```rust
    /// use gdtf_parser::utils::units::node::{Node, GdtfNodeError};
    /// use gdtf_parser::utils::units::name::Name;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct T {
    ///     names: Vec<Name>,
    /// }
    /// impl Node for T {
    ///     fn new_from_str(value: &str) -> Result<Self, GdtfNodeError> {
    ///         Ok(Self { names: Self::str_to_names_vec(value)? })
    ///     }
    /// }
    /// assert_eq!(T::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Name")}).unwrap(), T{ names: vec![Name::new("Name").unwrap()]});
    /// assert_eq!(T::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Name1.Name2")}).unwrap(), T{ names: vec![Name::new("Name1").unwrap(), Name::new("Name2").unwrap()]});
    /// assert!(T::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Name1.Name with invalid Char {")}).is_err());
    /// ```
    fn new_from_attr(attr: Attribute<'_>) -> Result<Self, GdtfNodeError> {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}


#[derive(Debug)]
/// Error that occures if the format of Node is wrong
pub struct GdtfNodeError {}

impl Display for GdtfNodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrong argument for Node in GDTF. Format must be Name.Name.Name...!")
    }
}

impl From<GdtfNameError> for GdtfNodeError {
    fn from(_: GdtfNameError) -> Self {
        GdtfNodeError {}
    }
}

impl Error for GdtfNodeError {}


#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::{GdtfNodeError, Node};

    #[derive(Debug, PartialEq)]
    struct T {
        names: Vec<Name>,
    }

    impl Node for T {
        fn new_from_str(value: &str) -> Result<Self, GdtfNodeError> {
            Ok(Self { names: Self::str_to_names_vec(value)? })
        }
    }

    #[test]
    fn test_strs_to_names_vec() -> Result<(), GdtfNodeError> {
        assert_eq!(vec![Name::new("Test")?, Name::new("Test2")?, Name::new("Test3")?], T::strs_to_names_vec(vec!["Test", "Test2", "Test3"])?);
        assert_eq!(vec![Name::new("Test")?, Name::new("Test 3")?], T::strs_to_names_vec(vec!["Test", "Test 3"])?);
        assert_eq!(vec![Name::new("Test")?], T::strs_to_names_vec(vec!["Test"])?);
        assert!(T::strs_to_names_vec(vec!["Test", "Te{"]).is_err());
        assert!(T::strs_to_names_vec(vec!["Te{"]).is_err());
        Ok(())
    }

    #[test]
    fn test_str_to_names_vec() -> Result<(), GdtfNodeError> {
        assert_eq!(vec![Name::new("Test")?], T::str_to_names_vec("Test")?);
        assert_eq!(vec![Name::new("Test")?, Name::new("Test3")?], T::str_to_names_vec("Test.Test3")?);
        assert_eq!(vec![Name::new("Test")?, Name::new("Test2")?, Name::new("Test3")?], T::str_to_names_vec("Test.Test2.Test3")?);
        assert!(T::str_to_names_vec("Te{").is_err());
        assert!(T::str_to_names_vec("Te{.Test").is_err());
        assert!(T::str_to_names_vec("Test.Te{.Test").is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_str() -> Result<(), GdtfNodeError> {
        assert_eq!(T::new_from_str("Name")?, T { names: vec![Name::new("Name")?] });
        assert_eq!(T::new_from_str("Name.Name2")?, T { names: vec![Name::new("Name")?, Name::new("Name2")?] });
        assert!(T::new_from_str("Invalid char {").is_err());
        assert!(T::new_from_str("Invalid char ȸ").is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_attr_owned() -> Result<(), GdtfNodeError> {
        assert_eq!(T::new_from_attr(testdata::to_attr_owned(b"Name"))?, T { names: vec![Name::new("Name")?] });
        assert_eq!(T::new_from_attr(testdata::to_attr_owned(b"Name.Name2"))?, T { names: vec![Name::new("Name")?, Name::new("Name2")?] });
        assert!(T::new_from_attr(testdata::to_attr_owned(b"Invalid char {")).is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_attr_borrowed() -> Result<(), GdtfNodeError> {
        assert_eq!(T::new_from_attr(testdata::to_attr_borrowed(b"Name"))?, T { names: vec![Name::new("Name")?] });
        assert_eq!(T::new_from_attr(testdata::to_attr_borrowed(b"Name.Name2"))?, T { names: vec![Name::new("Name")?, Name::new("Name2")?] });
        assert!(T::new_from_attr(testdata::to_attr_borrowed(b"Invalid char {")).is_err());
        Ok(())
    }
}