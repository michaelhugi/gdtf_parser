//!Module for Node used in DmxChannel.initial_function
use std::borrow::Borrow;
use std::convert::TryFrom;

use quick_xml::events::attributes::Attribute;

use crate::utils::units::name::Name;
use crate::utils::units::node::{GdtfNodeError, Node};

#[derive(Debug, PartialEq, Clone)]
///Node used in DmxChannel.initial_function.Link to the channel function that will be activated by default for this DMXChannel;
pub struct NodeChannelFunctionFilter(Option<Vec<Name>>);

impl NodeChannelFunctionFilter {
    ///New Node from str defined in GDTF-XML with checking if chars are valid for GDTF-Names
    pub fn new_from_str(value: &str) -> Result<Self, GdtfNodeError> {
        if value.is_empty() {
            return Ok(Self(None));
        }
        Ok(Self(Some(Self::str_to_names_vec(value)?)))
    }
    #[cfg(test)]
    ///New Node from a vec of str defined in GDTF-XML without checking if charse are valid for GDTF-Names
    pub fn new_from_strs(value: Vec<&str>) -> Result<Self, GdtfNodeError> {
        Ok(Self(Some(Self::strs_to_names_vec(value)?)))
    }
    ///New Node with None content
    pub fn none() -> Self {
        Self(None)
    }
}

impl Node for NodeChannelFunctionFilter {}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeChannelFunctionFilter {
    type Error = GdtfNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_from_str(value)
    }
}

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl TryFrom<Attribute<'_>> for NodeChannelFunctionFilter {
    type Error = GdtfNodeError;

    fn try_from(attr: Attribute<'_>) -> Result<Self, Self::Error> {
        Self::new_from_str(std::str::from_utf8(attr.value.borrow()).unwrap_or(""))
    }
}

///Default value is None
impl Default for NodeChannelFunctionFilter {
    fn default() -> Self {
        Self(None)
    }
}

