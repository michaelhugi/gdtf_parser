//!Holds the DMXChannel and it's children
use std::fmt::Debug;
use std::str::FromStr;

use quick_xml::events::{BytesStart, Event};
use quick_xml::events::attributes::Attribute;
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::LogicalChannel;
use crate::utils::deparse::{DeparseSingle, DeparseVec};
use crate::utils::deparse;
#[cfg(test)]
use crate::utils::deparse::{TestDeparseSingle, TestDeparseVec};
use crate::utils::errors::GdtfError;
use crate::utils::units::dmx_value::DmxValue;
use crate::utils::units::name::Name;
use crate::utils::units::node::Node;

pub mod logical_channel;

///This section defines the DMX channe
#[derive(Debug, PartialEq, Clone)]
pub struct DmxChannel {
    ///Number of the DMXBreak; Default value: 1; Special value: “Overwrite” – means that this number will be overwritten by Geometry Reference; Size: 4 bytes
    pub dmx_break: DmxBreak,
    ///Relative addresses of the current DMX channel from highest to least significant
    pub offset: Option<Offset>,
    ///Link to the channel function that will be activated by default for this DMXChannel;
    pub initial_function: Option<Node>,
    ///Highlight value for current channel; Special value: “None”. Default value: “None”.
    pub highlight: Option<DmxValue>,
    ///Name of the geometry the current channel controls.
    pub geometry: Name,
    ///List of logical channels
    pub logical_channels: Vec<LogicalChannel>,
}

impl DeparseSingle for DmxChannel {
    type PrimaryKey = ();
    type Error = GdtfError;

    const NODE_NAME: &'static [u8] = b"DMXChannel";

    fn read_single_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut dmx_break = DmxBreak::default();
        let mut offset = None;
        let mut initial_function = None;
        let mut highlight = None;
        let mut geometry = Default::default();
        let mut logical_channels: Vec<LogicalChannel> = Vec::new();

        for attr in event.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"DMXBreak" => dmx_break = DmxBreak::new_from_attr(attr),
                b"Offset" => offset = Offset::new_from_attr(attr),
                b"InitialFunction" => initial_function = Node::new_from_attr(attr)?,
                b"Highlight" => highlight = match DmxValue::new_from_attr(attr) {
                    Ok(attr) => Some(attr),
                    Err(_) => None
                },
                b"Geometry" => geometry = Name::new_from_attr(attr)?,
                _ => {}
            }
        }


        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == b"LogicalChannel" {
                        logical_channels.push(LogicalChannel::read_single_from_event(reader, e)?.0);
                    } else {
                        tree_down += 1;
                    }
                }
                Event::Eof => {
                    break;
                }

                Event::End(_) => {
                    tree_down -= 1;
                    if tree_down <= 0 {
                        break;
                    }
                }
                _ => {}
            }
        }
        buf.clear();

        Ok((Self {
            dmx_break,
            offset,
            initial_function,
            highlight,
            geometry,
            logical_channels,
        }, None))
    }
}

impl DeparseVec for DmxChannel {
    const PARENT_NODE_NAME: &'static [u8] = b"DMXChannels";
}

#[cfg(test)]
impl TestDeparseVec for DmxChannel {

}

#[cfg(test)]
impl TestDeparseSingle for DmxChannel {}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of Offset
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------


///The unit Offset used for DMXChannel used in GDTF
///Relative addresses of the current DMX channel from highest to least significant
#[derive(Debug, PartialEq, Clone)]
pub struct Offset(pub Vec<i32>);

impl Offset {
    ///Creates a new Option<Offset> from a string defined in gdtf-xml
    /// ## Examples
    /// ```rust
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::Offset;
    ///
    /// assert!(Offset::new_from_str("None").is_none());
    /// assert_eq!(Offset(vec![1]), Offset::new_from_str("1").unwrap());
    /// assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_str("0,1,2,-3").unwrap());
    ///
    /// //Handling of wrong values
    ///
    /// //More than i32::MAX
    /// assert!(Offset::new_from_str("2147483648").is_none());
    /// //Less than i32::MIN
    /// assert!(Offset::new_from_str("-2147483649").is_none());
    /// assert!(Offset::new_from_str("").is_none());
    /// assert!(Offset::new_from_str("Something else").is_none());
    /// ```
    pub fn new_from_str(s: &str) -> Option<Self> {
        if s == "None" {
            return None;
        }
        let mut v = Vec::new();
        for s in s.split(',').into_iter() {
            match i32::from_str(s) {
                Ok(s) => v.push(s),
                Err(_) => return None
            }
        }
        Some(Offset(v))
    }

