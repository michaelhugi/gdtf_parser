//!Holds the DMXChannel and it's children
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::LogicalChannel;
use crate::utils::deparse;
use crate::utils::deparse::{DeparseSingle, DeparseVec};
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::dmx_break::DmxBreak;
use crate::utils::units::highlight::Highlight;
use crate::utils::units::name::Name;
use crate::utils::units::node::node_dmx_channel_initial_function::NodeDmxChannelInitialFunction;
use crate::utils::units::offset::Offset;
use std::convert::TryInto;

pub mod logical_channel;

///This section defines the DMX channe
#[derive(Debug, PartialEq, Clone)]
pub struct DmxChannel {
    ///Number of the DMXBreak; Default value: 1; Special value: “Overwrite” – means that this number will be overwritten by Geometry Reference; Size: 4 bytes
    pub dmx_break: DmxBreak,
    ///Relative addresses of the current DMX channel from highest to least significant
    pub offset: Option<Offset>,
    ///Link to the channel function that will be activated by default for this DMXChannel;
    pub initial_function: NodeDmxChannelInitialFunction,
    ///Highlight value for current channel; Special value: “None”. Default value: “None”.
    pub highlight: Highlight,
    ///Name of the geometry the current channel controls.
    pub geometry: Name,
    ///List of logical channels
    pub logical_channels: Vec<LogicalChannel>,
}

impl DeparseSingle for DmxChannel {
    type PrimaryKey = ();

    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut dmx_break = DmxBreak::default();
        let mut offset = None;
        let mut initial_function: NodeDmxChannelInitialFunction = Default::default();
        let mut highlight = Highlight::default();
        let mut geometry = Default::default();
        let mut logical_channels: Vec<LogicalChannel> = Vec::new();

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"DMXBreak" => dmx_break = deparse::attr_to_str(&attr).into(),
                b"Offset" => offset = Offset::new_from_attr(attr),
                b"InitialFunction" => initial_function = attr.try_into()?,
                b"Highlight" => highlight = deparse::attr_to_str(&attr).into(),
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
                        logical_channels.push(LogicalChannel::single_from_event(reader, e)?.0);
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

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"DMXChannel"
    }

    fn single_event_name() -> String {
        "DMXChannel".to_string()
    }
}

impl DeparseVec for DmxChannel {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"DMXChannels"
    }
}

#[cfg(test)]
impl TestDeparseSingle for DmxChannel {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::fixture_type::dmx_mode::dmx_channel::DmxChannel;
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::LogicalChannel;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::dmx_break::DmxBreak;
    use crate::utils::units::dmx_value::DmxValue;
    use crate::utils::units::highlight::Highlight;
    use crate::utils::units::master::Master;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::node_dmx_channel_initial_function::NodeDmxChannelInitialFunction;
    use crate::utils::units::node::node_logical_channel_attribute::NodeLogicalChannelAttribute;
    use crate::utils::units::offset::Offset;
    use crate::utils::units::snap::Snap;

    #[test]
    fn test_normal() -> Result<(), GdtfError> {
        DmxChannel {
            dmx_break: DmxBreak::Value(1),
            offset: Some(Offset::new(vec![1])),
            initial_function: NodeDmxChannelInitialFunction::new_from_strs(vec!["Beam_Shutter1", "Shutter1", "Open"])?,
            highlight: Highlight::Value(DmxValue {
                initial_value: 8,
                n: 1,
                is_byte_shifting: false,
            }),
            geometry: Name::new("Beam")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                }
            ],
        }.test(None,
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
            initial_function: NodeDmxChannelInitialFunction::new_from_strs(vec!["Beam_Shutter1", "Shutter1", "Open"])?,
            highlight: Highlight::Value(DmxValue {
                initial_value: 8,
                n: 1,
                is_byte_shifting: false,
            }),
            geometry: Name::new("Beam")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                }
            ],
        }.test(None,
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
            initial_function: NodeDmxChannelInitialFunction::new_from_strs(vec!["Beam_Shutter1", "Shutter1", "Open"])?,
            highlight: Highlight::Value(DmxValue {
                initial_value: 8,
                n: 1,
                is_byte_shifting: false,
            }),
            geometry: Name::new("Beam")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                }
            ],
        }.test(None,
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
            initial_function: NodeDmxChannelInitialFunction::none(),
            highlight: Highlight::None,
            geometry: Name::new("")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                },
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::Yes,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: HashMap::new(),
                }
            ],
        }.test(None,
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
            initial_function: NodeDmxChannelInitialFunction::none(),
            highlight: Highlight::None,
            geometry: Name::new("")?,
            logical_channels: vec![],
        }.test(None,
               r#"
            <DMXChannel>
            </DMXChannel>
            "#,
        );
        Ok(())
    }
}