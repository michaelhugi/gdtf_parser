//!Module for Node used in ChannelFunction.attribute
use std::borrow::Borrow;
use std::convert::TryFrom;

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::units::name::Name;
use crate::utils::units::node::{GDTFNodeError, Node};

#[derive(Debug)]
///Node used in ChannelFunction.attribute. Link to attribute; Starting point is the attributes node. Default value: “NoFeature”.
pub enum NodeChannelFunctionAttribute {
    ///Used when a reference to a node is present
    Node(Vec<Name>),
    ///Used for special value NoFeature
    NoFeature,
}


impl NodeChannelFunctionAttribute {
    ///New Node from str defined in GDTF-XML with checking if chars are valid for GDTF-Names
    pub fn new_from_str(value: &str) -> Result<Self, GDTFNodeError> {
        if value == "" || value == "NoFeature" {
            return Ok(Self::NoFeature);
        }
        Ok(Self::Node(Self::str_to_names_vec(value)?))
    }
    ///New Node from str defined in GDTF-XML without checking if chars are valid for GDTF-Names
    pub fn new_from_str_unchecked(value: &str) -> Self {
        if value == "" || value == "NoFeature" {
            return Self::NoFeature;
        }
        Self::Node(Name::str_to_names_vec_unchecked(value))
    }
    #[cfg(test)]
    ///New Node from a vec of str defined in GDTF-XML without checking if charse are valid for GDTF-Names
    pub fn new_from_strs_unchecked(value: Vec<&str>) -> Self {
        Self::Node(Name::strs_to_names_vec_unchecked(value))
    }
    ///New Node with None content
    pub fn no_feature() -> Self {
        Self::NoFeature
    }
}

impl Node for NodeChannelFunctionAttribute {}

#[cfg(test)]
impl PartialEqAllowEmpty for NodeChannelFunctionAttribute {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        use NodeChannelFunctionAttribute::*;
        match self {
            Node(val1) => if let Node(val2) = other { Self::is_vec_eq_ordered(&val1, &val2, log) } else { false }
            NoFeature => if let NoFeature = other { true } else { false }
        }
    }
}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeChannelFunctionAttribute {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_from_str(value)
    }
}

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl From<Attribute<'_>> for NodeChannelFunctionAttribute {
    fn from(attr: Attribute<'_>) -> Self {
        Self::new_from_str_unchecked(std::str::from_utf8(attr.value.borrow()).unwrap_or_else(|_| ""))
    }
}


///Partial eq returns false if one is NoFeature, otherwise it compares the node value
impl PartialEq for NodeChannelFunctionAttribute {
    fn eq(&self, other: &Self) -> bool {
        use NodeChannelFunctionAttribute::*;
        match self {
            Node(v1) => if let Node(v2) = other { v1 == v2 } else { false }
            NoFeature => false
        }
    }
}

///Default value is NoFeature
impl Default for NodeChannelFunctionAttribute {
    fn default() -> Self {
        NodeChannelFunctionAttribute::NoFeature
    }
}