    ///Creates a new Option<Offset> from a quick-xml-attribute defined in gdtf-xml
    /// ## Examples
    /// ```rust
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::Offset;
    ///
    /// assert!(Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"None")}).is_none());
    /// assert_eq!(Offset(vec![1]), Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"1")}).unwrap());
    /// assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"0,1,2,-3")}).unwrap());
    ///
    /// //Handling wrong values
    ///
    /// //More than i32::MAX
    /// assert!(Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"2147483648")}).is_none());
    /// //Less than i32::MIN
    /// assert!(Offset::new_from_str("-2147483649").is_none());
    /// assert!(Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"")}).is_none());
    /// assert!(Offset::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Something else")}).is_none());
    /// ```
    pub fn new_from_attr(attr: Attribute) -> Option<Self> {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
    /// Creates a new Offset from a Vec<i32>
    /// ## Examples
    /// ```rust
    ///  use gdtf_parser::fixture_type::dmx_mode::dmx_channel::Offset;
    ///
    ///  assert_eq!(Offset(vec![]), Offset::new(vec![]));
    ///  assert_eq!(Offset(vec![1]), Offset::new(vec![1]));
    ///  assert_eq!(Offset(vec![1, 3]), Offset::new(vec![1, 3]));
    /// ```
    pub fn new(offsets: Vec<i32>) -> Self {
        Self(offsets)
    }
}


//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of Offset
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------


//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of DmxBreak
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------


///DMXBreak used for DMXChannel in GDTF
#[derive(Debug, PartialEq, Clone)]
pub enum DmxBreak {
    ///Number of the DMXBreak; Default value: 1
    Value(u32),
    ///means that this number will be overwritten by Geometry Reference
    Overwrite,
}

impl DmxBreak {
    ///Parses a string used in gdtf-xml-description to a DmxBreak
    /// ```rust
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::DmxBreak;
    ///
    /// assert_eq!(DmxBreak::new_from_str("32"), DmxBreak::Value(32));
    /// assert_eq!(DmxBreak::new_from_str("Overwrite"), DmxBreak::Overwrite);
    /// assert_eq!(DmxBreak::new_from_str("Anything else"), DmxBreak::Value(1));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        use DmxBreak::*;
        if s == "Overwrite" {
            Overwrite
        } else {
            Value(u32::from_str(s).unwrap_or(1))
        }
    }
    ///Parses a quick-xml-attribute from gdtf-xml-description to a DmxBreak
    /// ```rust
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::DmxBreak;
    ///
    /// assert_eq!(DmxBreak::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"32")}), DmxBreak::Value(32));
    /// assert_eq!(DmxBreak::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Overwrite")}), DmxBreak::Overwrite);
    /// assert_eq!(DmxBreak::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Anything else")}), DmxBreak::Value(1));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}

///```rust
/// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::DmxBreak;
///
/// assert_eq!(DmxBreak::Value(1), Default::default());
/// ```
impl Default for DmxBreak {
    fn default() -> Self {
        Self::Value(1)
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of DmxBreak
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::fixture_type::dmx_mode::dmx_channel::{DmxBreak, DmxChannel, Offset};
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::{LogicalChannel, Master, Snap};
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::dmx_value::DmxValue;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;

    #[test]
    fn test_normal() -> Result<(), GdtfError> {
        DmxChannel {
            dmx_break: DmxBreak::Value(1),
            offset: Some(Offset::new(vec![1])),
            initial_function: Node::new_from_str("Beam_Shutter1.Shutter1.Open")?,
            highlight: Some(DmxValue {
                initial_value: 8,
                n: 1,
                is_byte_shifting: false,
            }),
            geometry: Name::new("Beam")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: Node::new_from_str("Shutter1").unwrap(),
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                }
            ],
        }.compare_to_primary_key_and_xml(None,
                                         r#"
            <DMXChannel DMXBreak="1" Geometry="Beam" Highlight="8/1" InitialFunction="Beam_Shutter1.Shutter1.Open" Offset="1">
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No"></LogicalChannel>
            </DMXChannel>
            "#,
        );
        Ok(())
    }

