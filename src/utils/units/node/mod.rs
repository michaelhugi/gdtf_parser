//! Module for the unit Node used in GDTF
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

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
pub trait Node: Sized {
    ///Creates a vec of Names from a vec of str where names are checked for validity defined by GDTF
    fn strs_to_names_vec(names: Vec<&str>) -> Result<Vec<Name>, GdtfNodeError> {
        let mut ns = vec![];
        for name in names.iter() {
            ns.push(Name::new(name)?)
        }
        Ok(ns)
    }

    ///creates a vec of Names from single name (&str) where name is checked for validity defined by GDTF
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
}


#[derive(Debug)]
/// Error that occures if the format of GUID is wrong e.q. not XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX
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
    use crate::utils::units::name::Name;
    use crate::utils::units::node::{GdtfNodeError, Node};

    #[derive(Debug)]
    struct T {}

    impl Node for T {}


    #[test]
    fn test_strs_to_names_vec() -> Result<(), GdtfNodeError> {
        assert_eq!(vec![Name::new("Test")?, Name::new("Test2")?, Name::new("Test3")?], T::strs_to_names_vec(vec!["Test", "Test2", "Test3"])?);
        assert_eq!(vec![Name::new("Test")?, Name::new("Test3")?], T::strs_to_names_vec(vec!["Test", "Test3"])?);
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
}