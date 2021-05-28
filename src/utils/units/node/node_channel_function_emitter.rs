//!Module for Node used in ChannelFunction.emitter
use std::convert::TryFrom;

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::units::node::{GDTFNodeError, Node};
use crate::utils::units::node::node_option::NodeOption;
use crate::utils::partial_eq_option::partial_eq_option;

#[derive(Debug)]
///Node used in ChannelFunction.emitter. Optional link to emitter in the physical description; Starting point: Emitter Collect
pub struct NodeChannelFunctionEmitter(pub Option<Node>);

#[cfg(test)]
impl PartialEqAllowEmpty for NodeChannelFunctionEmitter {
    fn is_eq_allow_empty_impl(&self, other: &Self, _: bool) -> bool {
        Self::is_eq_allow_option(&self.0, &other.0)
    }
}

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl From<Attribute<'_>> for NodeChannelFunctionEmitter {
    fn from(attr: Attribute<'_>) -> Self {
        NodeChannelFunctionEmitter(Self::read_option_from_attr(attr))
    }
}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeChannelFunctionEmitter {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(NodeChannelFunctionEmitter(Self::try_read_from_str(value)?))
    }
}

///Partial eq returns false if one is None, otherwise it compares the node value
impl PartialEq for NodeChannelFunctionEmitter {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_option(&self.0, &other.0)
    }
}

///Default value is None
impl Default for NodeChannelFunctionEmitter {
    fn default() -> Self {
        NodeChannelFunctionEmitter(None)
    }
}

///Implements helper trait for Option<Node> to prevent code redundancy
impl NodeOption for NodeChannelFunctionEmitter {}

#[cfg(test)]
mod tests {
    use std::convert::{TryInto, TryFrom};

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;
    use crate::utils::units::node::node_channel_function_emitter::NodeChannelFunctionEmitter;

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        NodeChannelFunctionEmitter(None).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.Two").into(), true);
        NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        NodeChannelFunctionEmitter(None).assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.Two").into(), true);
        NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        NodeChannelFunctionEmitter(None).assert_eq_allow_empty(&"".try_into()?, true);
        NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?]))).assert_eq_allow_empty(&"One".try_into()?, true);
        NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))).assert_eq_allow_empty(&"One.Two".try_into()?, true);
        assert!(NodeChannelFunctionEmitter::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_ne!(NodeChannelFunctionEmitter(None), NodeChannelFunctionEmitter(None));
        assert_ne!(NodeChannelFunctionEmitter(None), NodeChannelFunctionEmitter(Some(Node(vec![]))));
        assert_ne!(NodeChannelFunctionEmitter(Some(Node(vec![]))), NodeChannelFunctionEmitter(None));
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![]))), NodeChannelFunctionEmitter(Some(Node(vec![]))));
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?]))), NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?]))));
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))), NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))));
        assert_ne!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))), NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?]))));
        assert_ne!(NodeChannelFunctionEmitter(Some(Node(vec!["Two".try_into()?, "One".try_into()?]))), NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))));
        Ok(())
    }

    #[test]
    fn test_default() {
        NodeChannelFunctionEmitter(None).assert_eq_allow_empty(&Default::default(), true)
    }
}