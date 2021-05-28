//! Module for the unit Node used in GDTF
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::units::name::{GDTFNameError, Name};

pub mod node_dmx_channel_initial_function;
pub mod node_logical_channel_attribute;
pub mod node_channel_function_attribute;
pub mod node_channel_function_wheel;
pub mod node_channel_function_emitter;
pub mod node_channel_function_filter;
pub mod node_channel_function_mode_master;
pub mod node_option;

///Node representation used in GDTF. A Node is a link to another xml node
#[derive(Debug)]
pub struct Node(pub Vec<Name>);


///Displays the tree of the node_2 without starting point
impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();
        for s2 in self.0.iter() {
            s = format!("{}.{}", s, s2);
        }
        let s = &s[1..];
        write!(f, "{}", s)
    }
}

impl Default for Node {
    fn default() -> Self {
        Node(vec![])
    }
}

///Parses a tuple of str and the starting point to a node_2, where str is in format Name.Name.Name...
impl TryFrom<&str> for Node {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "" {
            return Ok(Self(vec![]));
        }
        let value = value.split(".");
        let mut tree: Vec<Name> = vec![];
        for value in value.into_iter() {
            tree.push(value.try_into()?);
        }
        Ok(Self(tree))
    }
}

///Parses an xml attribute to a node, where attribute is in format Name.Name.Name... In case of error the default value is returned
impl From<Attribute<'_>> for Node {
    fn from(val: Attribute<'_>) -> Self {
        match std::str::from_utf8(val.value.borrow()) {
            Ok(val) => {
                if val == "" {
                    return Node(vec![]);
                }
                let mut v = vec![];
                for val in val.split(".").into_iter() {
                    if val != "" {
                        v.push(Name::Name(val.to_string()));
                    } else {
                        v.push(Name::Empty);
                    }
                }
                Node(v)
            }
            Err(_) => Self::default()
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEqAllowEmpty for Node {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        Self::is_vec_eq_ordered(&self.0, &other.0, log)
    }
}

#[derive(Debug)]
/// Error that occures if the format of GUID is wrong e.q. not XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX
pub struct GDTFNodeError {}

impl Display for GDTFNodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrong argument for Node in GDTF. Format must be Name.Name.Name...!")
    }
}

impl From<GDTFNameError> for GDTFNodeError {
    fn from(_: GDTFNameError) -> Self {
        GDTFNodeError {}
    }
}

impl Error for GDTFNodeError {}


