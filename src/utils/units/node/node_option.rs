use std::borrow::Borrow;
use std::convert::TryInto;
use std::fmt::{Display, Formatter};

use quick_xml::events::attributes::Attribute;

use crate::utils::units::node::{fmt, GDTFNodeError, Node};

#[cfg(test)]
pub trait NodeOption: Display {
    fn get_node_option(&self) -> &Option<Node>;

    ///Needed to compare none values even when partial_eq returns false
    fn assert_eq_including_none(&self, other: &Self) {
        match self.get_node_option() {
            None => if let None = other.get_node_option() {} else { panic!("left: None\n right: {}", other.get_node_option().as_ref().unwrap()) }
            Some(v1) => if let Some(v2) = other.get_node_option() { assert_eq!(v1, v2); } else { panic!("left: {}\n right: None", v1) }
        }
    }

    fn eq_exluding_none(&self, other: &Self) -> bool {
        match &self.get_node_option() {
            None => false,
            Some(v1) => if let Some(v2) = &other.get_node_option() { v1 == v2 } else { false }
        }
    }

    fn read_option_from_attr(attr: Attribute<'_>) -> Option<Node> {
        let value = match std::str::from_utf8(attr.value.borrow()) {
            Ok(s) => s,
            Err(_) => ""
        };
        if value == "" {
            None
        } else {
            Some(attr.into())
        }
    }

    fn try_read_from_str(value: &str) -> Result<Option<Node>, GDTFNodeError> {
        if value == "" {
            Ok(None)
        } else {
            Ok(Some(value.try_into()?))
        }
    }

    fn fmt_option(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.get_node_option() {
            None => write!(f, ""),
            Some(val) => write!(f, "{}", val)
        }
    }
}