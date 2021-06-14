//!Holds the DMXChannel and it's children
use std::fmt::Debug;
use std::str::FromStr;

use quick_xml::Reader;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::LogicalChannel;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::{ReadGdtf, ReadGdtfDataHolder};
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
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

///Helper struct for storing temporary data during deparsing
#[derive(Default)]
pub(crate) struct DmxChannelDataHolder {
    ///Number of the DMXBreak; Default value: 1; Special value: “Overwrite” – means that this number will be overwritten by Geometry Reference; Size: 4 bytes
    pub dmx_break: Option<DmxBreak>,
    ///Relative addresses of the current DMX channel from highest to least significant
    pub offset: Option<Offset>,
    ///Link to the channel function that will be activated by default for this DMXChannel;
    pub initial_function: Option<Node>,
    ///Highlight value for current channel; Special value: “None”. Default value: “None”.
    pub highlight: Option<DmxValue>,
    ///Name of the geometry the current channel controls.
    pub geometry: Option<Name>,
    ///List of logical channels
    pub logical_channels: Vec<LogicalChannel>,
}

impl ReadGdtf<DmxChannelDataHolder> for DmxChannel {
    type PrimaryKey = ();
    type Error = GdtfError;

    const NODE_NAME: &'static [u8] = b"DMXChannel";

    const PARENT_NODE_NAME: &'static [u8] = b"DMXChannels";
    const PRIMARY_KEY_NAME: &'static [u8] = b"";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_primary_key_from_attr(_: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed");
    }
}

