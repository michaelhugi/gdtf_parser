//! Module for the unit Node used in GDTF
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::str::Utf8Error;

use quick_xml::events::attributes::Attribute;

use crate::utils::units::name::Name;

///Node representation used in GDTF. A Node is a link to another xml node
#[derive(Debug)]
pub struct Node {
    ///The starting point to start finding the node
    pub starting_point: NodeStartingPoint,
    ///The tree of names where to finde the node
    pub tree: Vec<Name>,
}

pub enum NodeStartingPoint {
    SELF,
    ATTRIBUTE_COLLECT,
    Dummy2,
}

///Default value is an empty tree with no starting point
impl Default for Node {
    fn default() -> Self {
        Node { starting_point: NodeStartingPoint::NO_STARTING_POINT, tree: vec![] }
    }
}

///Displays the tree of the node without starting point
impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!();
    }
}

///Parses a tuple of str and the starting point to a node, where str is in format Name.Name.Name...
impl TryFrom<(&str, NodeStartingPoint)> for Node {
    type Error = GDTFNodeError;

    fn try_from(v: (&str, NodeStartingPoint)) -> Result<Self, Self::Error> {
        let value = v.0.split(".");
        let mut tree: Vec<Name> = vec![];
        for value in value.into_iter() {
            tree.push(value.try_into()?);
        }
        Ok(Node { starting_point: v.1, tree })
    }
}

///Parses a tuple of xml attribute and the starting point to a node, where attribute is in format Name.Name.Name... In case of error the default value is returned
impl From<(Attribute<'_>, NodeStartingPoint)> for Node {
    fn from(v: (Attribute<'_>, NodeStartingPoint)) -> Self {
        match std::str::from_utf8(v.0.value.borrow()) {
            Ok(val) => {
                match Self::try_from((val, v.1)) {
                    Ok(val) => val,
                    Err(_) => Self::default()
                }
            }
            Err(_) => Self::default()
        }
    }
}


#[cfg(test)]
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.tree == other.tree &&
            self.starting_point == other.starting_point
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

impl Error for GDTFNodeError {}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::node::Node;

    #[test]
    fn test_default() { unimplemented!(); }

    #[test]
    fn test_partial_eq() { unimplemented!(); }

    #[test]
    fn test_display() { unimplemented!(); }

    #[test]
    fn test_try_from_str() { unimplemented!(); }

    #[test]
    fn test_try_from_attr_owned() { unimplemented!(); }

    #[test]
    fn test_try_from_attr_borrowed() { unimplemented!(); }
}