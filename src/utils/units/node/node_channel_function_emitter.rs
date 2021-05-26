//!Module for Node used in ChannelFunction.emitter
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

use crate::utils::units::node::{GDTFNodeError, Node, NodeOption};
use crate::utils::units::snap::Snap::No;

#[derive(Debug)]
pub struct NodeChannelFunctionEmitter(pub Option<Node>);

impl From<Attribute<'_>> for NodeChannelFunctionEmitter {
    fn from(attr: Attribute<'_>) -> Self {
        let value = match std::str::from_utf8(attr.value.borrow()) {
            Ok(s) => s,
            Err(_) => ""
        };
        if value == "" {
            NodeChannelFunctionEmitter(None)
        } else {
            NodeChannelFunctionEmitter(Some(attr.into()))
        }
    }
}

impl TryFrom<&str> for NodeChannelFunctionEmitter {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "" {
            Ok(NodeChannelFunctionEmitter(None))
        } else {
            Ok(NodeChannelFunctionEmitter(Some(value.try_into()?)))
        }
    }
}

///Partial eq returns false if one is None, otherwise it compares the node value
impl PartialEq for NodeChannelFunctionEmitter {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            None => false,
            Some(v1) => if let Some(v2) = &other.0 { v1 == v2 } else { false }
        }
    }
}

impl Default for NodeChannelFunctionEmitter {
    fn default() -> Self {
        NodeChannelFunctionEmitter(None)
    }
}

impl Display for NodeChannelFunctionEmitter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            None => write!(f, ""),
            Some(val) => write!(f, "{}", val)
        }
    }
}

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
    use crate::utils::units::node::{Node, NodeOption};
    use crate::utils::units::node::node_channel_function_emitter::NodeChannelFunctionEmitter;

    #[test]
    fn test_from_attr_borrowed() {
        NodeChannelFunctionEmitter(None).assert_eq(&testdata::to_attr_borrowed(b"").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap()]))), testdata::to_attr_borrowed(b"One").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))), testdata::to_attr_borrowed(b"One.Two").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into().unwrap()]))), testdata::to_attr_borrowed(b"Some{Invalid.Two").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())]))), testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into());
    }

    #[test]
    fn test_from_attr_owned() {
        NodeChannelFunctionEmitter(None).assert_eq(&testdata::to_attr_owned(b"").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap()]))), testdata::to_attr_owned(b"One").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))), testdata::to_attr_owned(b"One.Two").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into().unwrap()]))), testdata::to_attr_owned(b"Some{Invalid.Two").into());
        assert_eq!(NodeChannelFunctionEmitter(Some(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())]))), testdata::to_attr_owned(b"Some{Invalid.T{wo").into());
    }

    #[test]
    fn test_try_from_str() {
        NodeChannelFunctionEmitter(None).assert_eq(&"".try_into().unwrap());
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
        NodeChannelFunctionEmitter(None).assert_eq(&Default::default())
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", NodeChannelFunctionEmitter(Some(Node(vec!["One2".try_into().unwrap(), "Two".try_into().unwrap()])))), "One2.Two");
        assert_eq!(format!("{}", NodeChannelFunctionEmitter(Some(Node(vec!["One2".try_into().unwrap()])))), "One2");
        assert_eq!(format!("{}", NodeChannelFunctionEmitter(Some(Node(vec!["".try_into().unwrap()])))), "");
        assert_eq!(format!("{}", NodeChannelFunctionEmitter(None)), "");
    }
}