//! Module for the unit Node used in GDTF
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
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
pub struct Node(Vec<Name>);

impl Default for Node {
    fn default() -> Self {
        Node(vec![])
    }
}

pub trait NodeHelper: PartialEqAllowEmpty {
    ///Creates a vec of Names from a vec of str where names are checked for validity defined by GDTF
    fn strs_to_names(names: Vec<&str>) -> Result<Vec<Name>, GDTFNodeError> {
        let mut ns = vec![];
        for name in names.iter() {
            ns.push(Name::new(name)?)
        }
        Ok(ns)
    }

    ///creates a vec of Names from single name (&str) where name is checked for validity defined by GDTF
    fn str_to_names(name: &str) -> Result<Vec<Name>, GDTFNodeError> {
        Ok(vec![Name::new(name)?])
    }

    #[cfg(test)]
    ///creates a new vec of Name from vec of str  where names are not checked for validity defined by GDTF
    fn strs_to_names_unchecked(names: Vec<&str>) -> Vec<Name> {
        let mut ns = vec![];
        for name in names.iter() {
            ns.push(Name::new_unchecked(name))
        }
        ns
    }
    #[cfg(test)]
    ///creates a new vec of Nams from single str where name is not checked for validity defined by GDTF
    fn str_to_names_unchecked(name: &str) -> Vec<Name> {
        vec![Name::new_unchecked(name)]
    }


}


impl Node {
    ///creates a new Node from vec of names (&str) where names are checked for validity defined by GDTF
    pub fn new(names: Vec<&str>) -> Result<Self, GDTFNodeError> {
        let mut ns = vec![];
        for name in names.iter() {
            ns.push(Name::new(name)?)
        }
        Ok(Self(ns))
    }

