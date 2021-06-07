//! Contains LogicalChannel and it's children
use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::events::attributes::Attribute;
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::ChannelFunction;
use crate::utils::deparse;
use crate::utils::deparse::DeparseSingle;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;
use crate::utils::units::node::Node;

pub mod channel_function;

///The Fixture Type Attribute is assinged to a LogicalChannel and defines the function of the LogicalChannel. All logical channels that are children of the same DMX channel are mutually exclusive. In a DMX mode, only one logical channel with the same attribute can reference the same geometry at a time.
#[derive(Debug, PartialEq, Clone)]
pub struct LogicalChannel {
    ///Link to the attribute; The starting point is the Attribute Collect
    pub attribute: Option<Node>,
    ///If snap is enabled, the logical channel will not fade between values. Instead, it will jump directly to the new value.; Value: “Yes”, “No”, “On”, “Off”. Default value: “No”
    pub snap: Snap,
    ///Defines if all the subordinate channel functions react to a Group Control defined by the control system. Values: “None”, “Grand”, “Group”; Default value: “None”.
    pub master: Master,
    ///Minimum fade time for moves in black action. MibFade is defined for the complete DMX range. Default value: 0; Unit: second
    pub mib_fade: f32,
    ///Minimum fade time for the subordinate channel functions to change DMX values by the control system. DMXChangeTimeLimit is defined for the complete DMX range. Default value: 0; Unit: second
    pub dmx_change_time_limit: f32,
    ///A list of channel functions
    pub channel_functions: HashMap<Name, ChannelFunction>,
}

impl DeparseSingle for LogicalChannel {
    type PrimaryKey = ();

    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut attribute = None;
        let mut snap: Snap = Snap::default();
        let mut master: Master = Master::default();
        let mut mib_fade: f32 = 0.;
        let mut dmx_change_time_limit: f32 = 0.;
        let mut channel_functions: HashMap<Name, ChannelFunction> = HashMap::new();

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Attribute" => attribute = Node::new_from_attr(attr)?,
                b"Snap" => snap = Snap::new_from_attr(attr),
                b"Master" => master = Master::new_from_attr(attr),
                b"MibFade" => mib_fade = deparse::attr_to_f32(&attr),
                b"DMXChangeTimeLimit" => dmx_change_time_limit = deparse::attr_to_f32(&attr),
                _ => {}
            }
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == b"ChannelFunction" {
                        let cf = ChannelFunction::single_from_event(reader, e)?;

                        channel_functions.insert(cf.1.unwrap(), cf.0);
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

        Ok((
            LogicalChannel {
                attribute,
                snap,
                master,
                mib_fade,
                dmx_change_time_limit,
                channel_functions,
            }
            , None))
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"LogicalChannel"
    }

    fn single_event_name() -> String {
        "LogicalChannel".to_string()
    }
}

#[cfg(test)]
impl TestDeparseSingle for LogicalChannel {}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of Snap
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

///Snap representation for Snap for LogicalChannel used in GDTF
/// If snap is enabled, the logical channel will not fade between values. Instead, it will jump directly to the new value
#[derive(Debug, PartialEq, Clone)]
pub enum Snap {
    No,
    Yes,
    On,
    Off,
}

///```rust
/// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::logical_channel::Snap;
///
/// assert_eq!(Snap::No, Default::default());
/// ```
impl Default for Snap {
    fn default() -> Self {
        Snap::No
    }
}

