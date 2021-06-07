//! Module for the unit Master for LogicalChannel in GDTF

use quick_xml::events::attributes::Attribute;

use crate::utils::deparse;

///Master representation for logicalChannel in GDTF
///Defines if all the subordinate channel functions react to a Group Control defined by the control system
#[derive(Debug, PartialEq, Clone)]
pub enum LogicalChannelMaster {
    None,
    Grand,
    Group,
}

///```rust
/// use gdtf_parser::utils::units::logical_channel_master::LogicalChannelMaster;
/// assert_eq!(LogicalChannelMaster::None, Default::default());
///```
impl Default for LogicalChannelMaster {
    fn default() -> Self {
        LogicalChannelMaster::None
    }
}

impl LogicalChannelMaster {
    ///Parses a string defined in gdtf-xml-description to a Master
    /// ```rust
    /// use gdtf_parser::utils::units::logical_channel_master::LogicalChannelMaster;
    /// assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_str("None"));
    /// assert_eq!(LogicalChannelMaster::Grand, LogicalChannelMaster::new_from_str("Grand"));
    /// assert_eq!(LogicalChannelMaster::Group, LogicalChannelMaster::new_from_str("Group"));
    /// assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_str("Anything strange like ȸ"));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        use LogicalChannelMaster::*;
        match s {
            "Grand" => Grand,
            "Group" => Group,
            _ => None
        }
    }

    ///Parses a quick-xml-attribute defined in gdtf-xml-description to a Master
    /// ```rust
    /// use gdtf_parser::utils::units::logical_channel_master::LogicalChannelMaster;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"None")}));
    /// assert_eq!(LogicalChannelMaster::Grand, LogicalChannelMaster::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Grand")}));
    /// assert_eq!(LogicalChannelMaster::Group, LogicalChannelMaster::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Group")}));
    /// assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Anything strange like {")}));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::logical_channel_master::LogicalChannelMaster;

    #[test]
    fn test_default() {
        assert_eq!(LogicalChannelMaster::None, Default::default())
    }

    #[test]
    fn test_new_from_str() {
        assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_str("None"));
        assert_eq!(LogicalChannelMaster::Grand, LogicalChannelMaster::new_from_str("Grand"));
        assert_eq!(LogicalChannelMaster::Group, LogicalChannelMaster::new_from_str("Group"));
        assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_str("Anything strange like ȸ"));
    }

    #[test]
    fn test_new_from_attr_owned() {
        assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_attr(testdata::to_attr_owned(b"None")));
        assert_eq!(LogicalChannelMaster::Grand, LogicalChannelMaster::new_from_attr(testdata::to_attr_owned(b"Grand")));
        assert_eq!(LogicalChannelMaster::Group, LogicalChannelMaster::new_from_attr(testdata::to_attr_owned(b"Group")));
        assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_attr(testdata::to_attr_owned(b"Anything strange like {")));
    }

    #[test]
    fn test_new_from_attr_borrowed() {
        assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_attr(testdata::to_attr_borrowed(b"None")));
        assert_eq!(LogicalChannelMaster::Grand, LogicalChannelMaster::new_from_attr(testdata::to_attr_borrowed(b"Grand")));
        assert_eq!(LogicalChannelMaster::Group, LogicalChannelMaster::new_from_attr(testdata::to_attr_borrowed(b"Group")));
        assert_eq!(LogicalChannelMaster::None, LogicalChannelMaster::new_from_attr(testdata::to_attr_borrowed(b"Anything strange like {")));
    }
}