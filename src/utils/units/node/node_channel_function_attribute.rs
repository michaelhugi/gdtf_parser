//!Module for Node used in ChannelFunction.attribute
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};

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


impl NodeChannelFunctionAttribute {
    ///Creates a new instance with names, where names are checked if they are valid for GDTF
    pub fn new(names: Vec<&str>) -> Result<Self, GDTFNodeError> {
        Ok(Self::Node(Node::new(names)?))
    }

    ///Creates a new instance with one name, where name is checked if it's valid for GDTF
    pub fn new_s(names: &str) -> Result<Self, GDTFNodeError> {
        Ok(Self::Node(Node::new_s(names)?))
    }
    #[cfg(test)]
    ///Creates a new instance with names, where names are not checked if they are valid for GDTF
    pub(crate) fn new_unchecked(names: Vec<&str>) -> Self {
        Self::Node(Node::new_unchecked(names))
    }
    #[cfg(test)]
    ///Creates a new instance with one name, where name is checked if it's valid for GDTF
    pub(crate) fn new_unchecked_s(name: &str) -> Self {
        Self::Node(Node::new_unchecked_s(name))
    }

    ///Creates a new instance with None content
    pub fn no_feature() -> Self {
        Self::NoFeature
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

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use NodeChannelFunctionAttribute as T;

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;
    use crate::utils::units::node::node_channel_function_attribute::NodeChannelFunctionAttribute;

    #[test]
    fn test_new() -> Result<(), GdtfError> {
        T::Node(Node(vec![Name::new("test")?])).assert_eq_allow_empty(&T::new(vec!["test"])?, true);
        T::Node(Node(vec![Name::new("test")?, Name::new("other")?])).assert_eq_allow_empty(&T::new(vec!["test", "other"])?, true);
        T::Node(Node(vec![Name::new("")?])).assert_eq_allow_empty(&T::new(vec![""])?, true);
        T::Node(Node(vec![])).assert_eq_allow_empty(&T::new(vec![])?, true);
        assert!(T::new(vec!["asdf{"]).is_err());
        assert!(T::new(vec!["test", "asdf{"]).is_err());
        Ok(())
    }

    #[test]
    fn test_new_s() -> Result<(), GdtfError> {
        T::Node(Node(vec![Name::new("test")?])).assert_eq_allow_empty(&T::new_s("test")?, true);
        T::Node(Node(vec![Name::new("")?])).assert_eq_allow_empty(&T::new_s("")?, true);
        T::Node(Node(vec![])).assert_eq_allow_empty(&T::new(vec![])?, true);
        assert!(T::new_s("asdf{").is_err());
        Ok(())
    }

    #[test]
    fn test_new_unchecked() -> Result<(), GdtfError> {
        T::Node(Node(vec![Name::new("test")?])).assert_eq_allow_empty(&T::new_unchecked(vec!["test"]), true);
        T::Node(Node(vec![Name::new("test")?, Name::new("other")?])).assert_eq_allow_empty(&T::new_unchecked(vec!["test", "other"]), true);
        T::Node(Node(vec![Name::new("")?])).assert_eq_allow_empty(&T::new_unchecked(vec![""]), true);
        T::Node(Node(vec![])).assert_eq_allow_empty(&T::new_unchecked(vec![]), true);
        T::Node(Node(vec![Name::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_unchecked(vec!["asdf{"]), true);
        T::Node(Node(vec![Name::new("test")?, Name::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_unchecked(vec!["test", "asdf{"]), true);
        Ok(())
    }

    #[test]
    fn test_new_unchecked_s() -> Result<(), GdtfError> {
        T::Node(Node(vec![Name::new("test")?])).assert_eq_allow_empty(&T::new_unchecked_s("test"), true);
        T::Node(Node(vec![Name::new("")?])).assert_eq_allow_empty(&T::new_unchecked_s(""), true);
        T::Node(Node(vec![Name::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_unchecked_s("asdf{"), true);
        Ok(())
    }

    #[test]
    fn test_none() {
        T::NoFeature.assert_eq_allow_empty(&T::no_feature(), true);
    }

    #[test]
    fn test_assert_eq_allow_empty() -> Result<(), GdtfError> {
        assert!(T::NoFeature.is_eq_allow_empty(&T::NoFeature, true));
        assert!(T::Node(Node(vec![])).is_eq_allow_empty(&T::Node(Node(vec![])), true));
        assert!(T::Node(Node(vec!["One".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["One".try_into()?])), true));
        assert!(T::Node(Node(vec!["".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["".try_into()?])), true));
        assert!(T::Node(Node(vec!["One".try_into()?, "Two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["One".try_into()?, "Two".try_into()?])), true));
        assert!(T::Node(Node(vec![Name::new("")?, "Two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec![Name::new("")?, "Two".try_into()?])), true));
        assert!(T::Node(Node(vec!["One".try_into()?, Name::new("")?])).is_eq_allow_empty(&T::Node(Node(vec!["One".try_into()?, Name::new("")?])), true));
        assert!(T::Node(Node(vec!["One".try_into()?, Name::new("")?])).is_eq_allow_empty(&T::Node(Node(vec!["One".try_into()?, Name::new("")?])), true));


        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec![])), false));
        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec![Name::new("")?])), false));
        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec![Name::new("")?])), false));
        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?])), false));
        assert!(!T::NoFeature.is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?, "two".try_into()?])), false));

        assert!(!T::Node(Node(vec![])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec![Name::new("")?])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec![Name::new("")?])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec!["one".try_into()?])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::NoFeature, false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["two".try_into()?, "one".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?, "".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?, "two2".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one2".try_into()?, "two".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["".try_into()?, "two".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec![Name::new("")?, "two".try_into()?])), false));
        assert!(!T::Node(Node(vec!["one".try_into()?, "two".try_into()?])).is_eq_allow_empty(&T::Node(Node(vec!["one".try_into()?, Name::new("")?])), false));
        Ok(())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        T::NoFeature.assert_eq_allow_empty(&testdata::to_attr_borrowed(b"NoFeature").into(), true);
        T::NoFeature.assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        T::Node(Node(vec!["One".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        T::Node(Node(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        T::Node(Node(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.Two").into(), true);
        T::Node(Node(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        T::NoFeature.assert_eq_allow_empty(&testdata::to_attr_owned(b"NoFeature").into(), true);
        T::NoFeature.assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        T::Node(Node(vec!["One".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        T::Node(Node(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        T::Node(Node(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.Two").into(), true);
        T::Node(Node(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")])).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        T::NoFeature.assert_eq_allow_empty(&"NoFeature".try_into()?, true);
        T::NoFeature.assert_eq_allow_empty(&"".try_into()?, true);
        T::Node(Node(vec!["One".try_into()?])).assert_eq_allow_empty(&"One".try_into()?, true);
        T::Node(Node(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&"One.Two".try_into()?, true);
        assert!(NodeChannelFunctionAttribute::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_ne!(
            T::NoFeature,
            T::NoFeature
        );
        assert_eq!(
            T::Node(Node(vec![])),
            T::Node(Node(vec![]))
        );
        assert_eq!(
            T::Node(Node(vec!["One".try_into()?])),
            T::Node(Node(vec!["One".try_into()?]))
        );
        assert_ne!(
            T::Node(Node(vec!["".try_into()?])),
            T::Node(Node(vec!["".try_into()?]))
        );
        assert_eq!(
            T::Node(Node(vec!["One".try_into()?, "Two".try_into()?])),
            T::Node(Node(vec!["One".try_into()?, "Two".try_into()?]))
        );
        assert_ne!(
            T::Node(Node(vec!["One".try_into()?, "Two".try_into()?])),
            T::Node(Node(vec!["One".try_into()?]))
        );
        assert_ne!(
            T::Node(Node(vec!["One".try_into()?])),
            T::Node(Node(vec!["One".try_into()?, "Two".try_into()?]))
        );
        assert_ne!(
            T::Node(Node(vec!["One".try_into()?])),
            T::NoFeature
        );
        assert_ne!(
            T::NoFeature,
            T::Node(Node(vec!["One".try_into()?]))
        );
        assert_ne!(
            T::NoFeature,
            T::Node(Node(vec!["".try_into()?]))
        );
        assert_ne!(
            T::Node(Node(vec!["".try_into()?])),
            T::NoFeature
        );
        assert_ne!(
            T::Node(Node(vec!["One".try_into()?, "Two2".try_into()?])),
            T::Node(Node(vec!["One".try_into()?, "Two".try_into()?]))
        );
        assert_ne!(
            T::Node(Node(vec!["One2".try_into()?, "Two".try_into()?])),
            T::Node(Node(vec!["One".try_into()?, "Two".try_into()?]))
        );
        Ok(())
    }

    #[test]
    fn test_default() {
        T::NoFeature.assert_eq_allow_empty(&Default::default(), true);
    }
}