#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use NodeChannelFunctionAttribute as T;

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::node_channel_function_attribute::NodeChannelFunctionAttribute;

    #[test]
    fn test_new_from_str() -> Result<(), GdtfError> {
        T::Node(vec![Name::new("test")?]).assert_eq_allow_empty(&T::new_from_str("test")?, true);
        T::Node(vec![Name::new("test")?, Name::new("test2")?]).assert_eq_allow_empty(&T::new_from_str("test.test2")?, true);
        T::NoFeature.assert_eq_allow_empty(&T::new_from_str("")?, true);
        assert!(T::new_from_str("asdf{").is_err());
        assert!(T::new_from_str("Test.asdf{").is_err());
        assert!(T::new_from_str("asdf{.Test").is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_str_unchecked() -> Result<(), GdtfError> {
        T::Node(vec![Name::new("test")?]).assert_eq_allow_empty(&T::new_from_str_unchecked("test"), true);
        T::Node(vec![Name::new("test")?, Name::new("test2")?]).assert_eq_allow_empty(&T::new_from_str_unchecked("test.test2"), true);
        T::NoFeature.assert_eq_allow_empty(&T::new_from_str_unchecked(""), true);
        T::Node(vec![Name::new_unchecked("asdf{")]).assert_eq_allow_empty(&T::new_from_str_unchecked("asdf{"), true);
        T::Node(vec![Name::new("Test")?, Name::new_unchecked("asdf{")]).assert_eq_allow_empty(&T::new_from_str_unchecked("Test.asdf{"), true);
        T::Node(vec![Name::new_unchecked("asdf{"), Name::new("Test")?]).assert_eq_allow_empty(&T::new_from_str_unchecked("asdf{.Test"), true);
        Ok(())
    }

    #[test]
    fn test_new_from_strs_unchecked() -> Result<(), GdtfError> {
        T::Node(vec![Name::new("test")?]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["test"]), true);
        T::Node(vec![Name::new("test")?, Name::new("test2")?]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["test", "test2"]), true);
        T::Node(vec![Name::new("")?]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec![""]), true);
        T::Node(vec![Name::new_unchecked("asdf{")]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["asdf{"]), true);
        T::Node(vec![Name::new("Test")?, Name::new_unchecked("asdf{")]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["Test", "asdf{"]), true);
        T::Node(vec![Name::new_unchecked("asdf{"), Name::new("Test")?]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["asdf{", "Test"]), true);
        Ok(())
    }

    #[test]
    fn test_partial_eq_allow_empty() -> Result<(), GdtfError> {
        T::new_from_str("")?.assert_eq_allow_empty(&T::new_from_str("")?, true);
        T::Node(vec![]).assert_eq_allow_empty(&T::Node(vec![]), true);
        T::new_from_str("test")?.assert_eq_allow_empty(&T::new_from_str("test")?, true);
        T::new_from_strs_unchecked(vec!["some", "test"]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["some", "test"]), true);
        T::new_from_strs_unchecked(vec!["", "test"]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["", "test"]), true);
        T::new_from_strs_unchecked(vec!["some", ""]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["some", ""]), true);

        T::new_from_str("")?.assert_ne_allow_empty(&T::new_from_str("test")?,true);
        T::new_from_str("some")?.assert_ne_allow_empty(&T::new_from_str("test")?,true);
        T::new_from_str("test")?.assert_ne_allow_empty(&T::new_from_str("")?,true);
        T::Node(vec![]).assert_ne_allow_empty(&T::new_from_str("")?,true);
        T::new_from_str("")?.assert_ne_allow_empty(&T::Node(vec![]),true);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        T::NoFeature.assert_eq_allow_empty(&"".try_into()?, true);
        T::Node(vec!["One".try_into()?]).assert_eq_allow_empty(&"One".try_into()?, true);
        T::Node(vec!["One".try_into()?, "Two".try_into()?]).assert_eq_allow_empty(&"One.Two".try_into()?, true);
        assert!(T::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        T::NoFeature.assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        T::Node(vec!["One".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        T::Node(vec!["One".try_into()?, "Two".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        T::Node(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.Two").into(), true);
        T::Node(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")]).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        T::NoFeature.assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        T::Node(vec!["One".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        T::Node(vec!["One".try_into()?, "Two".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        T::Node(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.Two").into(), true);
        T::Node(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_ne!(T::NoFeature, T::NoFeature);
        assert_ne!(T::NoFeature, T::Node(vec![]));
        assert_ne!(T::Node(vec![]), T::NoFeature);
        assert_eq!(T::Node(vec![]), T::Node(vec![]));
        assert_eq!(T::Node(vec!["One".try_into()?]), T::Node(vec!["One".try_into()?]));
        assert_eq!(T::Node(vec!["One".try_into()?, "Two".try_into()?]), T::Node(vec!["One".try_into()?, "Two".try_into()?]));
        assert_ne!(T::Node(vec!["One".try_into()?, "Two".try_into()?]), T::Node(vec!["One".try_into()?]));
        assert_ne!(T::Node(vec!["Two".try_into()?, "One".try_into()?]), T::Node(vec!["One".try_into()?, "Two".try_into()?]));
        Ok(())
    }

    #[test]
    fn test_no_feature() -> Result<(), GdtfError> {
        T::NoFeature.assert_eq_allow_empty(&T::no_feature(), true);
        Ok(())
    }

    #[test]
    fn test_default() -> Result<(), GdtfError> {
        T::NoFeature.assert_eq_allow_empty(&Default::default(), true);
        Ok(())
    }
}
