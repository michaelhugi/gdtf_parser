//!Module for Node used in ChannelFunction.attribute
use std::borrow::Borrow;
use std::convert::TryFrom;

use quick_xml::events::attributes::Attribute;

use crate::utils::units::name::Name;
use crate::utils::units::node::{GdtfNodeError, Node};

#[derive(Debug, PartialEq, Clone)]
///Node used in ChannelFunction.attribute. Link to attribute; Starting point is the attributes node. Default value: “NoFeature”.
pub enum NodeChannelFunctionAttribute {
    ///Used when a reference to a node is present
    Node(Vec<Name>),
    ///Used for special value NoFeature
    NoFeature,
}


impl NodeChannelFunctionAttribute {
    #[cfg(test)]
    ///New Node from a vec of str defined in GDTF-XML without checking if charse are valid for GDTF-Names
    pub fn new_from_strs(value: Vec<&str>) -> Result<Self, GdtfNodeError> {
        Ok(Self::Node(Self::strs_to_names_vec(value)?))
    }
    ///New Node with None content
    pub fn no_feature() -> Self {
        Self::NoFeature
    }
}

impl Node for NodeChannelFunctionAttribute {
    ///New Node from str defined in GDTF-XML with checking if chars are valid for GDTF-Names
    fn new_from_str(value: &str) -> Result<Self, GdtfNodeError> {
        if value.is_empty() || value == "NoFeature" {
            return Ok(Self::NoFeature);
        }
        Ok(Self::Node(Self::str_to_names_vec(value)?))
    }
}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeChannelFunctionAttribute {
    type Error = GdtfNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_from_str(value)
    }
}

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl TryFrom<Attribute<'_>> for NodeChannelFunctionAttribute {
    type Error = GdtfNodeError;

    fn try_from(attr: Attribute<'_>) -> Result<Self, Self::Error> {
        Self::new_from_str(std::str::from_utf8(attr.value.borrow()).unwrap_or(""))
    }
}

///Default value is NoFeature
impl Default for NodeChannelFunctionAttribute {
    fn default() -> Self {
        NodeChannelFunctionAttribute::NoFeature
    }
}
