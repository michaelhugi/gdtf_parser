//!Module for Node used in ChannelFunction.wheel
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::fmt;

use quick_xml::events::attributes::Attribute;

use crate::utils::units::node::Node;

#[derive(Debug)]
pub struct NodeChannelFunctionWheel(pub Option<Node>);

impl From<Attribute<'_>> for NodeChannelFunctionWheel {
    fn from(_: Attribute<'_>) -> Self {
        todo!()
    }
}

impl TryFrom<&str> for NodeChannelFunctionWheel {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
impl PartialEq for NodeChannelFunctionWheel {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Default for NodeChannelFunctionWheel {
    fn default() -> Self {
        todo!()
    }
}

impl Display for NodeChannelFunctionWheel {
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