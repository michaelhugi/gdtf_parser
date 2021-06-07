//! Module for the unit DMXBreak used for DmxChannel  in GDTF
//!
//! # Definition of DMXBreak
//! term used when a fixture needs more than one DMX start address

use std::str::FromStr;

use quick_xml::events::attributes::Attribute;

use crate::utils::deparse;

///DMXBreak used for DMXChannel in GDTF
#[derive(Debug, PartialEq, Clone)]
pub enum DmxChannelDmxBreak {
    ///Number of the DMXBreak; Default value: 1
    Value(u32),
    ///means that this number will be overwritten by Geometry Reference
    Overwrite,
}

impl DmxChannelDmxBreak {
    ///Parses a string used in gdtf-xml-description to a DmxChannelDmxBreak
    /// ```rust
    /// use gdtf_parser::utils::units::dmx_channel_dmx_break::DmxChannelDmxBreak;
    /// assert_eq!(DmxChannelDmxBreak::new_from_str("32"), DmxChannelDmxBreak::Value(32));
    /// assert_eq!(DmxChannelDmxBreak::new_from_str("Overwrite"), DmxChannelDmxBreak::Overwrite);
    /// assert_eq!(DmxChannelDmxBreak::new_from_str("Anything else"), DmxChannelDmxBreak::Value(1));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        use DmxChannelDmxBreak::*;
        if s == "Overwrite" {
            Overwrite
        } else {
            Value(u32::from_str(s).unwrap_or(1))
        }
    }
    ///Parses a quick-xml-attribute from gdtf-xml-description to a DmxChannelDmxBreak
    /// ```rust
    /// use gdtf_parser::utils::units::dmx_channel_dmx_break::DmxChannelDmxBreak;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(DmxChannelDmxBreak::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"32")}), DmxChannelDmxBreak::Value(32));
    /// assert_eq!(DmxChannelDmxBreak::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Overwrite")}), DmxChannelDmxBreak::Overwrite);
    /// assert_eq!(DmxChannelDmxBreak::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Anything else")}), DmxChannelDmxBreak::Value(1));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}

///```rust
/// use gdtf_parser::utils::units::dmx_channel_dmx_break::DmxChannelDmxBreak;
/// assert_eq!(DmxChannelDmxBreak::Value(1), Default::default());
/// ```
impl Default for DmxChannelDmxBreak {
    fn default() -> Self {
        Self::Value(1)
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::dmx_channel_dmx_break::DmxChannelDmxBreak as T;

    #[test]
    fn test_new_from_str() {
        assert_eq!(T::Value(23), T::new_from_str("23"));
        assert_eq!(T::Value(1), T::new_from_str("1"));
        assert_eq!(T::Value(145), T::new_from_str("145"));
        assert_eq!(T::Overwrite, T::new_from_str("Overwrite"));
        assert_eq!(T::Value(1), T::new_from_str("Something else"));
        assert_eq!(T::Value(1), T::new_from_str("23a"));
        assert_eq!(T::Value(1), T::new_from_str(""));
        assert_eq!(T::Value(1), T::new_from_str("a3"));
        assert_eq!(T::Value(1), T::new_from_str("Overwritee"));
    }

    #[test]
    fn test_new_from_attr_owned_valid() {
        assert_eq!(T::Value(23), T::new_from_attr(testdata::to_attr_owned(b"23")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_owned(b"1")));
        assert_eq!(T::Value(145), T::new_from_attr(testdata::to_attr_owned(b"145")));
        assert_eq!(T::Overwrite, T::new_from_attr(testdata::to_attr_owned(b"Overwrite")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_owned(b"Something else")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_owned(b"23a")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_owned(b"")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_owned(b"a3")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_owned(b"Overwritee")));
    }

    #[test]
    fn test_new_from_attr_borrowed_valid() {
        assert_eq!(T::Value(23), T::new_from_attr(testdata::to_attr_borrowed(b"23")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_borrowed(b"1")));
        assert_eq!(T::Value(145), T::new_from_attr(testdata::to_attr_borrowed(b"145")));
        assert_eq!(T::Overwrite, T::new_from_attr(testdata::to_attr_borrowed(b"Overwrite")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_borrowed(b"Something else")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_borrowed(b"23a")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_borrowed(b"")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_borrowed(b"a3")));
        assert_eq!(T::Value(1), T::new_from_attr(testdata::to_attr_borrowed(b"Overwritee")));
    }

    #[test]
    fn test_default() {
        assert_eq!(T::Value(1), Default::default());
    }
}