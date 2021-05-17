//! Module for the unit Node used in GDTF
use std::fmt::{Display, Formatter};

///Node representation used in GDTF
#[derive(Debug)]
pub struct Node {
    ///The string value of the name
    pub value: String,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            value: String::new()
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        Self { value: String::from(s) }
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