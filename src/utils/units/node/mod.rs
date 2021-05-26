//! Module for the unit Node used in GDTF
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

use crate::utils::units::name::{GDTFNameError, Name};

pub mod node_dmx_channel_initial_function;
pub mod node_logical_channel_attribute;
pub mod node_channel_function_attribute;
pub mod node_channel_function_wheel;
pub mod node_channel_function_emitter;
pub mod node_channel_function_filter;
pub mod node_channel_function_mode_master;

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
        let value = value.split(".");
        let mut tree: Vec<Name> = vec![];
        for value in value.into_iter() {
            tree.push(value.try_into()?);
        }
        Ok(Self { 0: tree })
    }
}

///Parses an xml attribute to a node, where attribute is in format Name.Name.Name... In case of error the default value is returned
impl From<Attribute<'_>> for Node {
    fn from(val: Attribute<'_>) -> Self {
        match std::str::from_utf8(val.value.borrow()) {
            Ok(val) => {
                let mut v = vec![];
                for val in val.split(".").into_iter() {
                    if (val != "") {
                        v.push(Name::Name(val.to_string()));
                    }
                }
                Node(v)
            }
            Err(_) => Self::default()
        }
    }
}

#[cfg(test)]
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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
pub trait NodeOption: Display {
    fn get_node_option(&self) -> &Option<Node>;

    ///Needed to compare none values even when partial_eq returns false
    fn assert_eq(&self, other: &Self) {
        match self.get_node_option() {
            None => if let None = other.get_node_option() {} else { panic!(format!("left: None\n right: {}", other.get_node_option().as_ref().unwrap())) }
            Some(v1) => if let Some(v2) = other.get_node_option() { assert_eq!(v1, v2); } else { panic!(format!("left: {}\n right: None", v1)) }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap(), "Three".try_into().unwrap(), "Four".try_into().unwrap(), "Five".try_into().unwrap()])),
            "One.Two.Three.Four.Five"
        );
        assert_eq!(
            format!("{}", Node(vec!["One".try_into().unwrap()])),
            "One"
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Node(vec![]), Default::default());
    }

    #[test]
    fn test_try_from_str() {
        assert_eq!(Node(vec!["One".try_into().unwrap()]),
                   "One".try_into().unwrap());
        assert_eq!(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]),
                   "One.Two".try_into().unwrap());
        assert_eq!(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap(), "Three".try_into().unwrap()]),
                   "One.Two.Three".try_into().unwrap());
        assert!(Node::try_from("One{Name").is_err());
        assert!(Node::try_from("One.Two{Name").is_err());
    }

    #[test]
    fn test_from_attr_borrowed() {
        assert_eq!(Node(vec!["One".try_into().unwrap()]),
                   testdata::to_attr_borrowed(b"One").into());
        assert_eq!(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]),
                   testdata::to_attr_borrowed(b"One.Two").into());
        assert_eq!(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap(), "Three".try_into().unwrap()]),
                   testdata::to_attr_borrowed(b"One.Two.Three").into());
        assert_eq!(Node(vec![Name::Name("On{e".to_string()), "Two".try_into().unwrap(), "Three".try_into().unwrap()]),
                   testdata::to_attr_borrowed(b"On{e.Two.Three").into());
        assert_eq!(Node(vec![Name::Name("On{e".to_string()), Name::Name("Tw{o".to_string()), "Three".try_into().unwrap()]),
                   testdata::to_attr_borrowed(b"On{e.Tw{o.Three").into());
        assert_eq!(Node(vec![Name::Name("On{e".to_string())]),
                   testdata::to_attr_borrowed(b"On{e").into());
    }

    #[test]
    fn test_from_attr_owned() {
        assert_eq!(Node(vec!["One".try_into().unwrap()]),
                   testdata::to_attr_owned(b"One").into());
        assert_eq!(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]),
                   testdata::to_attr_owned(b"One.Two").into());
        assert_eq!(Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap(), "Three".try_into().unwrap()]),
                   testdata::to_attr_owned(b"One.Two.Three").into());
        assert_eq!(Node(vec![Name::Name("On{e".to_string()), "Two".try_into().unwrap(), "Three".try_into().unwrap()]),
                   testdata::to_attr_owned(b"On{e.Two.Three").into());
        assert_eq!(Node(vec![Name::Name("On{e".to_string()), Name::Name("Tw{o".to_string()), "Three".try_into().unwrap()]),
                   testdata::to_attr_owned(b"On{e.Tw{o.Three").into());
        assert_eq!(Node(vec![Name::Name("On{e".to_string())]),
                   testdata::to_attr_owned(b"On{e").into());
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(
            Node(vec![]),
            Node(vec![])
        );
        assert_eq!(
            Node(vec!["One".try_into().unwrap()]),
            Node(vec!["One".try_into().unwrap()])
        );
        assert_eq!(
            Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]),
            Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()])
        );
        assert_ne!(
            Node(vec![]),
            Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()])
        );
        assert_ne!(
            Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]),
            Node(vec![])
        );
        assert_ne!(
            Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]),
            Node(vec!["Two".try_into().unwrap(), "One".try_into().unwrap()]),
        );
        assert_ne!(
            Node(vec!["One".try_into().unwrap(), "Two".try_into().unwrap()]),
            Node(vec!["One".try_into().unwrap()]),
        );
    }
}