//!Module for Node used in ChannelFunction.attribute
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::units::node::{GDTFNodeError, Node};

#[derive(Debug)]
///Node used in ChannelFunction.attribute. Link to attribute; Starting point is the attributes node. Default value: “NoFeature”.
pub enum NodeChannelFunctionAttribute {
    ///Used when a reference to a node is present
    Node(Node),
    ///Used for special value NoFeature
    NoFeature,
}

#[cfg(test)]
impl PartialEqAllowEmpty for NodeChannelFunctionAttribute {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        use NodeChannelFunctionAttribute::*;
        match self {
            Node(val1) => if let Node(val2) = other { val1.is_eq_allow_empty(&val2, log) } else { false }
            NoFeature => if let NoFeature = other { true } else { false }
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

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;
    use crate::utils::units::node::node_channel_function_attribute::NodeChannelFunctionAttribute;

    #[test]
    fn test_assert_eq_allow_empty() -> Result<(), GdtfError> {
        use NodeChannelFunctionAttribute as T;
        assert!(T::NoFeature.is_eq_allow_empty(&T::NoFeature, true));
        assert!(T::Node(Node(vec![])).is_eq_allow_empty(&T::Node(Node(vec![])), true));
        assert!(T::Node(Node(vec!["One".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["One".try_into()?])), true));
        assert!(T::Node(Node(vec!["".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["".try_into()?])), true));
        assert!(T::Node(Node(vec!["One".try_into()?, "Two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["One".try_into()?, "Two".try_into()?])), true));
        assert!(T::Node(Node(vec![Name::Name("".to_string()), "Two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec![Name::Name("".to_string()), "Two".try_into()?])), true));
        assert!(T::Node(Node(vec!["One".try_into()?, Name::Name("".to_string())])).is_eq_allow_empty(&T::Node(Node(vec!["One".try_into()?, Name::Name("".to_string())])), true));
        assert!(T::Node(Node(vec!["One".try_into()?, Name::Empty])).is_eq_allow_empty(&T::Node(Node(vec!["One".try_into()?, Name::Empty])), true));


        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec![])), false));
        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec![Name::Name("".to_string())])), false));
        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec![Name::Empty])), false));
        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?])), false));
        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?, "two".try_into()?])), false));

        assert!(!T::Node(Node(vec![])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec![Name::Name("".to_string())])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec![Name::Empty])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec!["one".try_into()?])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["two".try_into()?, "one".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?, "".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?, "two2".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one2".try_into()?, "two".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["".try_into()?, "two".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec![Name::Empty, "two".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?, Name::Empty])), false));
        Ok(())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        NodeChannelFunctionAttribute::NoFeature.assert_eq_allow_empty(&testdata::to_attr_borrowed(b"NoFeature").into(), true);
        NodeChannelFunctionAttribute::NoFeature.assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        NodeChannelFunctionAttribute::Node(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.Two").into(), true);
        NodeChannelFunctionAttribute::Node(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        NodeChannelFunctionAttribute::NoFeature.assert_eq_allow_empty(&testdata::to_attr_owned(b"NoFeature").into(), true);
        NodeChannelFunctionAttribute::NoFeature.assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        NodeChannelFunctionAttribute::Node(Node(vec![Name::Name("Some{Invalid".to_string()), "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.Two").into(), true);
        NodeChannelFunctionAttribute::Node(Node(vec![Name::Name("Some{Invalid".to_string()), Name::Name("T{wo".to_string())])).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        NodeChannelFunctionAttribute::NoFeature.assert_eq_allow_empty(&"NoFeature".try_into()?, true);
        NodeChannelFunctionAttribute::NoFeature.assert_eq_allow_empty(&"".try_into()?, true);
        NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?])).assert_eq_allow_empty(&"One".try_into()?, true);
        NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&"One.Two".try_into()?, true);
        assert!(NodeChannelFunctionAttribute::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_ne!(
            NodeChannelFunctionAttribute::NoFeature,
            NodeChannelFunctionAttribute::NoFeature
        );
        assert_eq!(
            NodeChannelFunctionAttribute::Node(Node(vec![])),
            NodeChannelFunctionAttribute::Node(Node(vec![]))
        );
        assert_eq!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["".try_into()?])),
            NodeChannelFunctionAttribute::Node(Node(vec!["".try_into()?]))
        );
        assert_eq!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two".try_into()?])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two".try_into()?]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two".try_into()?])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two".try_into()?]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?])),
            NodeChannelFunctionAttribute::NoFeature
        );
        assert_ne!(
            NodeChannelFunctionAttribute::NoFeature,
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::NoFeature,
            NodeChannelFunctionAttribute::Node(Node(vec!["".try_into()?]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["".try_into()?])),
            NodeChannelFunctionAttribute::NoFeature
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two2".try_into()?])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two".try_into()?]))
        );
        assert_ne!(
            NodeChannelFunctionAttribute::Node(Node(vec!["One2".try_into()?, "Two".try_into()?])),
            NodeChannelFunctionAttribute::Node(Node(vec!["One".try_into()?, "Two".try_into()?]))
        );
        Ok(())
    }

    #[test]
    fn test_default() {
        NodeChannelFunctionAttribute::NoFeature.assert_eq_allow_empty(&Default::default(), true)
    }

    #[test]
    fn test_display() -> Result<(), GdtfError> {
        assert_eq!(format!("{}", NodeChannelFunctionAttribute::Node(Node(vec!["One2".try_into()?, "Two".try_into()?]))), "One2.Two");
        assert_eq!(format!("{}", NodeChannelFunctionAttribute::Node(Node(vec!["One2".try_into()?]))), "One2");
        assert_eq!(format!("{}", NodeChannelFunctionAttribute::Node(Node(vec!["".try_into()?]))), "");
        assert_eq!(format!("{}", NodeChannelFunctionAttribute::NoFeature), "NoFeature");
        Ok(())
    }
}