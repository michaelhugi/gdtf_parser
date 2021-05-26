//!Module for Node used in LogicalChannel.attribute
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

use crate::utils::units::node::Node;

///Link to the attribute; The starting point is the Attribute Collect (see Annex A).
#[derive(Debug)]
pub struct NodeLogicalChannelAttribute {
    pub tree: Node,
}

impl From<Attribute<'_>> for NodeLogicalChannelAttribute {
    fn from(_: Attribute<'_>) -> Self {
        todo!()
    }
}

impl TryFrom<&str> for NodeLogicalChannelAttribute {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
impl PartialEq for NodeLogicalChannelAttribute {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Default for NodeLogicalChannelAttribute {
    fn default() -> Self {
        todo!()
    }
}

impl Display for NodeLogicalChannelAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}/*
#[cfg(test)]
mod tests {
    #[test]
    fn test_from_attr_borrowed() { unimplemented!() }

    #[test]
    fn test_from_attr_owned() { unimplemented!() }

    #[test]
    fn test_try_from_str() { unimplemented!() }

    #[test]
    fn test_partial_eq() {
        unimplemented!()
    }

    #[test]
    fn test_default() {
        unimplemented!()
    }   #[test]
    fn test_display(){unimplemented!()}
}*/