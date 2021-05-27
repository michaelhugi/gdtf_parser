//!Module for Node used in ChannelFunction.attribute
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

use crate::utils::test::assert_eq_allow_empty::AssertEqAllowEmpty;
use crate::utils::units::node::{GDTFNodeError, Node};

#[derive(Debug)]
///Node used in ChannelFunction.attribute. Link to attribute; Starting point is the attributes node. Default value: “NoFeature”.
pub enum NodeChannelFunctionAttribute {
    ///Used when a reference to a node is present
    Node(Node),
    ///Used for special value NoFeature
    NoFeature,
}

impl AssertEqAllowEmpty for NodeChannelFunctionAttribute {
    fn is_eq_allow_empty_no_log(&self, other: &Self) -> bool {
        use NodeChannelFunctionAttribute::*;
        match self {
            Node(val1) => if let Node(val2) = other { val1 == val2 } else { false }
            NoFeature => if let NoFeature = other { true } else { false }
        }
    }
}

#[cfg(test)]
///used because partial_eq will return false for NoFeature
impl NodeChannelFunctionAttribute {
    ///Needed to compare none values even when partial_eq returns false
    fn assert_eq(&self, other: &Self) {
        use NodeChannelFunctionAttribute::*;
        match self {
            NoFeature => if let NoFeature = other {} else { panic!("left: NoFeature\n right: {}", other) }
            Node(v1) => if let Node(v2) = other { assert_eq!(v1, v2); } else { panic!("left: {}\n right: NoFeature", v1) }
        }
    }
}

///Parses an xml attribute to a node. In case of error it returns default
impl From<Attribute<'_>> for NodeChannelFunctionAttribute {
    fn from(attr: Attribute<'_>) -> Self {
        let value = match std::str::from_utf8(attr.value.borrow()) {
            Ok(s) => s,
            Err(_) => ""
        };
        if value == "" || value == "NoFeature" {
            NodeChannelFunctionAttribute::NoFeature
        } else {
            NodeChannelFunctionAttribute::Node(attr.into())
        }
    }
}

///Parses a string to a Node
impl TryFrom<&str> for NodeChannelFunctionAttribute {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "" || value == "NoFeature" {
            Ok(NodeChannelFunctionAttribute::NoFeature)
        } else {
            Ok(NodeChannelFunctionAttribute::Node(value.try_into()?))
        }
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

///Displays NoFeature or the path to the node in format Name.Name.Name
impl Display for NodeChannelFunctionAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NodeChannelFunctionAttribute::Node(node) => write!(f, "{}", node),
            NodeChannelFunctionAttribute::NoFeature => write!(f, "NoFeature")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;
    use crate::utils::units::node::node_channel_function_attribute::NodeChannelFunctionAttribute;

    #[test]
    fn test_from_attr_borrowed() {
        NodeChannelFunctionAttribute::NoFeature.assert_eq(&testdata::to_attr_borrowed(b"NoFeature").into());
        NodeChannelFunctionAttribute::NoFeature.assert_eq(&testdata::to_attr_borrowed(b"").into());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap()])), testdata::to_attr_borrowed(b"One").into());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()])), testdata::to_attr_borrowed(b"One.Two").into());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into().unwrap()])), testdata::to_attr_borrowed(b"Some{Invalid.Two").into());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())])), testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into());
    }

    #[test]
    fn test_from_attr_owned() {
        NodeChannelFunctionAttribute::NoFeature.assert_eq(&testdata::to_attr_owned(b"NoFeature").into());
        NodeChannelFunctionAttribute::NoFeature.assert_eq(&testdata::to_attr_owned(b"").into());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap()])), testdata::to_attr_owned(b"One").into());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()])), testdata::to_attr_owned(b"One.Two").into());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into().unwrap()])), testdata::to_attr_owned(b"Some{Invalid.Two").into());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())])), testdata::to_attr_owned(b"Some{Invalid.T{wo").into());
    }

    #[test]
    fn test_try_from_str() {
        NodeChannelFunctionAttribute::NoFeature.assert_eq(&"NoFeature".try_into().unwrap());
        NodeChannelFunctionAttribute::NoFeature.assert_eq(&"".try_into().unwrap());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap()])), "One".try_into().unwrap());
        assert_eq!(NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()])), "One.Two".try_into().unwrap());
        assert!(NodeChannelFunctionAttribute::try_from("Some{Invalid").is_err());
    }

    #[test]
    fn test_partial_eq() {

        assert_ne!(
            NodeChannelFunctionAttribute::NoFeature,
            NodeChannelFunctionAttribute::NoFeature
        );
        assert_eq!(
            NodeChannelFunctionAttribute::Node(Node(vec![])),
            NodeChannelFunctionAttribute::Node(Node(vec![]))
        );
        assert_eq!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap()])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap()]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["".try_into().unwrap()])),
            NodeChannelFunctionAttribute::Node(Node(vec!["".try_into().unwrap()]))
        );
        assert_eq!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap()]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap()])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap()])),
            NodeChannelFunctionAttribute::NoFeature
        );
        assert_ne!(
            NodeChannelFunctionAttribute::NoFeature,
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap()]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::NoFeature,
            NodeChannelFunctionAttribute::Node(Node(vec!["".try_into().unwrap()]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["".try_into().unwrap()])),
            NodeChannelFunctionAttribute::NoFeature
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two2".try_into().unwrap()])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One2".try_into().unwrap(), "Two".try_into().unwrap()])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]))
        );
    }

    #[test]
    fn test_default() {
        NodeChannelFunctionAttribute::NoFeature.assert_eq(&Default::default())
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", NodeChannelFunctionAttribute::Node(Node(vec!["One2".try_into().unwrap(), "Two".try_into().unwrap()]))), "One2.Two");
        assert_eq!(format!("{}", NodeChannelFunctionAttribute::Node(Node(vec!["One2".try_into().unwrap()]))), "One2");
        assert_eq!(format!("{}", NodeChannelFunctionAttribute::Node(Node(vec!["".try_into().unwrap()]))), "");
        assert_eq!(format!("{}", NodeChannelFunctionAttribute::NoFeature), "NoFeature");
    }
}