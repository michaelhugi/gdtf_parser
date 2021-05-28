//!Module for Node used in ChannelFunction.Filter
use std::convert::TryFrom;

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::partial_eq_option::partial_eq_option;
use crate::utils::units::node::{GDTFNodeError, Node};
use crate::utils::units::node::node_option::NodeOption;

#[derive(Debug)]
///Node used in ChannelFunction.filter. Optional link to filter in the physical description; Starting point: Filter Collect
pub struct NodeChannelFunctionFilter(Option<Node>);

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl From<Attribute<'_>> for NodeChannelFunctionFilter {
    fn from(attr: Attribute<'_>) -> Self {
        NodeChannelFunctionFilter(Self::read_option_from_attr(attr))
    }
}

impl NodeChannelFunctionFilter {
    ///Creates a new instance with names, where names are checked if they are valid for GDTF
    pub fn new(names: Vec<&str>) -> Result<Self, GDTFNodeError> {
        Ok(Self(Some(Node::new(names)?)))
    }

    ///Creates a new instance with one name, where name is checked if it's valid for GDTF
    pub fn new_s(name: &str) -> Result<Self, GDTFNodeError> {
        Ok(Self(Some(Node::new_s(name)?)))
    }
    #[cfg(test)]
    ///Creates a new instance with names, where names are not checked if they are valid for GDTF
    pub(crate) fn new_unchecked(names: Vec<&str>) -> Self {
        Self(Some(Node::new_unchecked(names)))
    }
    #[cfg(test)]
    ///Creates a new instance with one name, where name is checked if it's valid for GDTF
    pub(crate) fn new_unchecked_s(name: &str) -> Self {
        Self(Some(Node::new_unchecked_s(name)))
    }

    ///Creates a new instance with None content
    pub fn none() -> Self {
        Self(None)
    }
}

#[cfg(test)]
impl PartialEqAllowEmpty for NodeChannelFunctionFilter {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        Self::is_eq_allow_option_allow_empty(&self.0, &other.0, log)
    }
}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeChannelFunctionFilter {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(NodeChannelFunctionFilter(Self::try_read_from_str(value)?))
    }
}

///Partial eq returns false if one is None, otherwise it compares the node value
impl PartialEq for NodeChannelFunctionFilter {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_option(&self.0, &other.0)
    }
}

///Default value is None
impl Default for NodeChannelFunctionFilter {
    fn default() -> Self {
        NodeChannelFunctionFilter(None)
    }
}

///Implements helper trait for Option<Node> to prevent code redundancy
impl NodeOption for NodeChannelFunctionFilter {}