    ///creates a new Node from single name (&str) where name is checked for validity defined by GDTF
    pub fn new_s(name: &str) -> Result<Self, GDTFNodeError> {
        Ok(Self(vec![Name::new(name)?]))
    }
    #[cfg(test)]
    ///creates a new Node from vec of names (&str) where names are not checked for validity defined by GDTF
    pub(crate) fn new_unchecked(names: Vec<&str>) -> Self {
        let mut ns = vec![];
        for name in names.iter() {
            ns.push(Name::new_unchecked(name))
        }
        Self(ns)
    }
    #[cfg(test)]
    ///creates a new Node from single name (&str) where name is not checked for validity defined by GDTF
    pub(crate) fn new_unchecked_s(name: &str) -> Self {
        Self(vec![Name::new_unchecked(name)])
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
                    v.push(Name::new_unchecked(val));
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

#[cfg(test)]
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
    fn test_new() -> Result<(), GdtfError> {
        Node(vec![Name::new("one")?]).assert_eq_allow_empty(&Node::new(vec!["one"]).unwrap(), true);
        Node(vec![Name::new("one")?, Name::new("two")?]).assert_eq_allow_empty(&Node::new(vec!["one", "two"]).unwrap(), true);
        Node(vec![Name::new("")?, Name::new("two")?]).assert_eq_allow_empty(&Node::new(vec!["", "two"]).unwrap(), true);
        Node(vec![]).assert_eq_allow_empty(&Node::new(vec![]).unwrap(), true);
        assert!(Node::new(vec!["asdf{"]).is_err());
        assert!(Node::new(vec!["some", "asdf{"]).is_err());
        Ok(())
    }

    #[test]
    fn test_new_s() -> Result<(), GdtfError> {
        Node(vec![Name::new("one")?]).assert_eq_allow_empty(&Node::new_s("one").unwrap(), true);
        Node(vec![Name::new("")?]).assert_eq_allow_empty(&Node::new_s("").unwrap(), true);
        assert!(Node::new_s("asdf{").is_err());
        Ok(())
    }

    #[test]
    fn test_new_unchecked() -> Result<(), GdtfError> {
        Node(vec![Name::new("one")?]).assert_eq_allow_empty(&Node::new_unchecked(vec!["one"]), true);
        Node(vec![Name::new("one")?, Name::new("two")?]).assert_eq_allow_empty(&Node::new_unchecked(vec!["one", "two"]), true);
        Node(vec![Name::new("")?, Name::new("two")?]).assert_eq_allow_empty(&Node::new_unchecked(vec!["", "two"]), true);
        Node(vec![]).assert_eq_allow_empty(&Node::new_unchecked(vec![]), true);
        Node(vec![Name::new_unchecked("asdf{")]).assert_eq_allow_empty(&Node::new_unchecked(vec!["asdf{"]), true);
        Node(vec![Name::new("one")?, Name::new_unchecked("asdf{")]).assert_eq_allow_empty(&Node::new_unchecked(vec!["one", "asdf{"]), true);
        Ok(())
    }

    #[test]
    fn test_new_unchecked_s() -> Result<(), GdtfError> {
        Node(vec![Name::new("one")?]).assert_eq_allow_empty(&Node::new_unchecked_s("one"), true);
        Node(vec![Name::new("")?]).assert_eq_allow_empty(&Node::new_unchecked_s(""), true);
        Node(vec![Name::new_unchecked("asdf{")]).assert_eq_allow_empty(&Node::new_unchecked_s("asdf{"), true);
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
        Node(vec!["One".try_into()?, Name::new("")?, "Three".try_into()?]).assert_eq_allow_empty(&"One..Three".try_into()?, true);
        Node(vec![]).assert_eq_allow_empty(&"".try_into()?, true);
        Node(vec![Name::new("")?, Name::new("")?]).assert_eq_allow_empty(&".".try_into()?, true);
        assert!(Node::try_from("One{Name").is_err());
        assert!(Node::try_from("One.Two{Name").is_err());
        Ok(())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        Node(vec!["One".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        Node(vec!["One".try_into()?, "Two".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        Node(vec!["One".try_into()?, "Two".try_into()?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two.Three").into(), true);
        Node(vec!["One".try_into()?, Name::new("")?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One..Three").into(), true);
        Node(vec![]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        Node(vec![Name::new("")?, Name::new("")?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b".").into(), true);
        Node(vec![Name::new_unchecked("On{e"), "Two".try_into()?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"On{e.Two.Three").into(), true);
        Node(vec![Name::new_unchecked("On{e"), Name::new_unchecked("Tw{o"), "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"On{e.Tw{o.Three").into(), true);
        Node(vec![Name::new_unchecked("On{e")]).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"On{e").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        Node(vec!["One".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        Node(vec!["One".try_into()?, "Two".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        Node(vec!["One".try_into()?, "Two".try_into()?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two.Three").into(), true);
        Node(vec!["One".try_into()?, Name::new("")?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"One..Three").into(), true);
        Node(vec![]).assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        Node(vec![Name::new("")?, Name::new("")?]).assert_eq_allow_empty(&testdata::to_attr_owned(b".").into(), true);
        Node(vec![Name::new_unchecked("On{e"), "Two".try_into()?, "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"On{e.Two.Three").into(), true);
        Node(vec![Name::new_unchecked("On{e"), Name::new_unchecked("Tw{o"), "Three".try_into()?]).assert_eq_allow_empty(&testdata::to_attr_owned(b"On{e.Tw{o.Three").into(), true);
        Node(vec![Name::new_unchecked("On{e")]).assert_eq_allow_empty(&testdata::to_attr_owned(b"On{e").into(), true);
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_eq!(Node::new(vec![])?, Node::new(vec![])?);
        assert_eq!(Node::new(vec!["One"])?, Node::new(vec!["One"])?);
        assert_eq!(Node::new(vec!["One", "Two"])?, Node::new(vec!["One", "Two"])?);
        assert_ne!(Node::new(vec![])?, Node::new(vec!["One", "Two"])?);
        assert_ne!(Node::new(vec!["One", "Two"])?, Node::new(vec![])?);
        assert_ne!(Node::new(vec!["One", "Two"])?, Node::new(vec!["Two", "One"])?);
        assert_ne!(Node::new(vec!["One", "Two"])?, Node::new(vec!["One"])?);
        assert_ne!(Node::new(vec![""])?, Node::new(vec![""])?);
        assert_ne!(Node::new(vec!["One", ""])?, Node::new(vec!["One", ""])?);
        Ok(())
    }

    #[test]
    fn test_partial_eq_allow_empty() -> Result<(), GdtfError> {
        Node::new(vec![])?.assert_eq_allow_empty(&Node::new(vec![])?, true);
        Node::new(vec!["One"])?.assert_eq_allow_empty(&Node::new(vec!["One"])?, true);
        Node::new(vec!["One", "Two"])?.assert_eq_allow_empty(&Node::new(vec!["One", "Two"])?, true);
        Node::new(vec![""])?.assert_eq_allow_empty(&Node::new(vec![""])?, true);
        Node::new(vec!["One", ""])?.assert_eq_allow_empty(&Node::new(vec!["One", ""])?, true);

        assert!(!Node::new(vec![])?.is_eq_allow_empty(&Node::new(vec!["One", "Two"])?, false));
        assert!(!Node::new(vec!["One", "Two"])?.is_eq_allow_empty(&Node::new(vec![])?, false));
        assert!(!Node::new(vec!["One", "Two"])?.is_eq_allow_empty(&Node::new(vec!["Two", "One"])?, false));
        assert!(!Node::new(vec!["One", "Two"])?.is_eq_allow_empty(&Node::new(vec!["One"])?, false));

        Ok(())
    }
}