#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;

    #[test]
    fn test_display() -> Result<(), GdtfError> {
        assert_eq!(
            format!("{}", Node(vec!["One".try_into()?, "Two".try_into()?, "Three".try_into()?, "Four".try_into()?, "Five".try_into()?])),
            "One.Two.Three.Four.Five"
        );
        assert_eq!(
            format!("{}", Node(vec!["One".try_into()?])),
            "One"
        );
        Ok(())
    }

    #[test]
    fn test_default() {
        Node(vec![]).assert_eq_allow_empty(&Default::default(), true);
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        Node(vec!["One".try_into()?]).assert_eq_allow_empty(&"One".try_into()?, true);
        Node(vec!["One".try_into()?, "Two".try_into()?]).assert_eq_allow_empty(&"One.Two".try_into()?, true);
        Node(vec!["One".try_into()?, "Two".try_into()?, "Three".try_into()?]).assert_eq_allow_empty(&"One.Two.Three".try_into()?, true);
        Node(vec!["One".try_into()?, Name::Empty, "Three".try_into()?]).assert_eq_allow_empty(&"One..Three".try_into()?, true);
        Node(vec![]).assert_eq_allow_empty(&"".try_into()?, true);
        Node(vec![Name::Empty, Name::Empty]).assert_eq_allow_empty(&".".try_into()?, true);
        assert!(Node::try_from("One{Name").is_err());
        assert!(Node::try_from("One.Two{Name").is_err());
        Ok(())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        Node(vec!["One".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        Node(vec!["One".try_into()?, "Two".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        Node(vec!["One".try_into()?, "Two".try_into()?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two.Three").into(), true);
        Node(vec!["One".try_into()?, Name::Empty, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One..Three").into(), true);
        Node(vec![]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        Node(vec![Name::Empty, Name::Empty]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b".").into(), true);
        Node(vec![Name::Name("On{e".to_string()), "Two".try_into()?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"On{e.Two.Three").into(), true);
        Node(vec![Name::Name("On{e".to_string()), Name::Name("Tw{o".to_string()), "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"On{e.Tw{o.Three").into(), true);
        Node(vec![Name::Name("On{e".to_string())]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"On{e").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        Node(vec!["One".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        Node(vec!["One".try_into()?, "Two".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        Node(vec!["One".try_into()?, "Two".try_into()?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two.Three").into(), true);
        Node(vec!["One".try_into()?, Name::Empty, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One..Three").into(), true);
        Node(vec![]).assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        Node(vec![Name::Empty, Name::Empty]).assert_eq_allow_empty(&testdata::to_attr_owned(b".").into(), true);
        Node(vec![Name::Name("On{e".to_string()), "Two".try_into()?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"On{e.Two.Three").into(), true);
        Node(vec![Name::Name("On{e".to_string()), Name::Name("Tw{o".to_string()), "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"On{e.Tw{o.Three").into(), true);
        Node(vec![Name::Name("On{e".to_string())]).assert_eq_allow_empty(&testdata::to_attr_owned(b"On{e").into(), true);
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_eq!(Node(vec![]), Node(vec![]));
        assert_eq!(Node(vec!["One".try_into()?]), Node(vec!["One".try_into()?]));
        assert_eq!(Node(vec!["One".try_into()?, "Two".try_into()?]), Node(vec!["One".try_into()?, "Two".try_into()?]));
        assert_ne!(Node(vec![]), Node(vec!["One".try_into()?, "Two".try_into()?]));
        assert_ne!(Node(vec!["One".try_into()?, "Two".try_into()?]), Node(vec![]));
        assert_ne!(Node(vec!["One".try_into()?, "Two".try_into()?]), Node(vec!["Two".try_into()?, "One".try_into()?]));
        assert_ne!(Node(vec!["One".try_into()?, "Two".try_into()?]), Node(vec!["One".try_into()?]));
        assert_ne!(Node(vec![Name::Empty]), Node(vec![Name::Empty]));
        assert_ne!(Node(vec!["One".try_into()?, Name::Empty]), Node(vec!["One".try_into()?, Name::Empty]));
        Ok(())
    }

    #[test]
    fn test_partial_eq_allow_empty() -> Result<(), GdtfError> {
        Node(vec![]).assert_eq_allow_empty(&Node(vec![]), true);

        Node(vec!["One".try_into()?]).assert_eq_allow_empty(&Node(vec!["One".try_into()?]), true);

        Node(vec!["One".try_into()?, "Two".try_into()?]).assert_eq_allow_empty(&Node(vec!["One".try_into()?, "Two".try_into()?]), true);

        Node(vec![Name::Empty]).assert_eq_allow_empty(&Node(vec![Name::Empty]), true);
        Node(vec!["One".try_into()?, Name::Empty]).assert_eq_allow_empty(&Node(vec!["One".try_into()?, Name::Empty]), true);

        assert!(!Node(vec![]).is_eq_allow_empty(&Node(vec!["One".try_into()?, "Two".try_into()?]), false));
        assert!(!Node(vec!["One".try_into()?, "Two".try_into()?]).is_eq_allow_empty(&Node(vec![]), false));
        assert!(!Node(vec!["One".try_into()?, "Two".try_into()?]).is_eq_allow_empty(&Node(vec!["Two".try_into()?, "One".try_into()?]), false));
        assert!(!Node(vec!["One".try_into()?, "Two".try_into()?]).is_eq_allow_empty(&Node(vec!["One".try_into()?]), false), true);
        assert!(!Node(vec![Name::Name("".to_string()), "Two".try_into()?]).is_eq_allow_empty(&Node(vec![Name::Empty, "Two".try_into()?]), false));
        assert!(!Node(vec!["One".try_into()?, Name::Name("".to_string())]).is_eq_allow_empty(&Node(vec!["One".try_into()?, Name::Empty]), false));


        Ok(())
    }
}