#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use NodeChannelFunctionFilter as T;

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;
    use crate::utils::units::node::node_channel_function_filter::NodeChannelFunctionFilter;

    #[test]
    fn test_partial_eq_allow_empty() -> Result<(), GdtfError> {
        assert!(T::new_s("")?.is_eq_allow_empty(&T::new_s("")?, true));
        assert!(T(Some(Node(vec![]))).is_eq_allow_empty(&T(Some(Node(vec![]))), true));
        assert!(T::new_s("test")?.is_eq_allow_empty(&T::new_s("test")?, true));
        assert!(T::new(vec!["some", "test"])?.is_eq_allow_empty(&T::new(vec!["some", "test"])?, true));
        assert!(T::new(vec!["", "test"])?.is_eq_allow_empty(&T::new(vec!["", "test"])?, true));
        assert!(T::new(vec!["some", ""])?.is_eq_allow_empty(&T::new(vec!["some", ""])?, true));

        assert!(!T::new_s("")?.is_eq_allow_empty(&T::new_s("test")?, false));
        assert!(!T::new_s("some")?.is_eq_allow_empty(&T::new_s("test")?, false));
        assert!(!T::new_s("test")?.is_eq_allow_empty(&T::new_s("")?, false));
        assert!(!T(Some(Node(vec![]))).is_eq_allow_empty(&T::new_s("")?, false));
        assert!(!T::new_s("")?.is_eq_allow_empty(&T(Some(Node(vec![]))), false));
        Ok(())
    }

    #[test]
    fn test_new() -> Result<(), GdtfError> {
        T(Some(Node(vec![Name::new("test")?]))).assert_eq_allow_empty(&T::new(vec!["test"])?, true);
        T(Some(Node(vec![Name::new("test")?, Name::new("other")?]))).assert_eq_allow_empty(&T::new(vec!["test", "other"])?, true);
        T(Some(Node(vec![Name::new("")?]))).assert_eq_allow_empty(&T::new(vec![""])?, true);
        T(Some(Node(vec![]))).assert_eq_allow_empty(&T::new(vec![])?, true);
        assert!(T::new(vec!["asdf{"]).is_err());
        assert!(T::new(vec!["test", "asdf{"]).is_err());
        Ok(())
    }

    #[test]
    fn test_new_s() -> Result<(), GdtfError> {
        T(Some(Node(vec![Name::new("test")?]))).assert_eq_allow_empty(&T::new_s("test")?, true);
        T(Some(Node(vec![Name::new("")?]))).assert_eq_allow_empty(&T::new_s("")?, true);
        assert!(T::new_s("asdf{").is_err());
        Ok(())
    }

    #[test]
    fn test_new_unchecked() -> Result<(), GdtfError> {
        T(Some(Node(vec![Name::new("test")?]))).assert_eq_allow_empty(&T::new_unchecked(vec!["test"]), true);
        T(Some(Node(vec![Name::new("test")?, Name::new("other")?]))).assert_eq_allow_empty(&T::new_unchecked(vec!["test", "other"]), true);
        T(Some(Node(vec![Name::new("")?]))).assert_eq_allow_empty(&T::new_unchecked(vec![""]), true);
        T(Some(Node(vec![]))).assert_eq_allow_empty(&T::new_unchecked(vec![]), true);
        T(Some(Node(vec![Name::new_unchecked("asdf{")]))).assert_eq_allow_empty(&T::new_unchecked(vec!["asdf{"]), true);
        T(Some(Node(vec![Name::new("test")?, Name::new_unchecked("asdf{")]))).assert_eq_allow_empty(&T::new_unchecked(vec!["test", "asdf{"]), true);
        Ok(())
    }

    #[test]
    fn test_new_unchecked_s() -> Result<(), GdtfError> {
        T(Some(Node(vec![Name::new("test")?]))).assert_eq_allow_empty(&T::new_unchecked_s("test"), true);
        T(Some(Node(vec![Name::new("")?]))).assert_eq_allow_empty(&T::new_unchecked_s(""), true);
        T(Some(Node(vec![Name::new_unchecked("asdf{")]))).assert_eq_allow_empty(&T::new_unchecked_s("asdf{"), true);
        Ok(())
    }

    #[test]
    fn test_none() {
        T(None).assert_eq_allow_empty(&T::none(), true)
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        T(Some(Node(vec!["One".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        T(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        T(Some(Node(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.Two").into(), true);
        T(Some(Node(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")]))).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        T(Some(Node(vec!["One".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        T(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        T(Some(Node(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.Two").into(), true);
        T(Some(Node(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")]))).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&"".try_into()?, true);
        T(Some(Node(vec!["One".try_into()?]))).assert_eq_allow_empty(&"One".try_into()?, true);
        T(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))).assert_eq_allow_empty(&"One.Two".try_into()?, true);
        assert!(T::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_ne!(T(None), T(None));
        assert_ne!(T(None), T(Some(Node(vec![]))));
        assert_ne!(T(Some(Node(vec![]))), T(None));
        assert_eq!(T(Some(Node(vec![]))), T(Some(Node(vec![]))));
        assert_eq!(T(Some(Node(vec!["One".try_into()?]))), T(Some(Node(vec!["One".try_into()?]))));
        assert_eq!(T(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))), T(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))));
        assert_ne!(T(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))), T(Some(Node(vec!["One".try_into()?]))));
        assert_ne!(T(Some(Node(vec!["Two".try_into()?, "One".try_into()?]))), T(Some(Node(vec!["One".try_into()?, "Two".try_into()?]))));
        Ok(())
    }

    #[test]
    fn test_default() {
        T(None).assert_eq_allow_empty(&Default::default(), true)
    }
}