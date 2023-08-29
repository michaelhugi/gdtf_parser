//! Contains LogicalChannel and it's children
use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::ChannelFunction;
use crate::fixture_type::dmx_mode::dmx_channel::DmxChannel;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::name::Name;
use crate::utils::units::node::Node;

pub mod channel_function;

///The Fixture Type Attribute is assinged to a LogicalChannel and defines the function of the LogicalChannel. All logical channels that are children of the same DMX channel are mutually exclusive. In a DMX mode, only one logical channel with the same attribute can reference the same geometry at a time.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LogicalChannel {
    ///Link to the attribute; The starting point is the Attribute Collect
    pub attribute: Node,
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

///Helper struct to temporary hold data during deparsing
#[derive(Default)]
pub(crate) struct LogicalChannelDataHolder {
    ///Link to the attribute; The starting point is the Attribute Collect
    pub attribute: Option<Node>,
    ///If snap is enabled, the logical channel will not fade between values. Instead, it will jump directly to the new value.; Value: “Yes”, “No”, “On”, “Off”. Default value: “No”
    pub snap: Option<Snap>,
    ///Defines if all the subordinate channel functions react to a Group Control defined by the control system. Values: “None”, “Grand”, “Group”; Default value: “None”.
    pub master: Option<Master>,
    ///Minimum fade time for moves in black action. MibFade is defined for the complete DMX range. Default value: 0; Unit: second
    pub mib_fade: Option<f32>,
    ///Minimum fade time for the subordinate channel functions to change DMX values by the control system. DMXChangeTimeLimit is defined for the complete DMX range. Default value: 0; Unit: second
    pub dmx_change_time_limit: Option<f32>,
    ///A list of channel functions
    pub channel_functions: HashMap<Name, ChannelFunction>,
}

impl ReadGdtf for LogicalChannel {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = LogicalChannelDataHolder;

    const NODE_NAME: &'static [u8] = b"LogicalChannel";
    const PARENT_NODE_NAME: &'static [u8] = DmxChannel::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = b"";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        match attr.key {
            b"Attribute" => data_holder.attribute = Node::new_from_attr(attr)?,
            b"Snap" => data_holder.snap = Some(Snap::new_from_attr(attr)),
            b"Master" => data_holder.master = Some(Master::new_from_attr(attr)),
            b"MibFade" => data_holder.mib_fade = Some(read::attr_to_f32(attr)),
            b"DMXChangeTimeLimit" => {
                data_holder.dmx_change_time_limit = Some(read::attr_to_f32(attr))
            }
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(
        data_holder: &mut Self::DataHolder,
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        _: bool,
    ) -> Result<(), Self::Error> {
        if event.name() == ChannelFunction::NODE_NAME {
            let cf = ChannelFunction::read_single_from_event(reader, event, true)?;
            data_holder.channel_functions.insert(
                cf.0.ok_or_else(|| {
                    Self::child_primary_key_not_found(
                        ChannelFunction::NODE_NAME,
                        ChannelFunction::PRIMARY_KEY_NAME,
                    )
                })?,
                cf.1,
            );
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            attribute: data_holder
                .attribute
                .ok_or_else(|| Self::attribute_not_found(b"Attribute"))?,
            snap: data_holder.snap.unwrap_or(Snap::No),
            master: data_holder.master.unwrap_or(Master::None),
            mib_fade: data_holder.mib_fade.unwrap_or(0_f32),
            dmx_change_time_limit: data_holder.dmx_change_time_limit.unwrap_or(0_f32),
            channel_functions: data_holder.channel_functions,
        })
    }
}

