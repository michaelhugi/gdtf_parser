use std::borrow::Borrow;
use std::convert::TryInto;
use std::fmt::{Display, Formatter};

use quick_xml::events::attributes::Attribute;

use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::units::node::{fmt, GDTFNodeError, Node};

pub trait NodeOption {
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
}

#[cfg(test)]
mod tests {
    use quick_xml::events::BytesStart;
    use quick_xml::Reader;

    use crate::utils::deparse::DeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::units::node::Node;
    use crate::utils::units::node::node_option::NodeOption;

    #[derive(Debug)]
    struct TestNodeOption(Option<Node>);

    impl NodeOption for TestNodeOption {}

    impl PartialEqAllowEmpty for TestNodeOption {
        fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
            Self::is_eq_allow_option(&self.0, &other.0)
        }
    }

    #[test]
    fn test_try_read_from_str() -> Result<(), GdtfError> {
        TestNodeOption(None).assert_eq_allow_empty(&TestNodeOption(TestNodeOption::try_read_from_str("")?), true);
        Ok(())
    }

    #[test]
    fn test_read_option_from_attr_owned() -> Result<(), GdtfError> {
        Ok(())
    }


    #[test]
    fn test_fmt_option() -> Result<(), GdtfError> { Ok(()) }
}