impl Snap {
    ///Creates a new snap from a string defined in gdtf-xml
    ///## Examples
    /// ```rust
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::logical_channel::Snap;
    ///
    /// assert_eq!(Snap::No,Snap::new_from_str("No"));
    /// assert_eq!(Snap::Yes,Snap::new_from_str("Yes"));
    /// assert_eq!(Snap::On,Snap::new_from_str("On"));
    /// assert_eq!(Snap::Off,Snap::new_from_str("Off"));
    /// assert_eq!(Snap::No,Snap::new_from_str("Anything else"));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        use Snap::*;
        match s {
            "No" => No,
            "Yes" => Yes,
            "On" => On,
            "Off" => Off,
            _ => Default::default()
        }
    }
    ///Creates a new snap from an xml attribute deparsed by quick-xml
    /// ## Examples
    /// ```rust
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::logical_channel::Snap;
    ///
    /// assert_eq!(Snap::No,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"No") }));
    /// assert_eq!(Snap::Yes,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Yes") }));
    /// assert_eq!(Snap::On,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"On") }));
    /// assert_eq!(Snap::Off,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Off") }));
    /// assert_eq!(Snap::No,Snap::new_from_attr(Attribute { key: &[], value: Cow::Borrowed(b"Anything else") }));
    /// ```

    pub fn new_from_attr(attr: Attribute) -> Self {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of Snap
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of Master
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

///Master representation for logicalChannel in GDTF
///Defines if all the subordinate channel functions react to a Group Control defined by the control system
#[derive(Debug, PartialEq, Clone)]
pub enum Master {
    None,
    Grand,
    Group,
}

///```rust
/// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::logical_channel::Master;
///
/// assert_eq!(Master::None, Default::default());
///```
impl Default for Master {
    fn default() -> Self {
        Master::None
    }
}

impl Master {
    ///Parses a string defined in gdtf-xml-description to a Master
    /// ```rust
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::logical_channel::Master;
    ///
    /// assert_eq!(Master::None, Master::new_from_str("None"));
    /// assert_eq!(Master::Grand, Master::new_from_str("Grand"));
    /// assert_eq!(Master::Group, Master::new_from_str("Group"));
    /// assert_eq!(Master::None, Master::new_from_str("Anything strange like ȸ"));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        use Master::*;
        match s {
            "Grand" => Grand,
            "Group" => Group,
            _ => None
        }
    }

    ///Parses a quick-xml-attribute defined in gdtf-xml-description to a Master
    /// ```rust
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::logical_channel::Master;
    ///
    /// assert_eq!(Master::None, Master::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"None")}));
    /// assert_eq!(Master::Grand, Master::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Grand")}));
    /// assert_eq!(Master::Group, Master::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Group")}));
    /// assert_eq!(Master::None, Master::new_from_attr(Attribute{key: &[], value: Cow::Borrowed(b"Anything strange like {")}));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of Master
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::{LogicalChannel, Master, Snap};
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::{ChannelFunction, Attribute};
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::dmx_value::DmxValue;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;

    #[test]
    fn test_normal() -> Result<(), GdtfError> {
        LogicalChannel {
            attribute: Node::new_from_str("ColorSub_M").unwrap(),
            snap: Snap::Yes,
            master: Master::Grand,
            mib_fade: 0.1,
            dmx_change_time_limit: 0.0,
            channel_functions: testdata::vec_to_hash_map(
                vec![
                    Name::new("Magenta")?,
                    Name::new("NoFeature")?],
                vec![
                    ChannelFunction {
                        attribute: Attribute::new_from_str("ColorSub_M")?,
                        original_attribute: "".to_string(),
                        dmx_from: DmxValue::new_from_str("0/1")?,
                        default: DmxValue::new_from_str("0/1")?,
                        physical_from: 0.0,
                        physical_to: 1.0,
                        real_fade: 0.0,
                        real_acceleration: 0.0,
                        wheel: None,
                        emitter: None,
                        filter: Node::new_from_str("Magenta")?,
                        mode_master: Node::new_from_str("Base_ColorMacro1")?,
                        mode_from: Some(DmxValue::new_from_str("0/1")?),
                        mode_to: Some(DmxValue::new_from_str("0/1")?),
                        channel_sets: HashMap::new(),
                    },
                    ChannelFunction {
                        attribute: Attribute::NoFeature,
                        original_attribute: "".to_string(),
                        dmx_from: DmxValue::new_from_str("0/1")?,
                        default: DmxValue::new_from_str("0/1")?,
                        physical_from: 0.0,
                        physical_to: 1.0,
                        real_fade: 0.0,
                        real_acceleration: 0.0,
                        wheel: None,
                        emitter: None,
                        filter: None,
                        mode_master: Node::new_from_str("Base_ColorMacro1")?,
                        mode_from: Some(DmxValue::new_from_str("1/1")?),
                        mode_to: Some(DmxValue::new_from_str("255/1")?),
                        channel_sets: HashMap::new(),
                    }
                ]),
        }.test(None,
               r#"
            <LogicalChannel Attribute="ColorSub_M" DMXChangeTimeLimit="0.000000" Master="Grand" MibFade="0.100000" Snap="Yes">
              <ChannelFunction Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Filter="Magenta" ModeFrom="0/1" ModeMaster="Base_ColorMacro1" ModeTo="0/1" Name="Magenta" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000">
              </ChannelFunction>
              <ChannelFunction Attribute="NoFeature" DMXFrom="0/1" Default="0/1" ModeFrom="1/1" ModeMaster="Base_ColorMacro1" ModeTo="255/1" Name="NoFeature" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000"/>
            </LogicalChannel>
            "#);
        Ok(())
    }

    #[test]
    fn test_min() -> Result<(), GdtfError> {
        LogicalChannel {
            attribute: None,
            snap: Snap::No,
            master: Master::None,
            mib_fade: 0.0,
            dmx_change_time_limit: 0.0,
            channel_functions: testdata::vec_to_hash_map(
                vec![
                    Name::new("Magenta")?,
                    Name::new("NoFeature")?
                ],
                vec![
                    ChannelFunction {
                        attribute: Attribute::new_from_str("ColorSub_M")?,
                        original_attribute: "".to_string(),
                        dmx_from: DmxValue::new_from_str("0/1")?,
                        default: DmxValue::new_from_str("0/1")?,
                        physical_from: 0.0,
                        physical_to: 1.0,
                        real_fade: 0.0,
                        real_acceleration: 0.0,
                        wheel: None,
                        emitter: None,
                        filter: Node::new_from_str("Magenta")?,
                        mode_master: Node::new_from_str("Base_ColorMacro1")?,
                        mode_from: Some(DmxValue::new_from_str("0/1")?),
                        mode_to: Some(DmxValue::new_from_str("0/1")?),
                        channel_sets: HashMap::new(),
                    },
                    ChannelFunction {
                        attribute: Attribute::new_from_str("NoFeature")?,
                        original_attribute: "".to_string(),
                        dmx_from: DmxValue::new_from_str("0/1")?,
                        default: DmxValue::new_from_str("0/1")?,
                        physical_from: 0.0,
                        physical_to: 1.0,
                        real_fade: 0.0,
                        real_acceleration: 0.0,
                        wheel: None,
                        emitter: None,
                        filter: None,
                        mode_master: Node::new_from_str("Base_ColorMacro1")?,
                        mode_from: Some(DmxValue::new_from_str("1/1")?),
                        mode_to: Some(DmxValue::new_from_str("255/1")?),
                        channel_sets: HashMap::new(),
                    }
                ]),
        }.test(None,
               r#"
            <LogicalChannel Attribute="" DMXChangeTimeLimit="" Master="" MibFade="" Snap="">
              <ChannelFunction Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Filter="Magenta" ModeFrom="0/1" ModeMaster="Base_ColorMacro1" ModeTo="0/1" Name="Magenta" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000">
              </ChannelFunction>
              <ChannelFunction Attribute="NoFeature" DMXFrom="0/1" Default="0/1" ModeFrom="1/1" ModeMaster="Base_ColorMacro1" ModeTo="255/1" Name="NoFeature" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000"/>
            </LogicalChannel>
            "#);
        Ok(())
    }

    #[test]
    fn test_snap_new_from_str() {
        assert_eq!(Snap::No, Snap::new_from_str("No"));
        assert_eq!(Snap::Yes, Snap::new_from_str("Yes"));
        assert_eq!(Snap::On, Snap::new_from_str("On"));
        assert_eq!(Snap::Off, Snap::new_from_str("Off"));
        assert_eq!(Snap::No, Snap::new_from_str("Anything else"));
    }

    #[test]
    fn test_snap_new_from_attr_owned() {
        assert_eq!(Snap::No, Snap::new_from_attr(testdata::to_attr_owned(b"No")));
        assert_eq!(Snap::Yes, Snap::new_from_attr(testdata::to_attr_owned(b"Yes")));
        assert_eq!(Snap::On, Snap::new_from_attr(testdata::to_attr_owned(b"On")));
        assert_eq!(Snap::Off, Snap::new_from_attr(testdata::to_attr_owned(b"Off")));
        assert_eq!(Snap::No, Snap::new_from_attr(testdata::to_attr_owned(b"Anything else")));
    }

    #[test]
    fn test_snap_new_from_attr_borrowed() {
        assert_eq!(Snap::No, Snap::new_from_attr(testdata::to_attr_borrowed(b"No")));
        assert_eq!(Snap::Yes, Snap::new_from_attr(testdata::to_attr_borrowed(b"Yes")));
        assert_eq!(Snap::On, Snap::new_from_attr(testdata::to_attr_borrowed(b"On")));
        assert_eq!(Snap::Off, Snap::new_from_attr(testdata::to_attr_borrowed(b"Off")));
        assert_eq!(Snap::No, Snap::new_from_attr(testdata::to_attr_borrowed(b"Anything else")));
    }


    #[test]
    fn test_snap_default() {
        assert_eq!(Snap::No, Default::default());
    }


    #[test]
    fn test_master_default() {
        assert_eq!(Master::None, Default::default())
    }

    #[test]
    fn test_master_new_from_str() {
        assert_eq!(Master::None, Master::new_from_str("None"));
        assert_eq!(Master::Grand, Master::new_from_str("Grand"));
        assert_eq!(Master::Group, Master::new_from_str("Group"));
        assert_eq!(Master::None, Master::new_from_str("Anything strange like ȸ"));
    }

    #[test]
    fn test_master_new_from_attr_owned() {
        assert_eq!(Master::None, Master::new_from_attr(testdata::to_attr_owned(b"None")));
        assert_eq!(Master::Grand, Master::new_from_attr(testdata::to_attr_owned(b"Grand")));
        assert_eq!(Master::Group, Master::new_from_attr(testdata::to_attr_owned(b"Group")));
        assert_eq!(Master::None, Master::new_from_attr(testdata::to_attr_owned(b"Anything strange like {")));
    }

    #[test]
    fn test_master_new_from_attr_borrowed() {
        assert_eq!(Master::None, Master::new_from_attr(testdata::to_attr_borrowed(b"None")));
        assert_eq!(Master::Grand, Master::new_from_attr(testdata::to_attr_borrowed(b"Grand")));
        assert_eq!(Master::Group, Master::new_from_attr(testdata::to_attr_borrowed(b"Group")));
        assert_eq!(Master::None, Master::new_from_attr(testdata::to_attr_borrowed(b"Anything strange like {")));
    }
}