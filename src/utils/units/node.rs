//! Module for the unit Node used in GDTF
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::utils::errors::GdtfError;

///Node representation used in GDTF
#[derive(Debug)]
pub struct Node {
    ///The string value of the name
    value: String,
}

impl Node {
    pub fn new() -> Node {
        Node {
            value: String::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value == ""
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<&str> for Node {
    type Error = GdtfError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Node { value: String::from(s) })
    }
}

#[cfg(test)]
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::node::Node;

    #[test]
    fn test_valid() {
        assert_eq!(
            Node { value: "Node".to_string() },
            Node::try_from("Node").unwrap()
        );
    }

    #[test]
    fn test_invalid_2() {
        assert_eq!(
            Node { value: "something invalid".to_string() },
            Node::try_from("something invalid").unwrap()
        );
    }

    #[test]
    fn test_invalid_3() {
        assert_eq!(
            Node { value: "".to_string() },
            Node::try_from("").unwrap()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Node { value: "Node".to_string() }), "Node");
    }
}