    #[test]
    fn test_normal_2() -> Result<(), GdtfError> {
        DmxChannel {
            dmx_break: DmxBreak::Value(2),
            offset: Some(Offset::new(vec![1, 2])),
            initial_function: Node::new_from_str("Beam_Shutter1.Shutter1.Open")?,
            highlight: Some(DmxValue {
                initial_value: 8,
                n: 1,
                is_byte_shifting: false,
            }),
            geometry: Name::new("Beam")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: Node::new_from_str("Shutter1")?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                }
            ],
        }.compare_to_primary_key_and_xml(None,
                                         r#"
            <DMXChannel DMXBreak="2" Geometry="Beam" Highlight="8/1" InitialFunction="Beam_Shutter1.Shutter1.Open" Offset="1,2">
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No"></LogicalChannel>
            </DMXChannel>
            "#,
        );
        Ok(())
    }

    #[test]
    fn test_normal_3() -> Result<(), GdtfError> {
        DmxChannel {
            dmx_break: DmxBreak::Overwrite,
            offset: Some(Offset::new(vec![1, 2])),
            initial_function: Node::new_from_str("Beam_Shutter1.Shutter1.Open")?,
            highlight: Some(DmxValue {
                initial_value: 8,
                n: 1,
                is_byte_shifting: false,
            }),
            geometry: Name::new("Beam")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: Node::new_from_str("Shutter1")?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                }
            ],
        }.compare_to_primary_key_and_xml(None,
                                         r#"
            <DMXChannel DMXBreak="Overwrite" Geometry="Beam" Highlight="8/1" InitialFunction="Beam_Shutter1.Shutter1.Open" Offset="1,2">
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No"></LogicalChannel>
            </DMXChannel>
            "#,
        );
        Ok(())
    }

    #[test]
    fn test_min() -> Result<(), GdtfError> {
        DmxChannel {
            dmx_break: DmxBreak::Value(1),
            offset: None,
            initial_function: None,
            highlight: None,
            geometry: Name::new("")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: Node::new_from_str("Shutter1")?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                },
                LogicalChannel {
                    attribute: Node::new_from_str("Shutter1")?,
                    snap: Snap::Yes,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                }
            ],
        }.compare_to_primary_key_and_xml(None,
                                         r#"
            <DMXChannel DMXBreak="" Geometry="" Highlight="" InitialFunction="" Offset="">
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No"></LogicalChannel>
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="Yes"></LogicalChannel>
            </DMXChannel>
            "#,
        );
        Ok(())
    }

    #[test]
    fn test_faulty() -> Result<(), GdtfError> {
        DmxChannel {
            dmx_break: DmxBreak::Value(1),
            offset: None,
            initial_function: None,
            highlight: None,
            geometry: Name::new("")?,
            logical_channels: vec![],
        }.compare_to_primary_key_and_xml(None,
                                         r#"
            <DMXChannel>
            </DMXChannel>
            "#,
        );
        Ok(())
    }


    #[test]
    fn test_offset_new_from_str() {
        assert!(Offset::new_from_str("None").is_none());
        assert_eq!(Offset(vec![1]), Offset::new_from_str("1").unwrap());
        assert_eq!(Offset(vec![-1]), Offset::new_from_str("-1").unwrap());
        assert_eq!(Offset(vec![1, 2]), Offset::new_from_str("1,2").unwrap());
        assert_eq!(Offset(vec![1, -2]), Offset::new_from_str("1,-2").unwrap());
        assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_str("0,1,2,-3").unwrap());
        assert_eq!(Offset(vec![i32::MAX, i32::MIN]), Offset::new_from_str("2147483647,-2147483648").unwrap());

        assert!(Offset::new_from_str("").is_none());
        assert!(Offset::new_from_str("Something else").is_none());
        assert!(Offset::new_from_str("2147483648,-2147483648").is_none());
        assert!(Offset::new_from_str("2147483648,-2147483649").is_none());
    }

    #[test]
    fn test_offset_new_from_attr_owned() {
        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"None")).is_none());
        assert_eq!(Offset(vec![1]), Offset::new_from_attr(testdata::to_attr_owned(b"1")).unwrap());
        assert_eq!(Offset(vec![-1]), Offset::new_from_attr(testdata::to_attr_owned(b"-1")).unwrap());
        assert_eq!(Offset(vec![1, 2]), Offset::new_from_attr(testdata::to_attr_owned(b"1,2")).unwrap());
        assert_eq!(Offset(vec![1, -2]), Offset::new_from_attr(testdata::to_attr_owned(b"1,-2")).unwrap());
        assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_attr(testdata::to_attr_owned(b"0,1,2,-3")).unwrap());
        assert_eq!(Offset(vec![i32::MAX, i32::MIN]), Offset::new_from_attr(testdata::to_attr_owned(b"2147483647,-2147483648")).unwrap());

        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"Something else")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"2147483648,-2147483648")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_owned(b"2147483648,-2147483649")).is_none());
    }

    #[test]
    fn test_offset_new_from_attr_borrowed() {
        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"None")).is_none());
        assert_eq!(Offset(vec![1]), Offset::new_from_attr(testdata::to_attr_borrowed(b"1")).unwrap());
        assert_eq!(Offset(vec![-1]), Offset::new_from_attr(testdata::to_attr_borrowed(b"-1")).unwrap());
        assert_eq!(Offset(vec![1, 2]), Offset::new_from_attr(testdata::to_attr_borrowed(b"1,2")).unwrap());
        assert_eq!(Offset(vec![1, -2]), Offset::new_from_attr(testdata::to_attr_borrowed(b"1,-2")).unwrap());
        assert_eq!(Offset(vec![0, 1, 2, -3]), Offset::new_from_attr(testdata::to_attr_borrowed(b"0,1,2,-3")).unwrap());
        assert_eq!(Offset(vec![i32::MAX, i32::MIN]), Offset::new_from_attr(testdata::to_attr_borrowed(b"2147483647,-2147483648")).unwrap());

        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"Something else")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"2147483648,-2147483648")).is_none());
        assert!(Offset::new_from_attr(testdata::to_attr_borrowed(b"2147483648,-2147483649")).is_none());
    }

    #[test]
    fn test_offset_new() {
        assert_eq!(Offset(vec![]), Offset::new(vec![]));
        assert_eq!(Offset(vec![1]), Offset::new(vec![1]));
        assert_eq!(Offset(vec![1, 3]), Offset::new(vec![1, 3]));
    }


    #[test]
    fn test_dmx_break_new_from_str() {
        assert_eq!(DmxBreak::Value(23), DmxBreak::new_from_str("23"));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_str("1"));
        assert_eq!(DmxBreak::Value(145), DmxBreak::new_from_str("145"));
        assert_eq!(DmxBreak::Overwrite, DmxBreak::new_from_str("Overwrite"));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_str("Something else"));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_str("23a"));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_str(""));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_str("a3"));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_str("Overwritee"));
    }

    #[test]
    fn test_dmx_break_new_from_attr_owned_valid() {
        assert_eq!(DmxBreak::Value(23), DmxBreak::new_from_attr(testdata::to_attr_owned(b"23")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_owned(b"1")));
        assert_eq!(DmxBreak::Value(145), DmxBreak::new_from_attr(testdata::to_attr_owned(b"145")));
        assert_eq!(DmxBreak::Overwrite, DmxBreak::new_from_attr(testdata::to_attr_owned(b"Overwrite")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_owned(b"Something else")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_owned(b"23a")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_owned(b"")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_owned(b"a3")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_owned(b"Overwritee")));
    }

    #[test]
    fn test_dmx_break_new_from_attr_borrowed_valid() {
        assert_eq!(DmxBreak::Value(23), DmxBreak::new_from_attr(testdata::to_attr_borrowed(b"23")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_borrowed(b"1")));
        assert_eq!(DmxBreak::Value(145), DmxBreak::new_from_attr(testdata::to_attr_borrowed(b"145")));
        assert_eq!(DmxBreak::Overwrite, DmxBreak::new_from_attr(testdata::to_attr_borrowed(b"Overwrite")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_borrowed(b"Something else")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_borrowed(b"23a")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_borrowed(b"")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_borrowed(b"a3")));
        assert_eq!(DmxBreak::Value(1), DmxBreak::new_from_attr(testdata::to_attr_borrowed(b"Overwritee")));
    }

    #[test]
    fn test_dmx_break_default() {
        assert_eq!(DmxBreak::Value(1), Default::default());
    }
}