//!Module for Node used in ChannelFunction.ModeMaster
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::partial_eq_option::partial_eq_option;
use crate::utils::units::node::{GDTFNodeError, Node};
use crate::utils::units::node::node_option::NodeOption;

#[derive(Debug)]
///Node used in ChannelFunction.mode_master. Link to DMX Channel or Channel Function; Starting point DMX mode
pub struct NodeChannelFunctionModeMaster(pub Option<Node>);

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl From<Attribute<'_>> for NodeChannelFunctionModeMaster {
    fn from(attr: Attribute<'_>) -> Self {
        NodeChannelFunctionModeMaster(Self::read_option_from_attr(attr))
    }
}

#[cfg(test)]
impl PartialEqAllowEmpty for NodeChannelFunctionModeMaster {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        Self::is_eq_allow_option(&self.0, &other.0)
    }
}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeChannelFunctionModeMaster {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(NodeChannelFunctionModeMaster(Self::try_read_from_str(value)?))
    }
}

///Partial eq returns false if one is None, otherwise it compares the node value
impl PartialEq for NodeChannelFunctionModeMaster {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_option(&self.0, &other.0)
    }
}

///Default value is None
impl Default for NodeChannelFunctionModeMaster {
    fn default() -> Self {
        NodeChannelFunctionModeMaster(None)
    }
}

///Implements helper trait for Option<Node> to prevent code redundancy
impl NodeOption for NodeChannelFunctionModeMaster {}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;
    use crate::utils::units::node::node_channel_function_mode_master::NodeChannelFunctionModeMaster;

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        NodeChannelFunctionModeMaster(None).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        NodeChannelFunctionModeMaster(Some(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.Two").into(), true);
        NodeChannelFunctionModeMaster(Some(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        NodeChannelFunctionModeMaster(None).assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        NodeChannelFunctionModeMaster(Some(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.Two").into(), true);
        NodeChannelFunctionModeMaster(Some(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        NodeChannelFunctionModeMaster(None).assert_eq_allow_empty(&"".try_into()?, true);
        NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?]))).assert_eq_allow_empty(&"One".try_into()?, true);
        NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))).assert_eq_allow_empty(&"One.Two".try_into()?, true);
        assert!(NodeChannelFunctionModeMaster::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_ne!(NodeChannelFunctionModeMaster(None), NodeChannelFunctionModeMaster(None));
        assert_ne!(NodeChannelFunctionModeMaster(None), NodeChannelFunctionModeMaster(Some(Node(vec![]))));
        assert_ne!(NodeChannelFunctionModeMaster(Some(Node(vec![]))), NodeChannelFunctionModeMaster(None));
        assert_eq!(NodeChannelFunctionModeMaster(Some(Node(vec![]))), NodeChannelFunctionModeMaster(Some(Node(vec![]))));
        assert_eq!(NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?]))), NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?]))));
        assert_eq!(NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))), NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))));
        assert_ne!(NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))), NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?]))));
        assert_ne!(NodeChannelFunctionModeMaster(Some(Node(vec!["Two".try_into()?, "One".try_into()?]))), NodeChannelFunctionModeMaster(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))));
        Ok(())
    }

    #[test]
    fn test_default() {
        NodeChannelFunctionModeMaster(None).assert_eq_allow_empty(&Default::default(), true)
    }
}