impl ReadGdtfDataHolder<DmxChannel> for DmxChannelDataHolder {
    fn read_any_attribute(&mut self, attr: Attribute<'_>) -> Result<(), <DmxChannel as ReadGdtf<Self>>::Error> {
        match attr.key {
            b"DMXBreak" => self.dmx_break = Some(DmxBreak::new_from_attr(attr)),
            b"Offset" => self.offset = Offset::new_from_attr(attr),
            b"InitialFunction" => self.initial_function = Node::new_from_attr(attr)?,
            b"Highlight" => self.highlight = match DmxValue::new_from_attr(attr) {
                Ok(attr) => Some(attr),
                Err(_) => None
            },
            b"Geometry" => self.geometry = Some(Name::new_from_attr(attr)?),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(&mut self, reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(), <DmxChannel as ReadGdtf<Self>>::Error> {
        if event.name() == b"LogicalChannel" {
            self.logical_channels.push(LogicalChannel::read_single_from_event(reader, event, has_children)?.1);
        }
        Ok(())
    }

    fn move_data(self) -> Result<DmxChannel, <DmxChannel as ReadGdtf<Self>>::Error> {
        Ok(DmxChannel {
            dmx_break: self.dmx_break.unwrap_or(DmxBreak::Value(1)),
            offset: self.offset,
            initial_function: self.initial_function,
            highlight: self.highlight,
            geometry: self.geometry.ok_or_else(|| Self::attribute_not_found(b"Geometry"))?,
            logical_channels: self.logical_channels,
        })
    }
}

#[cfg(test)]
impl TestReadGdtf<DmxChannelDataHolder> for DmxChannel {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(Self { dmx_break: DmxBreak::Value(1), offset: Some(Offset(vec![1, 2])), initial_function: Node::new_from_str("Yoke_Pan.Pan.Pan 1").unwrap(), highlight: Some(DmxValue { initial_value: 16, n: 1, is_byte_shifting: false }), geometry: Name::new("Yoke").unwrap(), logical_channels: vec![] })),
            (None, Some(Self { dmx_break: DmxBreak::Value(1), offset: Some(Offset(vec![1, 2])), initial_function: Node::new_from_str("Yoke_Pan.Pan.Pan 1").unwrap(), highlight: Some(DmxValue { initial_value: 16, n: 1, is_byte_shifting: false }), geometry: Name::new("Yoke").unwrap(), logical_channels: vec![] })),
            (None, Some(Self { dmx_break: DmxBreak::Value(1), offset: Some(Offset(vec![1, 2])), initial_function: Node::new_from_str("Yoke_Pan.Pan.Pan 1").unwrap(), highlight: Some(DmxValue { initial_value: 12, n: 2, is_byte_shifting: true }), geometry: Name::new("Yoke").unwrap(), logical_channels: LogicalChannel::testdata_vec() })),
            (None, Some(Self { dmx_break: DmxBreak::Value(2), offset: None, initial_function: Node::new_from_str("Yoke_Pan.Pan.Pan 1").unwrap(), highlight: None, geometry: Name::new("Head").unwrap(), logical_channels: LogicalChannel::testdata_vec() })),
            (None, Some(Self { dmx_break: DmxBreak::Value(1), offset: None, initial_function: Node::new_from_str("Yoke_Pan.Pan.Pan 1").unwrap(), highlight: None, geometry: Name::new("Yoke").unwrap(), logical_channels: LogicalChannel::testdata_vec() })),
            (None, Some(Self { dmx_break: DmxBreak::Value(55), offset: Some(Offset(vec![2])), initial_function: Node::new_from_str("Yoke_Pan.Pan.Pan 1").unwrap(), highlight: Some(DmxValue { initial_value: 16, n: 1, is_byte_shifting: false }), geometry: Name::new("Yoke").unwrap(), logical_channels: LogicalChannel::testdata_vec() })),
            (None, Some(Self { dmx_break: DmxBreak::Value(1), offset: Some(Offset(vec![1])), initial_function: Node::new_from_str("Yoke_Pan.Pan.Pan 1").unwrap(), highlight: Some(DmxValue { initial_value: 16, n: 1, is_byte_shifting: false }), geometry: Name::new("Yoke").unwrap(), logical_channels: LogicalChannel::testdata_vec() })),
            (None, Some(Self { dmx_break: DmxBreak::Value(1), offset: Some(Offset(vec![1, 2])), initial_function: None, highlight: Some(DmxValue { initial_value: 16, n: 1, is_byte_shifting: false }), geometry: Name::new("Yoke").unwrap(), logical_channels: LogicalChannel::testdata_vec() })),
            (None, Some(Self { dmx_break: DmxBreak::Value(1), offset: Some(Offset(vec![1, 3])), initial_function: Node::new_from_str("Yoke_Pan.Pan.Pan 1").unwrap(), highlight: Some(DmxValue { initial_value: 16, n: 1, is_byte_shifting: false }), geometry: Name::new("Yoke").unwrap(), logical_channels: LogicalChannel::testdata_vec() })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<DMXChannel DMXBreak="1" Geometry="Yoke" Highlight="16/1" InitialFunction="Yoke_Pan.Pan.Pan 1" Offset="1,2"></DMXChannel>"#.to_string(),
            r#"<DMXChannel DMXBreak="1" Geometry="Yoke" Highlight="16/1" InitialFunction="Yoke_Pan.Pan.Pan 1" Offset="1,2"/>"#.to_string(),
            format!(r#"<DMXChannel DMXBreak="1" Geometry="Yoke" Highlight="12/2s" InitialFunction="Yoke_Pan.Pan.Pan 1" Offset="1,2">{}</DMXChannel>"#, LogicalChannel::testdata_xml()),
            format!(r#"<DMXChannel DMXBreak="2" Geometry="Head" Highlight="None" InitialFunction="Yoke_Pan.Pan.Pan 1" >{}</DMXChannel>"#, LogicalChannel::testdata_xml()),
            format!(r#"<DMXChannel DMXBreak="1" Geometry="Yoke" InitialFunction="Yoke_Pan.Pan.Pan 1" Offset="None">{}</DMXChannel>"#, LogicalChannel::testdata_xml()),
            format!(r#"<DMXChannel DMXBreak="55" Geometry="Yoke" Highlight="16/1" InitialFunction="Yoke_Pan.Pan.Pan 1" Offset="2">{}</DMXChannel>"#, LogicalChannel::testdata_xml()),
            format!(r#"<DMXChannel DMXBreak="1" Geometry="Yoke" Highlight="16/1" InitialFunction="Yoke_Pan.Pan.Pan 1" Offset="1">{}</DMXChannel>"#, LogicalChannel::testdata_xml()),
            format!(r#"<DMXChannel DMXBreak="1" Geometry="Yoke" Highlight="16/1" Offset="1,2">{}</DMXChannel>"#, LogicalChannel::testdata_xml()),
            format!(r#"<DMXChannel DMXBreak="1" Geometry="Yoke" Highlight="16/1" InitialFunction="Yoke_Pan.Pan.Pan 1" Offset="1,3">{}</DMXChannel>"#, LogicalChannel::testdata_xml()),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![
            r#"<DMXChannel DMXBreak="1" Highlight="16/1" InitialFunction="Yoke_Pan.Pan.Pan 1" Offset="1,2"/>"#.to_string(),
        ]
    }
}

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
        Self::new_from_str(read::attr_to_str(&attr))
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
        Self::new_from_str(read::attr_to_str(&attr))
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
    use crate::fixture_type::dmx_mode::dmx_channel::{DmxBreak, DmxChannel, Offset};
    use crate::utils::read::TestReadGdtf;
    use crate::utils::testdata;

    #[test]
    fn test_deparse() {
        DmxChannel::execute_tests();
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