#[cfg(test)]
impl TestReadGdtf for LogicalChannel {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(Self {
                    attribute: Node::new_from_str("Pan").unwrap().unwrap(),
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 12.0,
                    channel_functions: HashMap::new(),
                }),
            ),
            (
                None,
                Some(Self {
                    attribute: Node::new_from_str("Pan").unwrap().unwrap(),
                    snap: Snap::Yes,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                }),
            ),
            (
                None,
                Some(Self {
                    attribute: Node::new_from_str("Pan").unwrap().unwrap(),
                    snap: Snap::On,
                    master: Master::Grand,
                    mib_fade: 18.032032,
                    dmx_change_time_limit: 12.0,
                    channel_functions: ChannelFunction::testdata_hash_map(),
                }),
            ),
            (
                None,
                Some(Self {
                    attribute: Node::new_from_str("Tilt").unwrap().unwrap(),
                    snap: Snap::Off,
                    master: Master::Group,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 12.0,
                    channel_functions: ChannelFunction::testdata_hash_map(),
                }),
            ),
            (
                None,
                Some(Self {
                    attribute: Node::new_from_str("Pan").unwrap().unwrap(),
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: ChannelFunction::testdata_hash_map(),
                }),
            ),
            (
                None,
                Some(Self {
                    attribute: Node::new_from_str("Pan").unwrap().unwrap(),
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 12.000001,
                    channel_functions: ChannelFunction::testdata_hash_map(),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<LogicalChannel Attribute="Pan" DMXChangeTimeLimit="12.000000" Master="None" MibFade="0.000000" Snap="No"/>"#.to_string(),
            r#"<LogicalChannel Attribute="Pan" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="Yes"></LogicalChannel>"#.to_string(),
            format!(r#"<LogicalChannel Attribute="Pan" DMXChangeTimeLimit="12.000000" Master="Grand" MibFade="18.032032" Snap="On">{}</LogicalChannel>"#, ChannelFunction::testdata_xml()),
            format!(r#"<LogicalChannel Attribute="Tilt" DMXChangeTimeLimit="12.000000" Master="Group"  Snap="Off">{}</LogicalChannel>"#, ChannelFunction::testdata_xml()),
            format!(r#"<LogicalChannel Attribute="Pan" MibFade="0.000000">{}</LogicalChannel>"#, ChannelFunction::testdata_xml()),
            format!(r#"<LogicalChannel Attribute="Pan" DMXChangeTimeLimit="12.000001" Master="None" MibFade="0.000000" Snap="No">{}</LogicalChannel>"#, ChannelFunction::testdata_xml()),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![
            r#"<LogicalChannel DMXChangeTimeLimit="12.000000" Master="None" MibFade="0.000000" Snap="No"/>"#.to_string(),
            r#"<LogicalChannel Attribute="Pan with invalid char {" DMXChangeTimeLimit="12.000000" Master="None" MibFade="0.000000" Snap="No"/>"#.to_string(),
        ]
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of Snap
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

///Snap representation for Snap for LogicalChannel used in GDTF
/// If snap is enabled, the logical channel will not fade between values. Instead, it will jump directly to the new value
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
            _ => Default::default(),
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
        Self::new_from_str(read::attr_to_str(&attr))
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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
            _ => None,
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
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of Master
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::{
        LogicalChannel, Master, Snap,
    };
    use crate::utils::read::TestReadGdtf;
    use crate::utils::testdata;

    #[test]
    fn test_deparse() {
        LogicalChannel::execute_tests();
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
        assert_eq!(
            Snap::No,
            Snap::new_from_attr(testdata::to_attr_owned(b"No"))
        );
        assert_eq!(
            Snap::Yes,
            Snap::new_from_attr(testdata::to_attr_owned(b"Yes"))
        );
        assert_eq!(
            Snap::On,
            Snap::new_from_attr(testdata::to_attr_owned(b"On"))
        );
        assert_eq!(
            Snap::Off,
            Snap::new_from_attr(testdata::to_attr_owned(b"Off"))
        );
        assert_eq!(
            Snap::No,
            Snap::new_from_attr(testdata::to_attr_owned(b"Anything else"))
        );
    }

    #[test]
    fn test_snap_new_from_attr_borrowed() {
        assert_eq!(
            Snap::No,
            Snap::new_from_attr(testdata::to_attr_borrowed(b"No"))
        );
        assert_eq!(
            Snap::Yes,
            Snap::new_from_attr(testdata::to_attr_borrowed(b"Yes"))
        );
        assert_eq!(
            Snap::On,
            Snap::new_from_attr(testdata::to_attr_borrowed(b"On"))
        );
        assert_eq!(
            Snap::Off,
            Snap::new_from_attr(testdata::to_attr_borrowed(b"Off"))
        );
        assert_eq!(
            Snap::No,
            Snap::new_from_attr(testdata::to_attr_borrowed(b"Anything else"))
        );
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
        assert_eq!(
            Master::None,
            Master::new_from_str("Anything strange like ȸ")
        );
    }

    #[test]
    fn test_master_new_from_attr_owned() {
        assert_eq!(
            Master::None,
            Master::new_from_attr(testdata::to_attr_owned(b"None"))
        );
        assert_eq!(
            Master::Grand,
            Master::new_from_attr(testdata::to_attr_owned(b"Grand"))
        );
        assert_eq!(
            Master::Group,
            Master::new_from_attr(testdata::to_attr_owned(b"Group"))
        );
        assert_eq!(
            Master::None,
            Master::new_from_attr(testdata::to_attr_owned(b"Anything strange like {"))
        );
    }

    #[test]
    fn test_master_new_from_attr_borrowed() {
        assert_eq!(
            Master::None,
            Master::new_from_attr(testdata::to_attr_borrowed(b"None"))
        );
        assert_eq!(
            Master::Grand,
            Master::new_from_attr(testdata::to_attr_borrowed(b"Grand"))
        );
        assert_eq!(
            Master::Group,
            Master::new_from_attr(testdata::to_attr_borrowed(b"Group"))
        );
        assert_eq!(
            Master::None,
            Master::new_from_attr(testdata::to_attr_borrowed(b"Anything strange like {"))
        );
    }
}
