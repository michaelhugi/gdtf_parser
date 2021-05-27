//!Module for Node used in ChannelFunction.emitter
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

use crate::utils::test::assert_eq_allow_empty::AssertEqAllowEmpty;
use crate::utils::units::node::{GDTFNodeError, Node};
use crate::utils::units::node::node_option::NodeOption;

#[derive(Debug)]
///Node used in ChannelFunction.emitter. Optional link to emitter in the physical description; Starting point: Emitter Collect
pub struct NodeChannelFunctionEmitter(pub Option<Node>);


impl AssertEqAllowEmpty for NodeChannelFunctionEmitter {
    fn is_eq_allow_empty_no_log(&self, other: &Self) -> bool {
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
        self.eq_exluding_none(other)
    }
}

///Default value is None
impl Default for NodeChannelFunctionEmitter {
    fn default() -> Self {
        NodeChannelFunctionEmitter(None)
    }
}

///Displays the node in format Name.Name.Name... or "" if None node is present
impl Display for NodeChannelFunctionEmitter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_option(f)
    }
}

///Implements helper trait for Option<Node> to prevent code redundancy
impl NodeOption for NodeChannelFunctionEmitter {
    fn get_node_option(&self) -> &Option<Node> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;
    use crate::utils::units::node::node_channel_function_emitter::NodeChannelFunctionEmitter;
    use crate::utils::units::node::node_option::NodeOption;

    #[test]
    fn test_from_attr_borrowed() {
        NodeChannelFunctionEmitter(None).assert_eq_including_none(&testdata::to_attr_borrowed(b"").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap()]))), testdata::to_attr_borrowed(b"One").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))), testdata::to_attr_borrowed(b"One.Two").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into().unwrap()]))), testdata::to_attr_borrowed(b"Some{Invalid.Two").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())]))), testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into());
    }

    #[test]
    fn test_from_attr_owned() {
        NodeChannelFunctionEmitter(None).assert_eq_including_none(&testdata::to_attr_owned(b"").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap()]))), testdata::to_attr_owned(b"One").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))), testdata::to_attr_owned(b"One.Two").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into().unwrap()]))), testdata::to_attr_owned(b"Some{Invalid.Two").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())]))), testdata::to_attr_owned(b"Some{Invalid.T{wo").into());
    }

    #[test]
    fn test_try_from_str() {
        NodeChannelFunctionEmitter(None).assert_eq_including_none(&"".try_into().unwrap());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap()]))), "One".try_into().unwrap());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))), "One.Two".try_into().unwrap());
        assert!(NodeChannelFunctionEmitter::try_from("Some{Invalid").is_err());
    }

    #[test]
    fn test_partial_eq() {
        assert_ne!(NodeChannelFunctionEmitter(None), NodeChannelFunctionEmitter(None));
        assert_ne!(NodeChannelFunctionEmitter(None), NodeChannelFunctionEmitter(Some(Node(vec![]))));
        assert_ne!(NodeChannelFunctionEmitter(Some(Node(vec![]))), NodeChannelFunctionEmitter(None));
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![]))), NodeChannelFunctionEmitter(Some(Node(vec![]))));
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap()]))), NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap()]))));
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))), NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))));
        assert_ne!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))), NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap()]))));
        assert_ne!(NodeChannelFunctionEmitter(Some(Node(vec!["Two".try_into().unwrap(), "One".try_into().unwrap()]))), NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))));
    }

    #[test]
    fn test_default() {
        NodeChannelFunctionEmitter(None).assert_eq_including_none(&Default::default())
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", NodeChannelFunctionEmitter(Some(Node(vec!["One2".try_into().unwrap(), "Two".try_into().unwrap()])))), "One2.Two");
        assert_eq!(format!("{}", NodeChannelFunctionEmitter(Some(Node(vec!["One2".try_into().unwrap()])))), "One2");
        assert_eq!(format!("{}", NodeChannelFunctionEmitter(Some(Node(vec!["".try_into().unwrap()])))), "");
        assert_eq!(format!("{}", NodeChannelFunctionEmitter(None)), "");
    }
}