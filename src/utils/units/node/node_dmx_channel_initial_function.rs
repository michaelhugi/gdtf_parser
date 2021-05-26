//!Module for Node used in DmxChannel.initial_function
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use quick_xml::events::attributes::Attribute;

use crate::utils::units::node::Node;
use std::fmt;

#[derive(Debug)]
pub struct NodeDmxChannelInitialFunction {
    pub tree: Node,
}

impl Default for NodeDmxChannelInitialFunction {
    fn default() -> Self {
        todo!()
    }
}

impl From<Attribute<'_>> for NodeDmxChannelInitialFunction {
    fn from(_: Attribute<'_>) -> Self {
        todo!()
    }
}

impl TryFrom<&str> for NodeDmxChannelInitialFunction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
impl PartialEq for NodeDmxChannelInitialFunction {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Display for NodeDmxChannelInitialFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
/*
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