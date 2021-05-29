//!Module for Node used in DmxChannel.initial_function
use std::borrow::Borrow;
use std::convert::TryFrom;

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::partial_eq_option::partial_eq_option;
use crate::utils::units::name::Name;
use crate::utils::units::node::{GDTFNodeError, Node};

#[derive(Debug)]
///Node used in DmxChannel.initial_function.Link to the channel function that will be activated by default for this DMXChannel;
pub struct NodeChannelFunctionWheel(Option<Vec<Name>>);

impl NodeChannelFunctionWheel {
    ///New Node from str defined in GDTF-XML with checking if chars are valid for GDTF-Names
    pub fn new_from_str(value: &str) -> Result<Self, GDTFNodeError> {
        if value == "" {
            return Ok(Self(None));
        }
        Ok(Self(Some(Self::str_to_names_vec(value)?)))
    }
    ///New Node from str defined in GDTF-XML without checking if chars are valid for GDTF-Names
    pub fn new_from_str_unchecked(value: &str) -> Self {
        if value == "" {
            return Self(None);
        }
        Self(Some(Name::str_to_names_vec_unchecked(value)))
    }
    #[cfg(test)]
    ///New Node from a vec of str defined in GDTF-XML without checking if charse are valid for GDTF-Names
    pub fn new_from_strs_unchecked(value: Vec<&str>) -> Self {
        Self(Some(Name::strs_to_names_vec_unchecked(value)))
    }
    ///New Node with None content
    pub fn none() -> Self {
        Self(None)
    }
}

impl Node for NodeChannelFunctionWheel {}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeChannelFunctionWheel {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_from_str(value)
    }
}

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl From<Attribute<'_>> for NodeChannelFunctionWheel {
    fn from(attr: Attribute<'_>) -> Self {
        Self::new_from_str_unchecked(std::str::from_utf8(attr.value.borrow()).unwrap_or_else(|_| ""))
    }
}


#[cfg(test)]
impl PartialEqAllowEmpty for NodeChannelFunctionWheel {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        Self::is_eq_option_of_vec_allow_empty(&self.0, &other.0, log)
    }
}


///Partial eq returns false if one is None, otherwise it compares the node value
impl PartialEq for NodeChannelFunctionWheel {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_option(&self.0, &other.0)
    }
}

///Default value is None
impl Default for NodeChannelFunctionWheel {
    fn default() -> Self {
        Self(None)
    }
}


#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use NodeChannelFunctionWheel as T;

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::node_channel_function_wheel::NodeChannelFunctionWheel;

    #[test]
    fn test_new_from_str() -> Result<(), GdtfError> {
        T(Some(vec![Name::new("test")?])).assert_eq_allow_empty(&T::new_from_str("test")?, true);
        T(Some(vec![Name::new("test")?, Name::new("test2")?])).assert_eq_allow_empty(&T::new_from_str("test.test2")?, true);
        T(None).assert_eq_allow_empty(&T::new_from_str("")?, true);
        assert!(T::new_from_str("asdf{").is_err());
        assert!(T::new_from_str("Test.asdf{").is_err());
        assert!(T::new_from_str("asdf{.Test").is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_str_unchecked() -> Result<(), GdtfError> {
        T(Some(vec![Name::new("test")?])).assert_eq_allow_empty(&T::new_from_str_unchecked("test"), true);
        T(Some(vec![Name::new("test")?, Name::new("test2")?])).assert_eq_allow_empty(&T::new_from_str_unchecked("test.test2"), true);
        T(None).assert_eq_allow_empty(&T::new_from_str_unchecked(""), true);
        T(Some(vec![Name::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_from_str_unchecked("asdf{"), true);
        T(Some(vec![Name::new("Test")?, Name::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_from_str_unchecked("Test.asdf{"), true);
        T(Some(vec![Name::new_unchecked("asdf{"), Name::new("Test")?])).assert_eq_allow_empty(&T::new_from_str_unchecked("asdf{.Test"), true);
        Ok(())
    }

    #[test]
    fn test_new_from_strs_unchecked() -> Result<(), GdtfError> {
        T(Some(vec![Name::new("test")?])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["test"]), true);
        T(Some(vec![Name::new("test")?, Name::new("test2")?])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["test", "test2"]), true);
        T(Some(vec![Name::new("")?])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec![""]), true);
        T(Some(vec![Name::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["asdf{"]), true);
        T(Some(vec![Name::new("Test")?, Name::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["Test", "asdf{"]), true);
        T(Some(vec![Name::new_unchecked("asdf{"), Name::new("Test")?])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["asdf{", "Test"]), true);
        Ok(())
    }

    #[test]
    fn test_partial_eq_allow_empty() -> Result<(), GdtfError> {
        T::new_from_str("")?.assert_eq_allow_empty(&T::new_from_str("")?, true);
        T(Some(vec![])).assert_eq_allow_empty(&T(Some(vec![])), true);
        T::new_from_str("test")?.assert_eq_allow_empty(&T::new_from_str("test")?, true);
        T::new_from_strs_unchecked(vec!["some", "test"]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["some", "test"]), true);
        T::new_from_strs_unchecked(vec!["", "test"]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["", "test"]), true);
        T::new_from_strs_unchecked(vec!["some", ""]).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["some", ""]), true);

        T::new_from_str("")?.assert_ne_allow_empty(&T::new_from_str("test")?);
        T::new_from_str("some")?.assert_ne_allow_empty(&T::new_from_str("test")?);
        T::new_from_str("test")?.assert_ne_allow_empty(&T::new_from_str("")?);
        T(Some(vec![])).assert_ne_allow_empty(&T::new_from_str("")?);
        T::new_from_str("")?.assert_ne_allow_empty(&T(Some(vec![])));
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&"".try_into()?, true);
        T(Some(vec!["One".try_into()?])).assert_eq_allow_empty(&"One".try_into()?, true);
        T(Some(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&"One.Two".try_into()?, true);
        assert!(T::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        T(Some(vec!["One".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        T(Some(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        T(Some(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.Two").into(), true);
        T(Some(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")])).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        T(Some(vec!["One".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        T(Some(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        T(Some(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.Two").into(), true);
        T(Some(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_ne!(T(None), T(None));
        assert_ne!(T(None), T(Some(vec![])));
        assert_ne!(T(Some(vec![])), T(None));
        assert_eq!(T(Some(vec![])), T(Some(vec![])));
        assert_eq!(T(Some(vec!["One".try_into()?])), T(Some(vec!["One".try_into()?])));
        assert_eq!(T(Some(vec!["One".try_into()?, "Two".try_into()?])), T(Some(vec!["One".try_into()?, "Two".try_into()?])));
        assert_ne!(T(Some(vec!["One".try_into()?, "Two".try_into()?])), T(Some(vec!["One".try_into()?])));
        assert_ne!(T(Some(vec!["Two".try_into()?, "One".try_into()?])), T(Some(vec!["One".try_into()?, "Two".try_into()?])));
        Ok(())
    }

    #[test]
    fn test_none() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&T::none(), true);
        Ok(())
    }

    #[test]
    fn test_default() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&Default::default(), true);
        Ok(())
    }
}