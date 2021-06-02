//!Module for Node used in LogicalChannel.attribute
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};

use quick_xml::events::attributes::Attribute;

use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::node::{GDTFNodeError, Node};

#[derive(Debug, PartialEq, Clone)]
///Node used in LogicalChannel.attribute. Link to the channel function that will be activated by default for this DMXChannel;
pub struct NodeLogicalChannelAttribute(Option<Vec<AttributeName>>);

impl Node for NodeLogicalChannelAttribute {}

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl From<Attribute<'_>> for NodeLogicalChannelAttribute {
    fn from(attr: Attribute<'_>) -> Self {
        Self::new_from_str_unchecked(std::str::from_utf8(attr.value.borrow()).unwrap_or_else(|_| ""))
    }
}

impl NodeLogicalChannelAttribute {
    ///New Node from str defined in GDTF-XML with checking if chars are valid for GDTF-Names
    pub fn new_from_str(value: &str) -> Result<Self, GDTFNodeError> {
        if value == "" {
            return Ok(Self(None));
        }
        let value = value.split(".");
        let mut tree: Vec<AttributeName> = vec![];
        for value in value.into_iter() {
            tree.push(value.try_into()?);
        }
        Ok(Self(Some(tree)))
    }
    ///New Node from str defined in GDTF-XML without checking if chars are valid for GDTF-Names
    pub fn new_from_str_unchecked(value: &str) -> Self {
        if value == "" {
            return Self(None);
        }
        let value = value.split(".");
        let mut tree: Vec<AttributeName> = vec![];
        for value in value.into_iter() {
            tree.push(AttributeName::new_from_str_unchecked(value));
        }
        Self(Some(tree))
    }

    ///New Node from a vec of AttributeNames
    pub fn new_from_attribute_names(names: Vec<AttributeName>) -> Result<Self, GDTFNodeError> {
        Ok(Self(Some(names)))
    }

    ///Creates a new instance with None content
    pub fn none() -> Self {
        Self(None)
    }

    ///New Node from a vec of str defined in GDTF-XML without checking if charse are valid for GDTF-Names
    pub fn new_from_strs_unchecked(names: Vec<&str>) -> Self {
        let mut vec = vec![];
        for name in names.into_iter() {
            vec.push(AttributeName::new_from_str_unchecked(name));
        }
        Self(Some(vec))
    }
}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeLogicalChannelAttribute {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::new_from_str(value)?)
    }
}

///Default value is None
impl Default for NodeLogicalChannelAttribute {
    fn default() -> Self {
        NodeLogicalChannelAttribute(None)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use NodeLogicalChannelAttribute as T;

    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::node::node_logical_channel_attribute::NodeLogicalChannelAttribute;

    #[test]
    fn test_new_from_str() -> Result<(), GdtfError> {
        assert_eq!(T(Some(vec![AttributeName::new_from_str("test")?])), T::new_from_str("test")?);
        assert_eq!(T(None), T::new_from_str("")?);
        assert!(T::new_from_str("asdf{").is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_strs_unchecked() -> Result<(), GdtfError> {
        assert_eq!(T(Some(vec![AttributeName::new_from_str("test")?])), T::new_from_strs_unchecked(vec!["test"]));
        assert_eq!(T(Some(vec![AttributeName::new_from_str("test")?, AttributeName::new_from_str("other")?])), T::new_from_strs_unchecked(vec!["test", "other"]));
        assert_eq!(T(Some(vec![AttributeName::new_from_str("")?])), T::new_from_strs_unchecked(vec![""]));
        assert_eq!(T(Some(vec![])), T::new_from_strs_unchecked(vec![]));
        assert_eq!(T(Some(vec![AttributeName::new_from_str_unchecked("asdf{")])), T::new_from_strs_unchecked(vec!["asdf{"]));
        assert_eq!(T(Some(vec![AttributeName::new_from_str("test")?, AttributeName::new_from_str_unchecked("asdf{")])), T::new_from_strs_unchecked(vec!["test", "asdf{"]));
        Ok(())
    }

    #[test]
    fn test_new_from_str_unchecked() -> Result<(), GdtfError> {
        assert_eq!(T(Some(vec![AttributeName::new_from_str("test")?])), T::new_from_str_unchecked("test"));
        assert_eq!(T(None), T::new_from_str_unchecked(""));
        assert_eq!(T(Some(vec![AttributeName::new_from_str_unchecked("asdf{")])), T::new_from_str_unchecked("asdf{"));
        Ok(())
    }

    #[test]
    fn test_none() {
        assert_eq!(T(None), T::none())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        assert_eq!(T(None), testdata::to_attr_borrowed(b"").into());
        assert_eq!(T(Some(vec!["One".try_into()?])), testdata::to_attr_borrowed(b"One").into());
        assert_eq!(T(Some(vec!["One".try_into()?, "Two".try_into()?])), testdata::to_attr_borrowed(b"One.Two").into());
        assert_eq!(T(Some(vec![AttributeName::new_from_str_unchecked("Some{Invalid"), "Two".try_into()?])), testdata::to_attr_borrowed(b"Some{Invalid.Two").into());
        assert_eq!(T(Some(vec![AttributeName::new_from_str_unchecked("Some{Invalid"), AttributeName::new_from_str_unchecked("T{wo")])), testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into());
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        assert_eq!(T(None), testdata::to_attr_owned(b"").into());
        assert_eq!(T(Some(vec!["One".try_into()?])), testdata::to_attr_owned(b"One").into());
        assert_eq!(T(Some(vec!["One".try_into()?, "Two".try_into()?])), testdata::to_attr_owned(b"One.Two").into());
        assert_eq!(T(Some(vec![AttributeName::new_from_str_unchecked("Some{Invalid"), "Two".try_into()?])), testdata::to_attr_owned(b"Some{Invalid.Two").into());
        assert_eq!(T(Some(vec![AttributeName::new_from_str_unchecked("Some{Invalid"), AttributeName::new_from_str_unchecked("T{wo")])), testdata::to_attr_owned(b"Some{Invalid.T{wo").into());
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        assert_eq!(T(None), "".try_into()?);
        assert_eq!(T(Some(vec!["One".try_into()?])), "One".try_into()?);
        assert_eq!(T(Some(vec!["One".try_into()?, "Two".try_into()?])), "One.Two".try_into()?);
        assert!(T::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_default() {
        assert_eq!(T(None), Default::default())
    }
}